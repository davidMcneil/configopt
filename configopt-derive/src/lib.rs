extern crate proc_macro;

mod configopt_defaults;
mod partial;

use configopt_defaults::CasingStyle;
use proc_macro_roids::IdentExt;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

#[proc_macro_derive(ConfigOpt, attributes(configopt))]
pub fn configopt_derive(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as DeriveInput);
    let ident = &ast.ident;
    let partial_ident = ident.prepend(partial::PARTIAL_TYPE_PREFIX);
    let partial_type = partial::partial_type(ast.clone());
    let partial_take = partial::from_fields(&ast.data, &partial::take_generator);
    let partial_patch = partial::from_fields(&ast.data, &partial::patch_generator);
    let partial_merge = partial::from_fields(&ast.data, &partial::merge_generator);
    let partial_clear = partial::from_fields(&ast.data, &partial::clear_generator);
    let partial_is_empty = partial::from_fields(&ast.data, &partial::is_empty_generator);
    let partial_is_complete = partial::from_fields(&ast.data, &partial::is_complete_generator);
    let partial_from = partial::from_fields(&ast.data, &partial::from_generator);
    let partial_try_from = partial::from_fields(&ast.data, &partial::try_from_generator);
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
    // #[configopt(partial_only)] can be used to disable deriving any ConfigOpt traits
    let partial_only = proc_macro_roids::contains_tag(
        &ast.attrs,
        &parse_quote!(configopt),
        &parse_quote!(partial_only),
    );
    let configopt = if !partial_only {
        let rename_type = configopt_defaults::structopt_rename_all(&ast.attrs)
            // Structopt defaults to kebab case if no `rename_all` attribute is specified
            .unwrap_or(CasingStyle::Kebab);
        let arg_default_match_arms = configopt_defaults::match_arms(&ast.data, rename_type);
        quote! {
            #lints
            impl ::configopt::ConfigOpt for #ident {}

            #lints
            impl ::configopt::ConfigOptDefaults for #partial_ident {
                fn arg_default(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
                    if let Some((arg_name, arg_path)) = arg_path.split_first() {
                        match arg_name.as_str() {
                            #arg_default_match_arms
                            _ => None,
                        }
                    } else {
                        None
                    }
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
