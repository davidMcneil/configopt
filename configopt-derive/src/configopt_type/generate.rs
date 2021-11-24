pub mod configopt_defaults;
pub mod core;
pub mod default_config_files;
pub mod handle_config_files;
pub mod toml_config;
mod try_from_error;

pub use try_from_error::generate as try_from_error;

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
            clippy::cargo,
            clippy::suspicious_else_formatting
        )]
        #[deny(clippy::correctness)]
        #[allow(dead_code, unreachable_code)]
        #[allow(unused_must_use)]
    }
}
