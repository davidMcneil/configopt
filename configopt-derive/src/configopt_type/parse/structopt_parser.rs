use super::CasingStyle;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Expr, GenericArgument, Ident, LitStr, Path, PathArguments,
    PathArguments::AngleBracketed,
    PathSegment, Token, Type, TypePath,
};

#[derive(PartialEq)]
pub enum StructOptAttr {
    RenameAll(CasingStyle),
    NameLitStr(String),
    Flatten,
    Subcommand,
    // We only care about some of the structopt attributes
    Unknown,
}

impl Parse for StructOptAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            input.parse::<Token![=]>()?; // skip '='

            if input.peek(LitStr) {
                let lit: LitStr = input.parse()?;
                let lit_str = lit.value();

                match &*name_str {
                    "rename_all" => Ok(StructOptAttr::RenameAll(
                        lit_str.parse().expect("infallible parse"),
                    )),
                    "name" => Ok(StructOptAttr::NameLitStr(lit_str)),
                    _ => Ok(StructOptAttr::Unknown),
                }
            } else {
                match input.parse::<Expr>() {
                    Ok(_) => {
                        if name_str == "name" {
                            panic!("`configopt` parsing `structopt` only supports string literal for argument name")
                        }
                    }
                    Err(e) => {
                        panic!("`configopt` parsing `structopt` expected `string literal` or `expression` after `=`, err: {}", e)
                    }
                }
                Ok(StructOptAttr::Unknown)
            }
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            let _nested;
            // Even though we do not do anything here we still need to consume the tokens from the ParseStream
            parenthesized!(_nested in input);
            Ok(StructOptAttr::Unknown)
        } else {
            // Attributes represented with a sole identifier.
            Ok(match name_str.as_ref() {
                "flatten" => StructOptAttr::Flatten,
                "subcommand" => StructOptAttr::Subcommand,
                _ => StructOptAttr::Unknown,
            })
        }
    }
}

pub fn parse_attrs(attrs: &[Attribute]) -> Vec<StructOptAttr> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("structopt"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<StructOptAttr, Token![,]>::parse_terminated)
                .expect("`configopt` failed to parse `structopt` attributes")
        })
        .collect()
}

struct TrimDefaultValueAttribute(Option<TokenStream>);

impl Parse for TrimDefaultValueAttribute {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        // let name: Ident = input.parse()?;
        Ok(if false {
            Self(None)
        } else {
            // This was not a `default_value` attribute so reassemble the TokenStream
            let token_stream = input.parse::<TokenStream>()?;
            // Self(Some(parse_quote! {#name#token_stream}))
            Self(Some(token_stream))
        })
    }
}

fn trim_required_attr(input: ParseStream) -> syn::Result<TokenStream> {
    let name: Ident = input.parse()?;
    let name_str = name.to_string();

    Ok(if input.peek(Token![=]) {
        // `name = value` attributes.
        input.parse::<Token![=]>()?; // skip '='

        let token_stream = if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            quote! {#lit}
        } else {
            match input.parse::<Expr>() {
                Ok(expr) => {
                    quote! {#expr}
                }
                Err(e) => {
                    panic!("`configopt` parsing `structopt` expected `string literal` or `expression` after `=`, err: {}", e)
                }
            }
        };
        match name_str.as_ref() {
            "required" => quote! {},
            _ => quote! {#name = #token_stream},
        }
    } else if input.peek(syn::token::Paren) {
        // `name(...)` attributes.
        let nested;
        parenthesized!(nested in input);
        let token_stream: TokenStream = nested.parse()?;
        quote! {#name(#token_stream)}
    } else {
        // Attributes represented with a sole identifier.
        match name_str.as_ref() {
            "required" => quote! {},
            _ => quote! {#name},
        }
    })
}

fn trim_required_attrs(input: ParseStream) -> syn::Result<Punctuated<TokenStream, Token![,]>> {
    Ok(input
        .parse_terminated::<_, Token![,]>(trim_required_attr)?
        .into_iter()
        .filter(|p| !p.is_empty())
        .collect())
}

/// Default values do not make sense for any fields of the fully optional `ConfigOpt` so we trim
/// them off
pub fn trim_structopt_required_attr(attr: &mut Attribute) {
    if !attr.path.is_ident("structopt") {
        return;
    }
    let tokens = attr
        .parse_args_with(trim_required_attrs)
        .expect("`ConfigOpt` failed to trim `structopt::default_value` attributes");
    attr.tokens = quote! {(#tokens)};
}

pub fn rename_all(attrs: &[Attribute]) -> Option<CasingStyle> {
    parse_attrs(attrs).into_iter().find_map(|a| match a {
        StructOptAttr::RenameAll(style) => Some(style),
        _ => None,
    })
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StructOptTy {
    Bool,
    Vec,
    Option,
    OptionOption,
    OptionVec,
    Other,
}

impl StructOptTy {
    pub fn from_syn_ty(ty: &syn::Type) -> Self {
        use StructOptTy::*;

        if is_simple_ty(ty, "bool") {
            Bool
        } else if is_generic_ty(ty, "Vec") {
            Vec
        } else if let Some(subty) = subty_if_name(ty, "Option") {
            if is_generic_ty(subty, "Option") {
                OptionOption
            } else if is_generic_ty(subty, "Vec") {
                OptionVec
            } else {
                Option
            }
        } else {
            Other
        }
    }
}

fn only_last_segment(ty: &syn::Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(TypePath {
            qself: None,
            path:
                Path {
                    leading_colon: None,
                    segments,
                },
        }) => only_one(segments.iter()),

        _ => None,
    }
}

fn subty_if<F>(ty: &syn::Type, f: F) -> Option<&syn::Type>
where
    F: FnOnce(&PathSegment) -> bool,
{
    only_last_segment(ty)
        .filter(|segment| f(segment))
        .and_then(|segment| {
            if let AngleBracketed(args) = &segment.arguments {
                only_one(args.args.iter()).and_then(|generic| {
                    if let GenericArgument::Type(ty) = generic {
                        Some(ty)
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
}

fn subty_if_name<'a>(ty: &'a syn::Type, name: &str) -> Option<&'a syn::Type> {
    subty_if(ty, |seg| seg.ident == name)
}

fn is_simple_ty(ty: &syn::Type, name: &str) -> bool {
    only_last_segment(ty)
        .map(|segment| {
            if let PathArguments::None = segment.arguments {
                segment.ident == name
            } else {
                false
            }
        })
        .unwrap_or(false)
}

fn is_generic_ty(ty: &syn::Type, name: &str) -> bool {
    subty_if_name(ty, name).is_some()
}

fn only_one<I, T>(mut iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
{
    iter.next().filter(|_| iter.next().is_none())
}
