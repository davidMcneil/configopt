extern crate proc_macro;

#[cfg(feature = "configopt")]
use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use proc_macro_roids::{DeriveInputExt, IdentExt};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, Data, DeriveInput,
    Field, Fields, Ident, Index, Meta, NestedMeta, Token, Type, Lit
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
    let partial_clear = from_fields(&ast.data, &clear_generator);
    let partial_is_empty = from_fields(&ast.data, &is_empty_generator);
    let partial_is_complete = from_fields(&ast.data, &is_complete_generator);
    let partial_from = from_fields(&ast.data, &from_generator);
    let partial_try_from = from_fields(&ast.data, &try_from_generator);
    #[cfg(feature = "configopt")]
    let partial_configopt = from_fields(&ast.data, &configopt_generator);
    let expanded = quote! {
        #partial_type

        impl #partial_ident {
            fn take(&mut self, other: &mut #partial_ident) {
                #partial_take
            }

            fn patch(&mut self, other: &mut #ident) {
                #partial_patch
            }

            fn clear(&mut self) {
                #partial_clear
            }

            fn is_empty(&self) -> bool {
                #partial_is_empty
            }

            fn is_complete(&self) -> bool {
                #partial_is_complete
            }
        }

        impl ::std::convert::From<#ident> for #partial_ident {
            fn from(other: #ident) -> Self {
                #partial_from
            }
        }

        impl ::std::convert::TryFrom<#partial_ident> for #ident {
            type Error = #partial_ident;
            fn try_from(partial: #partial_ident) -> Result<Self, Self::Error> {
                #partial_try_from
            }
        }
    };
    #[cfg(feature = "configopt")]
    let expanded = quote! {
        #expanded

        impl configopt::ConfigOptDefaults for #partial_ident {
            fn arg_default(&self, arg_path: &[String]) -> Option<String> {
                match arg_path
                    .iter()
                    .map(String::as_ref)
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    #partial_configopt
                    _ => None,
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn has_partial_attr(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .any(|a| a.path.is_ident(PARTIAL_ATTR_IDENT))
}

fn partial_type(full_type: DeriveInput) -> DeriveInput {
    let mut partial_type = full_type;

    // Change the ident to a partial ident
    partial_type.ident = partial_type.ident.prepend(PARTIAL_TYPE_PREFIX);

    // Get a list of attributes to preserve on the partial type
    let preserved_attrs = partial_type
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

    // Add the derive for the partial type
    let derives = partial_type
        .tag_parameters(&parse_quote!(partial), &parse_quote!(derive))
        .into_iter()
        .collect::<Punctuated<_, Token![,]>>();

    // Only include the preserved attributes of the original type
    partial_type.attrs.retain(|a| preserved_attrs.iter().any(|i| a.path.is_ident(i)));

    // Add the derives
    partial_type.append_derives(derives);

    // Make all fields partial
    match &mut partial_type.data {
        Data::Struct(data) => match &mut data.fields {
            Fields::Named(fields) => {
                for field in &mut fields.named {
                    make_field_partial(field, &preserved_attrs);
                }
            }
            Fields::Unnamed(fields) => {
                for field in &mut fields.unnamed {
                    make_field_partial(field, &preserved_attrs);
                }
            }
            Fields::Unit => {}
        },
        Data::Enum(_) => todo!(),
        Data::Union(_) => panic!("`Partial` cannot be derived for unions"),
    }
    partial_type
}

fn make_field_partial(field: &mut Field, preserved_attrs: &[Ident]) {
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
    field
        .attrs
        .retain(|a| preserved_attrs.iter().any(|i| a.path.is_ident(i)));
    // Wrap the type in an Option
    let ty: Type = parse_quote! {Option<#ty>};
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
        Data::Enum(_) => todo!(),
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
                    if let Some(self_val) = &mut self.#ident {
                        if let Some(other_val) = &mut other.#ident {
                            self_val.take(other_val);
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
                    if let Some(mut val) = self.#ident.take() {
                        val.patch(&mut other.#ident)
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

#[cfg(feature = "configopt")]
fn configopt_generator(
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
                // TODO: Handle structopt "flatten" attribute
                todo!();
            } else {
                let ty = &field.ty;
                let ty = match ty {
                    Type::Path(path) => proc_macro_roids::format_path(&path.path),
                    _ => panic!("TODO"),
                };
                // TODO: Handle "rename_all" structopt attribute
                // TODO: Handle structopt "name" attribute
                // TODO: Handle enums
                // structopt's default renaming is kebab case
                // let mut arg_name = ident.to_string().to_kebab_case();
                // TODO: This is a temporary hack
                let mut arg_name = ident.to_string();
                arg_name.make_ascii_uppercase();
                // Check if the structopt name is set if it is pull it out and use that as the arg_name
                // TODO: Clean this up
                // println!("{:?}", field.attrs.len());
                // println!("{:?}", proc_macro_roids::namespace_meta_lists(&field.attrs, &parse_quote!(structopt)).len());
                // for nested_meta in
                //     proc_macro_roids::namespace_parameters(&field.attrs, &parse_quote!(structopt))
                // {
                //     println!("nested_meta");
                //     match nested_meta {
                //         NestedMeta::Meta(meta) => match meta {
                //             Meta::Path(_) => {}
                //             Meta::List(_) => {}
                //             Meta::NameValue(meta) => {
                //                 println!("{:?}", meta.path);
                //                 if meta.path == parse_quote!(name) {
                //                     match meta.lit {
                //                         Lit::Str(s) => arg_name = s.value(),
                //                         _ => {},
                //                     }
                //                 }
                //             }
                //         },
                //         NestedMeta::Lit(_) => {}
                //     }
                // }
                // println!("NAME {:?}", arg_name);
                // TODO: Clean this up
                if ty.contains("Option<PathBuf") {
                    quote_spanned! {span=>
                        [#arg_name] => self.#ident.as_ref().and_then(|v| v.as_ref().map(|v| v.to_string_lossy().into_owned())),
                    }
                } else if ty.contains("Option<") {
                    quote_spanned! {span=>
                        [#arg_name] => self.#ident.as_ref().and_then(|v| v.as_ref().map(|v| v.to_string())),
                    }
                } else if ty.contains("Vec<") {
                    quote_spanned! {span=>
                        [#arg_name] => self.#ident.as_ref().map(|v| v.join(",")),
                    }
                }else if ty.contains("PathBuf") {
                    quote_spanned! {span=>
                        [#arg_name] => self.#ident.as_ref().map(|v| v.to_string_lossy().into_owned()),
                    }
                } else {
                    quote_spanned! {span=>
                        [#arg_name] => self.#ident.as_ref().map(|v| v.to_string()),
                    }
                }
            }
        })
        .collect()
}
