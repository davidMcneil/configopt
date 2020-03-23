use super::util::{self, ParsedField};
use proc_macro2::TokenStream;
use proc_macro_roids::{DeriveInputExt, IdentExt};
use quote::{quote, quote_spanned};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DeriveInput, Field,
    Fields, Ident, Token, Type,
};

pub fn partial_ident(ident: &Ident) -> Ident {
    ident.prepend("Partial")
}

pub fn partial_type(full_type: DeriveInput) -> DeriveInput {
    let mut partial_type = full_type;

    // Change the ident to a partial ident
    partial_type.ident = partial_ident(&partial_type.ident);

    // Get a list of attributes to retain on the partial type
    let retained_attrs = partial_type
        .tag_parameters(&parse_quote!(configopt), &parse_quote!(attrs))
        .into_iter()
        .map(|meta| {
            proc_macro_roids::nested_meta_to_path(&meta)
                .expect("#[configopt(attrs(..))] expected a path not a Rust literal")
                .get_ident()
                .cloned()
                .expect("#[configopt(attrs(..))] expected an ident")
        })
        .collect::<Vec<_>>();

    // Get the derives for the partial type
    let mut derives = partial_type
        .tag_parameters(&parse_quote!(configopt), &parse_quote!(derive))
        .into_iter()
        .collect::<Punctuated<_, Token![,]>>();

    // Only retain attributes we have explicitly opted to preserve
    retain_attrs(&mut partial_type.attrs, &retained_attrs);

    // Add the derives
    derives.push(parse_quote! {Default});
    partial_type.append_derives(derives);

    // Make all fields partial
    match &mut partial_type.data {
        Data::Struct(data) => match &mut data.fields {
            Fields::Named(fields) => {
                for field in &mut fields.named {
                    make_field_partial(field, &retained_attrs);
                }
            }
            Fields::Unnamed(_) => panic!("`ConfigOpt` cannot be derived for unnamed struct"),
            Fields::Unit => panic!("`ConfigOpt` cannot be derived for unit structs"),
        },
        Data::Enum(_) => panic!("`ConfigOpt` cannot be derived for enums"),
        Data::Union(_) => panic!("`ConfigOpt` cannot be derived for unions"),
    }
    partial_type
}

fn retain_attrs(attrs: &mut Vec<Attribute>, to_retain: &[Ident]) {
    attrs.retain(|a| to_retain.iter().any(|i| a.path.is_ident(i)))
}

fn make_field_partial(field: &mut Field, retained_attrs: &[Ident]) {
    let has_configopt_nested_attr = util::has_configopt_nested_attr(field);
    let ty = &mut field.ty;

    // If the field had a configopt type attribute, modify the type with the partial type prefix
    if has_configopt_nested_attr {
        let inner_ty = util::inner_ty(ty);
        *inner_ty = partial_ident(inner_ty);
    }

    // Only retain attributes we have explicitly opted to preserve
    retain_attrs(&mut field.attrs, &retained_attrs);

    // Wrap the type in an Option. We intentionally do not use the fully qualified path to
    // `Option` because some custom derives do not handle it correctly (ie `structopt`).
    let ty: Type = parse_quote!(Option<#ty>);
    field.ty = ty;
}

pub fn take(fields: &[ParsedField]) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let ident = field.ident();
            let span = field.span();
            if field.nested() {
                quote_spanned! {span=>
                    if let Some(mut other_val) = other.#ident.take() {
                        if let Some(self_val) = &mut self.#ident {
                            self_val.take(&mut other_val);
                        } else {
                            self.#ident = Some(other_val);
                        }
                    }
                }
            } else {
                quote_spanned! {span=>
                    if other.#ident.is_some() {
                        self.#ident = other.#ident.take();
                    }
                }
            }
        })
        .collect()
}

pub fn patch(fields: &[ParsedField]) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let ident = field.ident();
            let span = field.span();
            if field.nested() {
                quote_spanned! {span=>
                    if let Some(self_val) = &mut self.#ident {
                        if let Some(other_val) = &mut other.#ident {
                            self_val.patch(other_val);
                        }
                    } else {
                        self.#ident = other.#ident.take();
                    }
                }
            } else {
                quote_spanned! {span=>
                    if self.#ident.is_none() {
                        self.#ident = other.#ident.take();
                    }
                }
            }
        })
        .collect()
}

pub fn merge(fields: &[ParsedField]) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let ident = field.ident();
            let span = field.span();
            if field.nested() {
                quote_spanned! {span=>
                    if let Some(mut val) = self.#ident.take() {
                        val.merge(&mut other.#ident)
                    }
                }
            } else {
                quote_spanned! {span=>
                    if let Some(val) = self.#ident.take() {
                        other.#ident = val;
                    }
                }
            }
        })
        .collect()
}

pub fn clear(fields: &[ParsedField]) -> TokenStream {
    fields
        .iter()
        .map(|field| {
            let ident = field.ident();
            let span = field.span();
            quote_spanned! {span=>
                self.#ident = None;
            }
        })
        .collect()
}

pub fn is_empty(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let ident = field.ident();
        let span = field.span();
        quote_spanned! {span=>
            self.#ident.is_none()
        }
    });
    quote! {
        #(#field_tokens)&&*
    }
}

pub fn is_complete(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let ident = field.ident();
        let span = field.span();
        if field.nested() {
            quote_spanned! {span=>
                self.#ident.as_ref().map_or(false, |val| val.is_complete())
            }
        } else {
            quote_spanned! {span=>
                self.#ident.is_some()
            }
        }
    });
    quote! {
        #(#field_tokens)&&*
    }
}

pub fn from(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let ident = field.ident();
        let span = field.span();
        if field.nested() {
            quote_spanned! {span=>
                #ident: Some(other.#ident.into()),
            }
        } else {
            quote_spanned! {span=>
                #ident: Some(other.#ident),
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
        let ident = field.ident();
        let span = field.span();
        // We check upfront if the type `is_complete` so all these `unwrap`'s are ok
        if field.nested() {
            quote_spanned! {span=>
                #ident: ::std::convert::TryInto::try_into(partial.#ident.unwrap()).unwrap(),
            }
        } else {
            quote_spanned! {span=>
                #ident: partial.#ident.unwrap(),
            }
        }
    });
    let create = quote! {
        Self {
            #(#field_tokens)*
        }
    };
    quote! {
        if !partial.is_complete() {
            return Err(partial);
        }
        Ok(#create)
    }
}
