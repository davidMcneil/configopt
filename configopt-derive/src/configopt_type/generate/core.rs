use crate::configopt_type::parse::{FieldType, ParsedField, ParsedVariant, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Ident};

pub fn take(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.flatten() {
                quote_spanned! {span=>
                    #self_field.take(&mut #other_field);
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if !#other_field.is_empty() {
                            ::std::mem::swap(&mut #self_field, &mut #other_field);
                        }
                    },
                    StructOptTy::Bool
                    | StructOptTy::Option
                    | StructOptTy::OptionOption
                    | StructOptTy::OptionVec
                    | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if #other_field.is_some() {
                                #self_field = #other_field.take();
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

pub fn patch(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.flatten() {
                quote_spanned! {span=>
                    #self_field.patch(&mut #other_field);
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if #self_field.is_empty() {
                            ::std::mem::swap(&mut #self_field, &mut #other_field);
                        }
                    },
                    StructOptTy::Bool
                    | StructOptTy::Option
                    | StructOptTy::OptionOption
                    | StructOptTy::OptionVec
                    | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if #self_field.is_none() {
                                #self_field = #other_field.take();
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

pub fn take_for(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.flatten() {
                quote_spanned! {span=>
                    #self_field.take_for(&mut #other_field);
                }
            } else if field.subcommand() {
                quote_spanned! {span=>
                    // TODO: handle subcommands
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if !#self_field.is_empty() {
                            ::std::mem::swap(&mut #other_field, &mut #self_field);
                        }
                    },
                    StructOptTy::Option | StructOptTy::OptionOption | StructOptTy::OptionVec => {
                        quote_spanned! {span=>
                            if #self_field.is_some() {
                                #other_field = #self_field.take();
                            }
                        }
                    }
                    StructOptTy::Bool | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if let Some(value) = #self_field.take() {
                                #other_field = value;
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

// fn clear(fields: &[ParsedField]) -> TokenStream {
//     fields
//         .iter()
//         .map(|field| {
//             let ident = field.ident();
//             let span = field.span();
//             quote_spanned! {span=>
//                 self.#ident = None;
//             }
//         })
//         .collect()
// }

// fn is_empty(fields: &[ParsedField]) -> TokenStream {
//     let field_tokens = fields.iter().map(|field| {
//         let ident = field.ident();
//         let span = field.span();
//         quote_spanned! {span=>
//             self.#ident.is_none()
//         }
//     });
//     quote! {
//         #(#field_tokens)&&*
//     }
// }

// fn is_complete(fields: &[ParsedField]) -> TokenStream {
//     let field_tokens = fields.iter().map(|field| {
//         let ident = field.ident();
//         let span = field.span();
//         if field.flatten() {
//             quote_spanned! {span=>
//                 self.#ident.as_ref().map_or(false, |val| val.is_complete())
//             }
//         } else {
//             quote_spanned! {span=>
//                 self.#ident.is_some()
//             }
//         }
//     });
//     quote! {
//         #(#field_tokens)&&*
//     }
// }

// fn from(fields: &[ParsedField]) -> TokenStream {
//     let field_tokens = fields.iter().map(|field| {
//         let ident = field.ident();
//         let span = field.span();
//         if field.flatten() {
//             quote_spanned! {span=>
//                 #ident: Some(other.#ident.into()),
//             }
//         } else {
//             quote_spanned! {span=>
//                 #ident: Some(other.#ident),
//             }
//         }
//     });
//     quote! {
//         Self {
//             #(#field_tokens)*
//         }
//     }
// }

// fn try_from(fields: &[ParsedField]) -> TokenStream {
//     let field_tokens = fields.iter().map(|field| {
//         let ident = field.ident();
//         let span = field.span();
//         // We check upfront if the type `is_complete` so all these `unwrap`'s are ok
//         if field.flatten() {
//             quote_spanned! {span=>
//                 #ident: ::std::convert::TryInto::try_into(partial.#ident.unwrap()).unwrap(),
//             }
//         } else {
//             quote_spanned! {span=>
//                 #ident: partial.#ident.unwrap(),
//             }
//         }
//     });
//     let create = quote! {
//         Self {
//             #(#field_tokens)*
//         }
//     };
//     quote! {
//         if !partial.is_complete() {
//             return Err(partial);
//         }
//         Ok(#create)
//     }
// }

pub fn take_enum(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| match variant.field_type() {
            FieldType::Unnamed => {
                let full_ident = variant.full_ident();
                let full_configopt_ident = variant.full_configopt_ident();
                quote! {
                    (#full_ident(self_variant), #full_configopt_ident(other_variant)) => {
                        self_variant.take(other_variant);
                    }
                }
            }
            FieldType::Named | FieldType::Unit => {
                quote! {}
            }
        })
        .collect()
}
