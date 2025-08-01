use std::{borrow::Cow, path::PathBuf, process::Command, sync::Arc};

use anyhow::bail;
use faststr::FastStr;
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap;

use super::CodegenItem;
use crate::{
    Codegen, CodegenBackend, Context, DefId, fmt::fmt_file, middle::context::DefLocation,
    rir::ItemPath,
};

#[derive(Clone)]
pub struct Workspace<B> {
    base_dir: Arc<std::path::Path>,
    cg: Codegen<B>,
}

fn run_cmd(cmd: &mut Command) -> Result<(), anyhow::Error> {
    let status = cmd.status()?;

    if !status.success() {
        bail!("run cmd {:?} failed", cmd)
    }

    Ok(())
}

struct CrateInfo {
    name: FastStr,
    main_mod_path: Option<ItemPath>,
    deps: Vec<FastStr>,
    workspace_deps: Vec<FastStr>,
    items: Vec<DefId>,
    re_pubs: Vec<DefId>,
    user_gen: Option<String>,
}

impl<B> Workspace<B>
where
    B: CodegenBackend + Send,
{
    fn cx(&self) -> &Context {
        &self.cg
    }

    pub fn new(base_dir: PathBuf, cg: Codegen<B>) -> Self {
        Workspace {
            base_dir: Arc::from(base_dir),
            cg,
        }
    }

    pub fn group_defs(&self, entry_def_ids: &[DefId]) -> Result<(), anyhow::Error> {
        let location_map = self.collect_def_ids(entry_def_ids, None);
        let entry_map = location_map.iter().into_group_map_by(|item| item.1);

        let entry_deps = entry_map
            .iter()
            .map(|(k, v)| {
                let def_ids = v.iter().map(|i| i.0).copied().collect_vec();
                let deps = self
                    .collect_def_ids(&def_ids, Some(&location_map))
                    .into_iter()
                    .collect_vec();
                (k, deps)
            })
            .collect::<FxHashMap<_, _>>();

        if !self.base_dir.exists() {
            std::fs::create_dir_all(&*self.base_dir).unwrap();
        }

        let this = self.clone();

        let members = entry_map
            .keys()
            .map(|k| {
                let name = self.cx().crate_name(k);
                format!("    \"{name}\"")
            })
            .dedup()
            .sorted()
            .join(",\n");

        let mut cargo_toml = toml::from_str::<toml::Value>(&unsafe {
            String::from_utf8_unchecked(std::fs::read(self.base_dir.join("Cargo.toml")).unwrap())
        })
        .unwrap();

        let reflect_dep = if self.cg.with_descriptor {
            r#"pilota-thrift-reflect = "*""#
        } else {
            r#""#
        };

        let fieldmask_dep = if self.cg.with_field_mask {
            r#"pilota-thrift-fieldmask = "*""#
        } else {
            r#""#
        };

        crate::codegen::toml::merge_tomls(
            &mut cargo_toml,
            toml::from_str::<toml::Value>(&format!(
                r#"[workspace]
    members = [
    {members}
    ]

    [workspace.dependencies]
    pilota = "*"
    {reflect_dep}
    {fieldmask_dep}
    anyhow = "1"
    volo = "*"
    volo-{} = "*""#,
                if B::PROTOCOL == "thrift" {
                    "thrift"
                } else if B::PROTOCOL == "protobuf" {
                    "grpc"
                } else {
                    panic!("unknown protocol")
                }
            ))
            .unwrap(),
        );

        let workspace_deps = cargo_toml
            .get("workspace")
            .unwrap()
            .get("dependencies")
            .unwrap()
            .as_table()
            .unwrap()
            .keys()
            .map(FastStr::new)
            .collect_vec();

        std::fs::write(
            self.base_dir.join("Cargo.toml"),
            toml::to_string_pretty(&cargo_toml).unwrap(),
        )?;

        entry_deps
            .par_iter()
            .try_for_each_with(this, |this, (k, deps)| {
                let name = this.cx().crate_name(k);
                let deps = deps.iter().filter(|dep| dep.1 != ***k).collect_vec();
                let (main_mod_path, re_pubs, deps) = match k {
                    DefLocation::Fixed(_, path) => (
                        Some(path.clone()),
                        deps.iter().map(|v| v.0).collect_vec(),
                        deps.iter()
                            .map(|dep| this.cx().crate_name(&dep.1))
                            .sorted()
                            .dedup()
                            .collect_vec(),
                    ),
                    DefLocation::Dynamic => (None, vec![], vec![]),
                };
                this.create_crate(
                    &this.base_dir,
                    CrateInfo {
                        main_mod_path,
                        workspace_deps: workspace_deps.clone(),
                        name,
                        re_pubs,
                        items: entry_map[*k].iter().map(|(k, _)| **k).collect_vec(),
                        deps,
                        user_gen: this.cx().plugin_gen.get(k).map(|v| v.value().clone()),
                    },
                )
            })?;

        Ok(())
    }

    fn collect_def_ids(
        &self,
        input: &[DefId],
        locations: Option<&FxHashMap<DefId, DefLocation>>,
    ) -> FxHashMap<DefId, DefLocation> {
        self.cg.db.collect_def_ids(input, locations)
    }

    fn create_crate(
        &self,
        base_dir: impl AsRef<std::path::Path>,
        info: CrateInfo,
    ) -> anyhow::Result<()> {
        if !base_dir.as_ref().join(&*info.name).exists() {
            run_cmd(
                Command::new("cargo")
                    .arg("init")
                    .arg("--lib")
                    .arg("--vcs")
                    .arg("none")
                    .current_dir(base_dir.as_ref())
                    .arg(&*info.name),
            )?;
        };

        let cargo_toml_path = base_dir.as_ref().join(&*info.name).join("Cargo.toml");

        let mut cargo_toml = toml::from_str::<toml::Value>(&unsafe {
            String::from_utf8_unchecked(std::fs::read(&cargo_toml_path)?)
        })
        .unwrap();

        let deps = info
            .deps
            .iter()
            .map(|s| Cow::from(format!(r#"{s} = {{ path = "../{s}" }}"#)))
            .chain(
                info.workspace_deps
                    .iter()
                    .map(|s| Cow::from(format!(r#"{s}.workspace = true"#))),
            )
            .join("\n");

        super::toml::merge_tomls(
            &mut cargo_toml,
            toml::from_str::<toml::Value>(&format!("[dependencies]\n{deps}")).unwrap(),
        );

        std::fs::write(
            &cargo_toml_path,
            toml::to_string_pretty(&cargo_toml).unwrap(),
        )?;

        let mut lib_rs_stream = String::default();
        lib_rs_stream.push_str("include!(\"gen.rs\");\n");
        lib_rs_stream.push_str("pub use r#gen::*;\n\n");

        if let Some(user_gen) = info.user_gen {
            if !user_gen.is_empty() {
                lib_rs_stream.push_str("include!(\"custom.rs\");\n");

                let mut custom_rs_stream = String::default();
                custom_rs_stream.push_str(&user_gen);

                let custom_rs = base_dir.as_ref().join(&*info.name).join("src/custom.rs");

                std::fs::write(&custom_rs, custom_rs_stream)?;

                fmt_file(custom_rs);
            }
        }

        let mut gen_rs_stream = String::default();
        self.cg.write_items(
            &mut gen_rs_stream,
            info.items
                .iter()
                .map(|def_id| CodegenItem::from(*def_id))
                .chain(info.re_pubs.into_iter().map(|def_id| CodegenItem {
                    def_id,
                    kind: super::CodegenKind::RePub,
                })),
            base_dir.as_ref().join(&*info.name).join("src").as_path(),
        );
        if let Some(main_mod_path) = info.main_mod_path {
            gen_rs_stream.push_str(&format!(
                "pub use {}::*;",
                main_mod_path.iter().map(|item| item.to_string()).join("::")
            ));
        }
        gen_rs_stream = format! {r#"pub mod r#gen {{
            #![allow(warnings, clippy::all)]
            {gen_rs_stream}
        }}"#};

        let lib_rs_stream = lib_rs_stream.lines().map(|s| s.trim_end()).join("\n");
        let gen_rs_stream = gen_rs_stream.lines().map(|s| s.trim_end()).join("\n");

        let lib_rs = base_dir.as_ref().join(&*info.name).join("src/lib.rs");
        let gen_rs = base_dir.as_ref().join(&*info.name).join("src/gen.rs");

        std::fs::write(&lib_rs, lib_rs_stream)?;
        std::fs::write(&gen_rs, gen_rs_stream)?;

        fmt_file(lib_rs);
        fmt_file(gen_rs);

        Ok(())
    }

    pub(crate) fn write_crates(self) -> anyhow::Result<()> {
        self.group_defs(&self.cx().codegen_items)
    }
}
