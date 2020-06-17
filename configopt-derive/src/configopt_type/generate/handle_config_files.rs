use crate::configopt_type::parse::{self, FieldType, ParsedField, ParsedVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn generate_for_struct(parsed: &[ParsedField]) -> TokenStream {
    let has_config_fields = parse::has_configopt_fields(parsed);
    if has_config_fields {
        quote! {
            if self.generate_config.unwrap_or_default() {
                return Some(self.toml_config())
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
    let has_config_fields = parse::has_configopt_fields(parsed);
    let patch_subcommands = parsed
        .iter()
        .filter(|f| f.subcommand())
        .map(|field| {
            let field_ident = field.ident();
            let self_field = quote! {self.#field_ident};
            quote! {
                if let Some(s) = #self_field.as_mut() {
                    s.patch_with_config_files()?;
                }
            }
        })
        .collect::<TokenStream>();
    if has_config_fields {
        quote! {
            use ::std::convert::TryFrom;
            let mut from_default_config_files = #configopt_ident::from_default_config_files()?;
            let mut from_config_files = #configopt_ident::try_from(self.config_files.as_slice())?;
            from_config_files.patch(&mut from_default_config_files);
            self.patch(&mut from_config_files);
            #patch_subcommands
            Ok(self)
        }
    } else {
        quote! {
            #patch_subcommands
            Ok(self)
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
                        if let Some(config) = variant.maybe_config_file() {
                            return Some(config);
                        }
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
