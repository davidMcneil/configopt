use crate::configopt_type::parse::{FieldType, ParsedField, ParsedVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

fn has_configopt_fields(parsed: &[ParsedField]) -> bool {
    parsed.iter().any(|f| f.ident() == "generate_config")
}

pub fn generate_for_struct(parsed: &[ParsedField]) -> TokenStream {
    let has_config_fields = has_configopt_fields(parsed);
    if has_config_fields {
        quote! {
            if self.generate_config {
                use ::std::io::Write;
                use ::configopt::TomlConfigGenerator;
                let out = ::std::io::stdout();
                writeln!(&mut out.lock(), "{}", self.toml_config()).expect("Error writing Error to stdout");
                ::std::process::exit(0);
            }
            // TODO: handle recursive subcommands
        }
    } else {
        quote! {
            // TODO: handle recursive subcommands
        }
    }
}

pub fn patch_for_struct(parsed: &[ParsedField], configopt_ident: &Ident) -> TokenStream {
    let has_config_fields = has_configopt_fields(parsed);
    if has_config_fields {
        quote! {
            use ::std::convert::TryFrom;
            let mut from_config_files = #configopt_ident::try_from(self.config_files.as_slice())?;
            self.patch(&mut from_config_files);
            Ok(self)
            // TODO: handle recursive subcommands
        }
    } else {
        quote! {
            Ok(self)
            // TODO: handle recursive subcommands
        }
    }
}

pub fn generate_for_enum(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| match variant.field_type() {
            FieldType::Unnamed => {
                let full_configopt_ident = variant.full_configopt_ident();
                quote! {
                    #full_configopt_ident(variant) => {
                        variant.maybe_generate_config_file_and_exit();
                    }
                }
            }
            FieldType::Named | FieldType::Unit => {
                quote! {}
            }
        })
        .collect()
}

pub fn patch_for_enum(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| match variant.field_type() {
            FieldType::Unnamed => {
                let full_configopt_ident = variant.full_configopt_ident();
                quote! {
                    #full_configopt_ident(variant) => {
                        variant.patch_with_config_files()?;
                    }
                }
            }
            FieldType::Named | FieldType::Unit => {
                quote! {}
            }
        })
        .collect()
}
