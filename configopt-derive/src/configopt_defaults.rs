use super::util::{ParsedField, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

fn to_default(field: &ParsedField) -> TokenStream {
    let ident = field.ident();
    let span = field.span();
    // If this had a custom to_default use that otherwise use ConfigOptDefaults
    let to_default = if let Some(expr) = field.to_default() {
        quote! {
            Some(#expr(&v))
        }
    } else {
        quote! {
            (&v).arg_default(arg_path)
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
        result
    };
    // Based on the type of the field convert it to a String. Everything is wrapped
    // in an Option because this is always run on a `Partial` type.
    //
    // Once Rust has specialization this can be significantly simplified.
    match StructOptTy::from_syn_ty(field.ty()) {
        StructOptTy::Bool | StructOptTy::Other => quote_spanned! {span=>
            self.#ident
                .as_ref()
                .and_then(|v| #to_default)
        },
        StructOptTy::Vec => quote_spanned! {span=>
            self.#ident
                .as_ref()
                .map(|vec| {
                    let vec = vec.iter()
                        .map(|v| #to_default)
                        .flatten()
                        .collect::<Vec<_>>();
                    #join_os_str_vec
                })
        },
        StructOptTy::Option => quote_spanned! {span=>
            self.#ident
                .as_ref()
                .and_then(|o| o.as_ref().and_then(|v| #to_default))
        },
        StructOptTy::OptionOption => quote_spanned! {span=>
            self.#ident
                .as_ref()
                .and_then(|oo|
                    oo.as_ref().and_then(|o| o.as_ref().and_then(|v| #to_default)))
        },
        StructOptTy::OptionVec => quote_spanned! {span=>
            self.#ident
                .as_ref()
                .and_then(|o| o.as_ref().map(|vec| {
                    let vec = vec.iter()
                        .map(|v| #to_default)
                        .flatten()
                        .collect::<Vec<_>>();
                    #join_os_str_vec
                }))
        },
    }
}

pub fn match_arms(fields: &[ParsedField]) -> TokenStream {
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
