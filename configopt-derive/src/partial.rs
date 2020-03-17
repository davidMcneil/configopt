use proc_macro2::{Span, TokenStream};
use proc_macro_roids::{DeriveInputExt, IdentExt};
use quote::{quote, quote_spanned};
use syn::{
    parse_quote, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DeriveInput, Field,
    Fields, Ident, Index, Token, Type,
};

pub const PARTIAL_TYPE_PREFIX: &str = "Partial";

/// Check if a field is annotated with #[configopt(nested)]
fn has_configopt_nested_attr(field: &Field) -> bool {
    proc_macro_roids::contains_tag(
        &field.attrs,
        &parse_quote!(configopt),
        &parse_quote!(nested),
    )
}

fn retain_attrs(attrs: &mut Vec<Attribute>, to_retain: &[Ident]) {
    attrs.retain(|a| to_retain.iter().any(|i| a.path.is_ident(i)))
}

pub fn partial_type(full_type: DeriveInput) -> DeriveInput {
    let mut partial_type = full_type;

    // Change the ident to a partial ident
    partial_type.ident = partial_type.ident.prepend(PARTIAL_TYPE_PREFIX);

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
    let derives = partial_type
        .tag_parameters(&parse_quote!(configopt), &parse_quote!(derive))
        .into_iter()
        .collect::<Punctuated<_, Token![,]>>();

    // Only include the explicitly retained attributes
    retain_attrs(&mut partial_type.attrs, &retained_attrs);

    // Add the derives
    partial_type.append_derives(derives);

    // Make all fields partial
    match &mut partial_type.data {
        Data::Struct(data) => match &mut data.fields {
            Fields::Named(fields) => {
                for field in &mut fields.named {
                    make_field_partial(field, &retained_attrs);
                }
            }
            Fields::Unnamed(fields) => {
                for field in &mut fields.unnamed {
                    make_field_partial(field, &retained_attrs);
                }
            }
            Fields::Unit => {}
        },
        Data::Enum(_) => panic!("`ConfigOpt` cannot be derived for enums"),
        Data::Union(_) => panic!("`ConfigOpt` cannot be derived for unions"),
    }
    partial_type
}

fn make_field_partial(field: &mut Field, retained_attrs: &[Ident]) {
    let has_configopt_nested_attr = has_configopt_nested_attr(field);
    let ty = &mut field.ty;

    // If the field had a configopt type attribute, modify the type with the partial type prefix
    if has_configopt_nested_attr {
        match ty {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last_mut() {
                    segment.ident = segment.ident.prepend(PARTIAL_TYPE_PREFIX);
                } else {
                    panic!("`#[configopt]` could not find a last segment in the type path to make partial");
                }
            }
            _ => {
                panic!("`#[configopt]` only supports types specified by a path");
            }
        }
    } else {
        // Wrap the type in an Option. We intentionally do not use the fully qualified path to
        // `Option` because some custom derives do not handle it correctly (ie `structopt`).
        let ty: Type = parse_quote!(Option<#ty>);
        field.ty = ty;
    }

    // Only retain attributes we have explicitly opted to preserve
    retain_attrs(&mut field.attrs, &retained_attrs);
}

pub enum StructFieldType {
    Named,
    Unnamed,
}

pub fn from_fields(
    data: &Data,
    generator: impl Fn(&Punctuated<Field, Token![,]>, StructFieldType) -> TokenStream,
) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => generator(&fields.named, StructFieldType::Named),
            Fields::Unnamed(fields) => generator(&fields.unnamed, StructFieldType::Unnamed),
            Fields::Unit => TokenStream::new(),
        },
        Data::Enum(_) => unreachable!(),
        Data::Union(_) => unreachable!(),
    }
}

