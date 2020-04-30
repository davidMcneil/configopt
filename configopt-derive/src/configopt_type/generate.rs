pub mod configopt_defaults;
pub mod core;
pub mod default_config_files;
pub mod handle_config_files;
pub mod toml_config;

use proc_macro2::TokenStream;
use quote::quote;

pub fn lints() -> TokenStream {
    quote! {
        #[allow(unused_variables)]
        #[allow(unknown_lints)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo
        )]
        #[deny(clippy::correctness)]
        #[allow(dead_code, unreachable_code)]
    }
}
