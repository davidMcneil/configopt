extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use proc_macro_roids::{DeriveInputExt, IdentExt};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, Attribute, Data,
    DeriveInput, Field, Fields, Ident, Index, Token, Type,
};

const PARTIAL_TYPE_PREFIX: &str = "Partial";
const PARTIAL_ATTR_IDENT: &str = "partial";

#[proc_macro_derive(Partial, attributes(partial))]
pub fn partial_derive(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as DeriveInput);
    let ident = &ast.ident;
    let partial_ident = ident.prepend(PARTIAL_TYPE_PREFIX);
    let partial_type = partial_type(ast.clone());
    let partial_take = from_fields(&ast.data, &take_generator);
    let partial_patch = from_fields(&ast.data, &patch_generator);
    let partial_merge = from_fields(&ast.data, &merge_generator);
    let partial_clear = from_fields(&ast.data, &clear_generator);
    let partial_is_empty = from_fields(&ast.data, &is_empty_generator);
    let partial_is_complete = from_fields(&ast.data, &is_complete_generator);
    let partial_from = from_fields(&ast.data, &from_generator);
    let partial_try_from = from_fields(&ast.data, &try_from_generator);
    let lints = quote! {
        #[allow(unused_variables)]
        #[allow(unknown_lints)]
        #[allow(
            clippy::style,
            clippy::complexity,
            clippy::pedantic,
            clippy::restriction,
            clippy::perf,
            clippy::deprecated,
            clippy::nursery,
            clippy::cargo
        )]
        #[deny(clippy::correctness)]
        #[allow(dead_code, unreachable_code)]
    };
    let expanded = quote! {
        #lints
        #partial_type

        #lints
        impl #partial_ident {
            /// Take each field from `other` and set it in `self`
            fn take(&mut self, other: &mut #partial_ident) {
                #partial_take
            }

            /// For each field in `self` if it is `None`, take the value from `other` and set it in `self`
            fn patch(&mut self, other: &mut #partial_ident) {
                #partial_patch
            }

            /// Take each field from `self` and set it in `other`
            fn merge(&mut self, other: &mut #ident) {
                #partial_merge
            }

            /// Clear all fields from `self`
            fn clear(&mut self) {
                #partial_clear
            }

            /// Check if all fields of `self` are `None`
            fn is_empty(&self) -> bool {
                #partial_is_empty
            }

            /// Check if all fields of `self` are `Some` applied recursively
            fn is_complete(&self) -> bool {
                #partial_is_complete
            }
        }

        #lints
        impl ::std::convert::From<#ident> for #partial_ident {
            fn from(other: #ident) -> Self {
                #partial_from
            }
        }

        #lints
        impl ::std::convert::TryFrom<#partial_ident> for #ident {
            type Error = #partial_ident;
            fn try_from(partial: #partial_ident) -> Result<Self, Self::Error> {
                #partial_try_from
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

/// Check if a field is annotated with #[partial]
fn has_partial_attr(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .any(|a| a.path.is_ident(PARTIAL_ATTR_IDENT))
}

fn retain_attrs(attrs: &mut Vec<Attribute>, to_retain: &[Ident]) {
    attrs.retain(|a| to_retain.iter().any(|i| a.path.is_ident(i)))
}

fn partial_type(full_type: DeriveInput) -> DeriveInput {
    let mut partial_type = full_type;

    // Change the ident to a partial ident
    partial_type.ident = partial_type.ident.prepend(PARTIAL_TYPE_PREFIX);

    // Get a list of attributes to retain on the partial type
    let retained_attrs = partial_type
        .tag_parameters(&parse_quote!(partial), &parse_quote!(attrs))
        .into_iter()
        .map(|meta| {
            proc_macro_roids::nested_meta_to_path(&meta)
                .expect("#[partial(attrs(..))] expected a path not a Rust literal")
                .get_ident()
                .cloned()
                .expect("#[partial(attrs(..))] expected an ident")
        })
        .collect::<Vec<_>>();

    // Get the derives for the partial type
    let derives = partial_type
        .tag_parameters(&parse_quote!(partial), &parse_quote!(derive))
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
        Data::Enum(_) => panic!("`Partial` cannot be derived for enums"),
        Data::Union(_) => panic!("`Partial` cannot be derived for unions"),
    }
    partial_type
}

fn make_field_partial(field: &mut Field, retained_attrs: &[Ident]) {
    let has_partial_attr = has_partial_attr(field);
    let ty = &mut field.ty;

    // If the field had a partial attribute, modify the type with the partial type prefix
    if has_partial_attr {
        match ty {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last_mut() {
                    segment.ident = segment.ident.prepend(PARTIAL_TYPE_PREFIX);
                } else {
                    panic!("`#[partial]` could not find a last segment in the type path to make partial");
                }
            }
            _ => {
                panic!("`#[partial]` only supports types specified by a path");
            }
        }
    }

    // Only retain attributes we have explicitly opted to preserve
    retain_attrs(&mut field.attrs, &retained_attrs);

    // Wrap the type in an Option. We intentionally do not use the fully qualified path to
    // `Option` because some custom derives do not handle it correctly (ie `structopt`).
    let ty: Type = parse_quote!(Option<#ty>);
    field.ty = ty;
}

enum StructFieldType {
    Named,
    Unnamed,
}

fn from_fields(
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

fn take_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(i, field);
            let span = field.span();
            if has_partial_attr(field) {
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

fn patch_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(i, field);
            let span = field.span();
            if has_partial_attr(field) {
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

fn merge_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(i, field);
            let span = field.span();
            if has_partial_attr(field) {
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

fn clear_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    fields
        .into_iter()
        .enumerate()
        .map(|(i, field)| {
            let ident = field_ident(i, field);
            let span = field.span();
            quote_spanned! {span=>
                self.#ident = None;
            }
        })
        .collect()
}

fn is_empty_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    let field_tokens = fields.into_iter().enumerate().map(|(i, field)| {
        let ident = field_ident(i, field);
        let span = field.span();
        quote_spanned! {span=>
            self.#ident.is_none()
        }
    });
    quote! {
        #(#field_tokens)&&*
    }
}

fn is_complete_generator(
    fields: &Punctuated<Field, Token![,]>,
    _field_type: StructFieldType,
) -> TokenStream {
    let field_tokens = fields.into_iter().enumerate().map(|(i, field)| {
        let ident = field_ident(i, field);
        let span = field.span();
        if has_partial_attr(field) {
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

fn from_generator(
    fields: &Punctuated<Field, Token![,]>,
    field_type: StructFieldType,
) -> TokenStream {
    let field_tokens = fields.into_iter().enumerate().map(|(i, field)| {
        let ident = field_ident(i, field);
        let span = field.span();
        let conversion = if has_partial_attr(field) {
            quote_spanned! {span=>
                Some(other.#ident.into()),
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

fn try_from_generator(
    fields: &Punctuated<Field, Token![,]>,
    field_type: StructFieldType,
) -> TokenStream {
    let field_tokens = fields.into_iter().enumerate().map(|(i, field)| {
        let ident = field_ident(i, field);
        let span = field.span();
        // We check upfront if the type `is_complete` so all these `unwrap`'s are ok
        let conversion = if has_partial_attr(field) {
            quote_spanned! {span=>
                ::std::convert::TryInto::try_into(partial.#ident.unwrap()).unwrap(),
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
