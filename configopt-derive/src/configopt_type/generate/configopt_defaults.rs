use crate::configopt_type::parse::{FieldType, ParsedField, ParsedVariant, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

fn to_os_string(field: &ParsedField) -> TokenStream {
    if field.is_structopt_flatten() {
        panic!("`to_os_string` does not make sense for a flattened field");
    }

    if field.is_subcommand() {
        panic!("`to_os_string` does not make sense for a subcommand field");
    }

    let field_ident = field.ident();
    let self_field = quote! {self.#field_ident};
    let span = field.span();

    // If this had a custom to_os_string use that otherwise use ConfigOptArgToOsString
    let to_os_string = if let Some(expr) = field.to_os_string() {
        quote! {
            Some(#expr(&value))
        }
    } else {
        quote! {
            value.arg_to_os_string(arg_path)
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
        Some(result)
    };
    // Based on the type of the field convert it to a String. Everything is wrapped
    // in an Option because this is always run on a `ConfigOpt` type.
    //
    // Once Rust has specialization this can be significantly simplified.
    match field.structopt_ty() {
        StructOptTy::Vec if field.is_positional_vec() => quote_spanned! {span=>
            {
                let vec = #self_field.iter()
                    .map(|value| #to_os_string)
                    .flatten()
                    .collect::<Vec<_>>();
                #join_os_str_vec
            }
        },
        StructOptTy::Vec => quote_spanned! {span=>
            {
                if let Some(value) = &#self_field {
                    let vec = value.iter()
                        .map(|value| #to_os_string)
                        .flatten()
                        .collect::<Vec<_>>();
                    #join_os_str_vec
                } else {
                    None
                }
            }
        },
        StructOptTy::Bool | StructOptTy::Option | StructOptTy::Other => quote_spanned! {span=>
            #self_field
                .as_ref()
                .and_then(|value| #to_os_string)
        },
        StructOptTy::OptionOption => quote_spanned! {span=>
            #self_field
                .as_ref()
                .and_then(|o| o.as_ref().and_then(|value| #to_os_string))
        },
        StructOptTy::OptionVec => quote_spanned! {span=>
            #self_field
                .as_ref()
                .and_then(|vec| {
                    let vec = vec.iter()
                        .map(|value| #to_os_string)
                        .flatten()
                        .collect::<Vec<_>>();
                    #join_os_str_vec
                })
        },
    }
}

pub fn for_struct(fields: &[ParsedField]) -> TokenStream {
    let normal_fields = fields
        .iter()
        .filter(|f| !f.is_structopt_flatten() && !f.is_subcommand());
    let normal_fields = normal_fields
        .map(|field| {
            let arg_name = field.structopt_name();
            let to_os_string = to_os_string(field);
            quote! {
                #arg_name => #to_os_string,
            }
        })
        .collect::<TokenStream>();
    let flat_fields = fields.iter().filter(|f| f.is_structopt_flatten());
    let flat_fields = flat_fields
        .map(|field| {
            let field_ident = field.ident();
            let self_field = quote! {self.#field_ident};
            quote! {
                if let Some(default) = #self_field.arg_to_os_string(full_arg_path) {
                    return Some(default);
                }
            }
        })
        .collect::<TokenStream>();
    let subcommand_fields = fields.iter().filter(|f| f.is_subcommand());
    let subcommand_fields = subcommand_fields
        .map(|field| {
            let field_ident = field.ident();
            let self_field = quote! {self.#field_ident};
            quote! {
                "cmd3" => {
                    #self_field
                        .as_ref()
                        .and_then(|value| value.arg_to_os_string(full_arg_path))
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
                            value.arg_to_os_string(&arg_path[1..])
                        }
                    }
                }
                FieldType::Named(_) => {
                    quote_spanned! {span=>
                        // TODO: Actually lookup the values
                        #full_configopt_ident{..} => None,
                    }
                }
            }
        })
        .collect()
}
