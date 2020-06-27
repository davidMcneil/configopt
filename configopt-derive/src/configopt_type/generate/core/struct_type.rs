use crate::configopt_type::parse::{ParsedField, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Ident};

struct FieldNames {
    self_field: TokenStream,
    other_field: TokenStream,
    deref_self_field: TokenStream,
    deref_other_field: TokenStream,
}

impl FieldNames {
    fn new(field_ident: &Ident, self_prefix: &str, other_prefix: &str, references: bool) -> Self {
        let self_field = format!("{}{}", self_prefix, field_ident)
            .parse::<TokenStream>()
            .unwrap();
        let other_field = format!("{}{}", other_prefix, field_ident)
            .parse::<TokenStream>()
            .unwrap();
        if references {
            Self {
                self_field: quote! {#self_field},
                other_field: quote! {#other_field},
                deref_self_field: quote! {*#self_field},
                deref_other_field: quote! {*#other_field},
            }
        } else {
            Self {
                self_field: quote! {&mut #self_field},
                other_field: quote! {&mut #other_field},
                deref_self_field: quote! {#self_field},
                deref_other_field: quote! {#other_field},
            }
        }
    }
}

pub(crate) fn patch_with_prefix(
    self_prefix: &str,
    other_prefix: &str,
    references: bool,
    fields: &[ParsedField],
) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let FieldNames {
                self_field,
                other_field,
                deref_other_field: _,
                deref_self_field,
            } = FieldNames::new(field_ident, self_prefix, other_prefix, references);
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.patch(#other_field);
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if (#self_field).is_empty() {
                            ::std::mem::swap(#self_field, #other_field);
                        }
                    },
                    StructOptTy::Bool
                    | StructOptTy::Option
                    | StructOptTy::OptionOption
                    | StructOptTy::OptionVec
                    | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if (#self_field).is_none() {
                                #deref_self_field = (#other_field).take();
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn take_with_prefix(
    self_prefix: &str,
    other_prefix: &str,
    references: bool,
    fields: &[ParsedField],
) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let FieldNames {
                self_field,
                other_field,
                deref_other_field: _,
                deref_self_field,
            } = FieldNames::new(field_ident, self_prefix, other_prefix, references);
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.take(#other_field);
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if !(#other_field).is_empty() {
                            ::std::mem::swap(#self_field, #other_field);
                        }
                    },
                    StructOptTy::Bool
                    | StructOptTy::Option
                    | StructOptTy::OptionOption
                    | StructOptTy::OptionVec
                    | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if (#other_field).is_some() {
                                #deref_self_field = (#other_field).take();
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn patch_for_with_prefix(
    self_prefix: &str,
    other_prefix: &str,
    references: bool,
    fields: &[ParsedField],
) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let FieldNames {
                self_field,
                other_field,
                deref_self_field: _,
                deref_other_field,
            } = FieldNames::new(field_ident, self_prefix, other_prefix, references);
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.patch_for(#other_field);
                }
            } else if field.subcommand() {
                quote_spanned! {span=>
                    // TODO: handle subcommands
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => {
                        quote_spanned! {span=>
                            if (#other_field).is_empty() {
                                ::std::mem::swap(#other_field, #self_field);
                            }
                        }
                    }
                    StructOptTy::Option | StructOptTy::OptionOption | StructOptTy::OptionVec => {
                        quote_spanned! {span=>
                            if (#other_field).is_none() {
                                #deref_other_field = (#self_field).take();
                            }
                        }
                    }
                    StructOptTy::Bool | StructOptTy::Other => {
                        quote_spanned! {span=>}
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn take_for_with_prefix(
    self_prefix: &str,
    other_prefix: &str,
    references: bool,
    fields: &[ParsedField],
) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let FieldNames {
                self_field,
                other_field,
                deref_other_field,
                deref_self_field: _,
            } = FieldNames::new(field_ident, self_prefix, other_prefix, references);
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.take_for(#other_field);
                }
            } else if field.subcommand() {
                quote_spanned! {span=>
                    // TODO: handle subcommands
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if !(#self_field).is_empty() {
                            ::std::mem::swap(#other_field, #self_field);
                        }
                    },
                    StructOptTy::Option | StructOptTy::OptionOption | StructOptTy::OptionVec => {
                        quote_spanned! {span=>
                            if (#self_field).is_some() {
                                #deref_other_field = (#self_field).take();
                            }
                        }
                    }
                    StructOptTy::Bool | StructOptTy::Other => {
                        quote_spanned! {span=>
                            if let Some(value) = (#self_field).take() {
                                #deref_other_field = value;
                            }
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn is_empty_with_prefix(prefix: &str, fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = format!("{}{}", prefix, field_ident)
            .parse::<TokenStream>()
            .unwrap();
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #self_field.is_empty()
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #self_field.is_none()
            }
        } else {
            match field.structopt_ty() {
                StructOptTy::Vec => quote_spanned! {span=>
                    // TODO: how to handle vectors
                    #self_field.is_empty()
                },
                _ => {
                    quote_spanned! {span=>
                        #self_field.is_none()
                    }
                }
            }
        }
    });
    quote! {
        #(#field_tokens)&&*
    }
}

pub(crate) fn is_complete_with_prefix(prefix: &str, fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = format!("{}{}", prefix, field_ident)
            .parse::<TokenStream>()
            .unwrap();
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #self_field.is_complete()
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #self_field.as_ref().map_or(false, |val| val.is_complete())
            }
        } else {
            match field.structopt_ty() {
                StructOptTy::Vec => quote_spanned! {span=>
                    // TODO: how to handle vectors
                    true
                },
                _ => {
                    quote_spanned! {span=>
                        #self_field.is_some()
                    }
                }
            }
        }
    });
    quote! {
        #(#field_tokens)&&*
    }
}

pub(crate) fn is_convertible_with_prefix(prefix: &str, fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = format!("{}{}", prefix, field_ident)
            .parse::<TokenStream>()
            .unwrap();
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #self_field.is_convertible()
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #self_field.as_ref().map_or(false, |val| val.is_convertible())
            }
        } else {
            match field.structopt_ty() {
                // We do not include `StructOptTy::Bool` here. If there is no value set for the
                // bool we default the value to `false`.
                StructOptTy::Other => quote_spanned! {span=>
                    #self_field.is_some()
                },
                _ => {
                    quote_spanned! {span=>
                        true
                    }
                }
            }
        }
    });
    quote! {
        #(#field_tokens)&&*
    }
}

pub(crate) fn from(fields: &[ParsedField], other: &Ident) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let other_field = quote! {#other.#field_ident};
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #field_ident: #other_field.into(),
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #field_ident: Some(#other_field.into()),
            }
        } else {
            match field.structopt_ty() {
                StructOptTy::Bool | StructOptTy::Other => quote_spanned! {span=>
                    #field_ident: Some(#other_field),
                },
                _ => {
                    quote_spanned! {span=>
                        #field_ident: #other_field,
                    }
                }
            }
        }
    });
    quote! {
        Self {
            #(#field_tokens)*
        }
    }
}

pub(crate) fn try_from(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = quote! {configopt.#field_ident};
        // We check upfront if the type `is_convertible` so all these `unwrap`'s are ok
        if field.structopt_flatten() {
            quote_spanned! {span=>
                #field_ident: #self_field.try_into().ok().unwrap(),
            }
        } else if field.subcommand() {
            quote_spanned! {span=>
                #field_ident: #self_field.unwrap().try_into().ok().unwrap(),
            }
        } else {
            match field.structopt_ty() {
                StructOptTy::Bool => quote_spanned! {span=>
                    #field_ident: #self_field.unwrap_or_default(),
                },
                StructOptTy::Other => quote_spanned! {span=>
                    #field_ident: #self_field.unwrap(),
                },
                _ => {
                    quote_spanned! {span=>
                        #field_ident: #self_field,
                    }
                }
            }
        }
    });
    quote! {
        Ok(Self {
            #(#field_tokens)*
        })
    }
}

pub(crate) fn patch(fields: &[ParsedField]) -> TokenStream {
    patch_with_prefix("self.", "other.", false, fields)
}

pub(crate) fn take(fields: &[ParsedField]) -> TokenStream {
    take_with_prefix("self.", "other.", false, fields)
}

pub(crate) fn patch_for(fields: &[ParsedField]) -> TokenStream {
    patch_for_with_prefix("self.", "other.", false, fields)
}

pub(crate) fn take_for(fields: &[ParsedField]) -> TokenStream {
    take_for_with_prefix("self.", "other.", false, fields)
}

pub(crate) fn is_empty(fields: &[ParsedField]) -> TokenStream {
    is_empty_with_prefix("self.", fields)
}

pub(crate) fn is_complete(fields: &[ParsedField]) -> TokenStream {
    is_complete_with_prefix("self.", fields)
}

pub(crate) fn is_convertible(fields: &[ParsedField]) -> TokenStream {
    is_convertible_with_prefix("self.", fields)
}