fn field_ident(index: usize, field: &Field) -> TokenStream {
    if let Some(ident) = &field.ident {
        quote! {#ident}
    } else {
        // If it is a tuple struct use the index as the ident
        let index = Index {
            index: index as u32,
            span: Span::call_site(),
        };
        quote! {#index}
    }
}

pub fn take_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(i, field);
            let span = field.span();
            if has_configopt_nested_attr(field) {
                quote_spanned! {span=>
                    self.#ident.take(&mut other.#ident);
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

pub fn patch_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(i, field);
            let span = field.span();
            if has_configopt_nested_attr(field) {
                quote_spanned! {span=>
                    self.#ident.patch(&mut other.#ident);
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

pub fn merge_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(i, field);
            let span = field.span();
            if has_configopt_nested_attr(field) {
                quote_spanned! {span=>
                    self.#ident.merge(&mut other.#ident);
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

pub fn clear_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(i, field);
            let span = field.span();
            if has_configopt_nested_attr(field) {
                quote_spanned! {span=>
                    self.#ident.clear();
                }
            } else {
                quote_spanned! {span=>
                    self.#ident = None;
                }
            }
        })
        .collect()
}

pub fn is_empty_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    let field_tokens = fields.into_iter().enumerate().map(|(i, field)| {
        let ident = field_ident(i, field);
        let span = field.span();
        if has_configopt_nested_attr(field) {
            quote_spanned! {span=>
                self.#ident.is_empty()
            }
        } else {
            quote_spanned! {span=>
                self.#ident.is_none()
            }
        }
    });
    quote! {
        #(#field_tokens)&&*
    }
}

pub fn is_complete_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    let field_tokens = fields.into_iter().enumerate().map(|(i, field)| {
        let ident = field_ident(i, field);
        let span = field.span();
        if has_configopt_nested_attr(field) {
            quote_spanned! {span=>
                self.#ident.is_complete()
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

pub fn from_generator(
    fields: &Punctuated<Field, Token![,]>,
    field_type: StructFieldType,
) -> TokenStream {
    let field_tokens = fields.into_iter().enumerate().map(|(i, field)| {
        let ident = field_ident(i, field);
        let span = field.span();
        let conversion = if has_configopt_nested_attr(field) {
            quote_spanned! {span=>
                other.#ident.into(),
            }
        } else {
            quote_spanned! {span=>
                Some(other.#ident),
            }
        };
        match field_type {
            StructFieldType::Named => {
                quote! {
                    #ident: #conversion
                }
            }
            StructFieldType::Unnamed => conversion,
        }
    });
    match field_type {
        StructFieldType::Named => {
            quote! {
                Self {
                    #(#field_tokens)*
                }
            }
        }
        StructFieldType::Unnamed => {
            quote! {
                Self(
                    #(#field_tokens)*
                )
            }
        }
    }
}

pub fn try_from_generator(
    fields: &Punctuated<Field, Token![,]>,
    field_type: StructFieldType,
) -> TokenStream {
    let field_tokens = fields.into_iter().enumerate().map(|(i, field)| {
        let ident = field_ident(i, field);
        let span = field.span();
        // We check upfront if the type `is_complete` so all these `unwrap`'s are ok
        let conversion = if has_configopt_nested_attr(field) {
            quote_spanned! {span=>
                ::std::convert::TryInto::try_into(partial.#ident).unwrap(),
            }
        } else {
            quote_spanned! {span=>
                partial.#ident.unwrap(),
            }
        };
        match field_type {
            StructFieldType::Named => {
                quote! {
                    #ident: #conversion
                }
            }
            StructFieldType::Unnamed => conversion,
        }
    });
    let create = match field_type {
        StructFieldType::Named => {
            quote! {
                Self {
                    #(#field_tokens)*
                }
            }
        }
        StructFieldType::Unnamed => {
            quote! {
                Self(
                    #(#field_tokens)*
                )
            }
        }
    };
    quote! {
        if !partial.is_complete() {
            return Err(partial);
        }
        Ok(#create)
    }
}
