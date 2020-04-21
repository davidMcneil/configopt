use crate::configopt_type::parse::{FieldType, ParsedField, ParsedVariant, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

fn to_default(field: &ParsedField) -> TokenStream {
    if field.flatten() {
        panic!("`to_default` does not make sense for a flattened field");
    }

    if field.subcommand() {
        panic!("`to_default` does not make sense for a subcommand field");
    }

    let field_ident = field.ident();
    let self_field = quote! {self.#field_ident};
    let span = field.span();

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
                // TODO: configurable separator
                result.push(" ");
            }
            result.push(&v);
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    };
    // Based on the type of the field convert it to a String. Everything is wrapped
    // in an Option because this is always run on a `ConfigOpt` type.
    //
    // Once Rust has specialization this can be significantly simplified.
    match field.structopt_ty() {
        StructOptTy::Vec => quote_spanned! {span=>
            {
                let vec = #self_field.iter()
                    .map(|value| #to_default)
                    .flatten()
                    .collect::<Vec<_>>();
                #join_os_str_vec
            }
        },
        StructOptTy::Bool | StructOptTy::Option | StructOptTy::Other => quote_spanned! {span=>
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
    let normal_fields = fields.iter().filter(|f| !f.flatten() && !f.subcommand());
    let normal_fields = normal_fields
        .map(|field| {
            let arg_name = field.structopt_name();
            let to_default = to_default(field);
            quote! {
                #arg_name => #to_default,
            }
        })
        .collect::<TokenStream>();
    let flat_fields = fields.iter().filter(|f| f.flatten());
    let flat_fields = flat_fields
        .map(|field| {
            let field_ident = field.ident();
            let self_field = quote! {self.#field_ident};
            quote! {
                if let Some(default) = #self_field.arg_default(full_arg_path) {
                    return Some(default);
                }
            }
        })
        .collect::<TokenStream>();
    let subcommand_fields = fields.iter().filter(|f| f.subcommand());
    let subcommand_fields = subcommand_fields
        .map(|field| {
            let field_ident = field.ident();
            let self_field = quote! {self.#field_ident};
            quote! {
                "cmd3" => {
                    #self_field
                        .as_ref()
                        .and_then(|value| value.arg_default(full_arg_path))
                }
            }
        })
        .collect::<TokenStream>();
    quote! {
        match arg_name.as_str() {
            #normal_fields
            #subcommand_fields
            _ => {
                // Try every flat field to see if we can get a match
                #flat_fields
                None
            },
        }
    }
}

pub fn for_enum(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            let span = variant.span();
            let structopt_name = variant.structopt_name();
            match variant.field_type() {
                FieldType::Unit => {
                    quote_spanned! {span=>
                        #full_configopt_ident => None,
                    }
                }
                FieldType::Unnamed => {
                    quote_spanned! {span=>
                        #full_configopt_ident(value) if #structopt_name == arg_path[0] => {
                            value.arg_default(&arg_path[1..])
                        }
                    }
                }
                FieldType::Named => {
                    quote_spanned! {span=>
                        // TODO: Actually lookup the values
                        #full_configopt_ident{..} => None,
                    }
                }
            }
        })
        .collect()
}
