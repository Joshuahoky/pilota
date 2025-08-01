pub mod r#gen {
    #![allow(warnings, clippy::all)]

    pub mod article {

        pub mod image {

            include!("article/image/mod.rs");

            pub mod cdn {

                include!("article/image/cdn/mod.rs");
            }
        }
    }

    pub mod author {

        include!("author/mod.rs");
    }

    pub mod common {

        include!("common/mod.rs");
    }
}
