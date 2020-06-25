use crate::configopt_type::parse::{ParsedField, StructOptTy};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Ident};

pub fn patch(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.structopt_flatten() {
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

pub fn take(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.structopt_flatten() {
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

pub fn patch_for(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.structopt_flatten() {
                quote_spanned! {span=>
                    #self_field.patch_for(&mut #other_field);
                }
            } else if field.subcommand() {
                quote_spanned! {span=>
                    // TODO: handle subcommands
                }
            } else {
                match field.structopt_ty() {
                    StructOptTy::Vec => quote_spanned! {span=>
                        if #other_field.is_empty() {
                            ::std::mem::swap(&mut #other_field, &mut #self_field);
                        }
                    },
                    StructOptTy::Option | StructOptTy::OptionOption | StructOptTy::OptionVec => {
                        quote_spanned! {span=>
                            if #other_field.is_none() {
                                #other_field = #self_field.take();
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

pub fn take_for(fields: &[ParsedField], other: &Ident) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let field_ident = field.ident();
            let span = field.span();
            let self_field = quote! {self.#field_ident};
            let other_field = quote! {#other.#field_ident};
            if field.structopt_flatten() {
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

pub fn is_empty(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = quote! {self.#field_ident};
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

pub fn is_complete(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = quote! {self.#field_ident};
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

pub fn is_convertible(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let field_ident = field.ident();
        let span = field.span();
        let self_field = quote! {self.#field_ident};
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

pub fn from(fields: &[ParsedField], other: &Ident) -> TokenStream {
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

pub fn try_from(fields: &[ParsedField]) -> TokenStream {
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
