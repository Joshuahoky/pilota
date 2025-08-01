use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let idl_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("idl");

    // For zero_value
    let zero_value_idl = idl_dir.join("zero_value.proto");
    pilota_build::Builder::pb()
        .ignore_unused(false)
        .include_dirs(vec![zero_value_idl.parent().unwrap().to_path_buf()])
        .keep_unknown_fields([zero_value_idl.clone()])
        .compile_with_config(
            vec![pilota_build::IdlService::from_path(zero_value_idl)],
            pilota_build::Output::File(out_dir.join("zero_value.rs")),
        );

    // For fieldmask
    let fieldmask_idl = idl_dir.join("fieldmask.thrift");
    pilota_build::Builder::thrift()
        .ignore_unused(false)
        .include_dirs(vec![fieldmask_idl.parent().unwrap().to_path_buf()])
        .with_descriptor(true)
        .with_field_mask(true)
        .compile_with_config(
            vec![pilota_build::IdlService::from_path(fieldmask_idl)],
            pilota_build::Output::File(out_dir.join("fieldmask.rs")),
        );
}
