use crate::configopt_type::parse::{FieldType, ParsedField, ParsedVariant, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

fn to_default(field: &ParsedField) -> TokenStream {
    let field_ident = field.ident();
    let self_field = quote! {self.#field_ident};
    let span = field.span();

    if field.subcommand() {
        return quote_spanned! {span=>
            None
        };
    }

    if field.flatten() {
        return quote_spanned! {span=>
            #self_field.arg_default(arg_path)
        };
    }

    // If this had a custom to_default use that otherwise use ConfigOptDefaults
    let to_default = if let Some(expr) = field.to_default() {
        quote! {
            Some(#expr(&value))
        }
    } else {
        quote! {
            (&value).arg_default(arg_path)
        }
    };
    // Code to join a Vec<OsString> into a OsString
    let join_os_str_vec = quote! {
        let mut result = ::std::ffi::OsString::new();
        for (i, v) in vec.iter().enumerate() {
            if i != 0 {
                result.push(",");
            }
            result.push(&v);
        }
        Some(result)
    };
    // Based on the type of the field convert it to a String. Everything is wrapped
    // in an Option because this is always run on a `ConfigOpt` type.
    //
    // Once Rust has specialization this can be significantly simplified.
    match field.structopt_ty() {
        StructOptTy::Bool => quote_spanned! {span=>
            {
                let value = #self_field;
                #to_default
            }
        },
        StructOptTy::Vec => quote_spanned! {span=>
            {
                let vec = #self_field.iter()
                    .map(|value| #to_default)
                    .flatten()
                    .collect::<Vec<_>>();
                #join_os_str_vec
            }
        },
        StructOptTy::Option | StructOptTy::Other => quote_spanned! {span=>
            #self_field
                .as_ref()
                .and_then(|value| #to_default)
        },
        StructOptTy::OptionOption => quote_spanned! {span=>
            #self_field
                .as_ref()
                .and_then(|o| o.as_ref().and_then(|value| #to_default))
        },
        StructOptTy::OptionVec => quote_spanned! {span=>
            #self_field
                .as_ref()
                .and_then(|vec| {
                    let vec = vec.iter()
                        .map(|value| #to_default)
                        .flatten()
                        .collect::<Vec<_>>();
                    #join_os_str_vec
                })
        },
    }
}

pub fn for_struct(fields: &[ParsedField]) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let arg_name = field.structopt_name();
            let to_default = to_default(field);
            quote! {
                #arg_name => #to_default,
            }
        })
        .collect()
}

pub fn for_enum(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            let span = variant.span();
            // TODO: Handle other variants
            if let FieldType::Unnamed = variant.field_type() {
                quote_spanned! {span=>
                    #full_configopt_ident(value) => {
                        value.arg_default(&arg_path[1..])
                    }
                }
            } else {
                quote! {}
            }
        })
        .collect()
}
