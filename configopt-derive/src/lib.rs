extern crate proc_macro;

mod configopt_defaults;
mod partial;
mod toml_config_generator;
mod util;

use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};
use util::{CasingStyle, ParsedField};

#[proc_macro_derive(ConfigOpt, attributes(configopt))]
pub fn configopt_derive(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as DeriveInput);
    let ident = &ast.ident;
    let structopt_rename = util::structopt_rename_all(&ast.attrs)
        // Structopt defaults to kebab case if no `rename_all` attribute is specified
        .unwrap_or(CasingStyle::Kebab);
    let serde_rename = CasingStyle::Verbatim; // TODO
    let parsed_fields = ParsedField::parse_fields(&ast.data, structopt_rename, serde_rename);

    let lints = quote! {
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
    };

    let partial_ident = partial::partial_ident(ident);
    let partial_type = partial::partial_type(ast.clone());
    let partial_take = partial::take(&parsed_fields);
    let partial_patch = partial::patch(&parsed_fields);
    let partial_merge = partial::merge(&parsed_fields);
    let partial_clear = partial::clear(&parsed_fields);
    let partial_is_empty = partial::is_empty(&parsed_fields);
    let partial_is_complete = partial::is_complete(&parsed_fields);
    let partial_from = partial::from(&parsed_fields);
    let partial_try_from = partial::try_from(&parsed_fields);
    let partial = quote! {
        #lints
        #partial_type

        #lints
        impl #partial_ident {
            /// Take each field from `other` and set it in `self`
            pub fn take(&mut self, other: &mut #partial_ident) {
                #partial_take
            }

            /// For each field in `self` if it is `None`, take the value from `other` and set it in `self`
            pub fn patch(&mut self, other: &mut #partial_ident) {
                #partial_patch
            }

            /// Take each field from `self` and set it in `other`
            pub fn merge(&mut self, other: &mut #ident) {
                #partial_merge
            }

            /// Clear all fields from `self`
            pub fn clear(&mut self) {
                #partial_clear
            }

            /// Check if all fields of `self` are `None`
            pub fn is_empty(&self) -> bool {
                #partial_is_empty
            }

            /// Check if all fields of `self` are `Some` applied recursively
            pub fn is_complete(&self) -> bool {
                #partial_is_complete
            }
        }

        #lints
        impl ::std::convert::From<#ident> for #partial_ident {
            fn from(other: #ident) -> Self {
                #partial_from
            }
        }

        #lints
        impl ::std::convert::TryFrom<#partial_ident> for #ident {
            type Error = #partial_ident;
            fn try_from(partial: #partial_ident) -> Result<Self, Self::Error> {
                #partial_try_from
            }
        }
    };

    // #[configopt(partial_only)] can be used to disable deriving any other traits
    let partial_only = proc_macro_roids::contains_tag(
        &ast.attrs,
        &parse_quote!(configopt),
        &parse_quote!(partial_only),
    );
    let configopt = if !partial_only {
        let configopt_defaults_match_arms = configopt_defaults::match_arms(&parsed_fields);
        let toml_config_generator_with_prefix = toml_config_generator::config(&parsed_fields);
        quote! {
            #lints
            impl ::configopt::ConfigOpt for #ident {}

            #lints
            impl ::configopt::ConfigOptDefaults for #partial_ident {
                fn arg_default(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
                    if let Some((arg_name, arg_path)) = arg_path.split_first() {
                        match arg_name.as_str() {
                            #configopt_defaults_match_arms
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
            }

            #lints
            impl ::configopt::TomlConfigGenerator for #partial_ident {
                fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String {
                    let app = #ident::clap();
                    #toml_config_generator_with_prefix
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #partial
        #configopt
    };
    proc_macro::TokenStream::from(expanded)
}
