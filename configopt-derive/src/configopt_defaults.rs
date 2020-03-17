use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use std::{convert::Infallible, str::FromStr};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Data, Expr, Field, Fields, FieldsNamed, GenericArgument, Ident, LitStr, Path,
    PathArguments,
    PathArguments::AngleBracketed,
    PathSegment, Token, Type, TypePath,
};

#[derive(Clone, Copy, PartialEq)]
pub enum CasingStyle {
    Camel,
    Kebab,
    Pascal,
    ScreamingSnake,
    Snake,
    Verbatim,
}

impl FromStr for CasingStyle {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "camel" | "camelcase" => Self::Camel,
            "kebab" | "kebabcase" => Self::Kebab,
            "pascal" | "pascalcase" => Self::Pascal,
            "screamingsnake" | "screamingsnakecase" => Self::ScreamingSnake,
            "snake" | "snakecase" => Self::Snake,
            "verbatim" | "verbatimcase" => Self::Verbatim,
            _ => panic!("Invalid value for `rename_all` attribute"),
        })
    }
}

#[derive(PartialEq)]
enum StructOptAttr {
    RenameAll(CasingStyle),
    NameLitStr(String),
    // We only care about some of the structopt attributes
    Unknown,
}

impl Parse for StructOptAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        // A significant portion of this code was copied directly from `structopt`

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
            Ok(StructOptAttr::Unknown)
        }
    }
}

fn parse_structopt_attrs(attrs: &[Attribute]) -> Vec<StructOptAttr> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("structopt"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<StructOptAttr, Token![,]>::parse_terminated)
                .expect("`configopt` failed to parse `structopt` attributes")
        })
        .collect()
}

pub fn structopt_rename_all(attrs: &[Attribute]) -> Option<CasingStyle> {
    parse_structopt_attrs(attrs)
        .into_iter()
        .find_map(|a| match a {
            StructOptAttr::RenameAll(style) => Some(style),
            _ => None,
        })
}

#[derive(PartialEq)]
#[allow(clippy::large_enum_variant)]
enum ConfigOptAttr {
    Nested,
    ToDefault(Expr),
}

impl Parse for ConfigOptAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            input.parse::<Token![=]>()?; // skip '='

            match input.parse::<Expr>() {
                Ok(expr) => {
                    if name_str == "to_default" {
                        Ok(ConfigOptAttr::ToDefault(expr))
                    } else {
                        panic!(
                            "`configopt` unrecognized `name = value` attribute {}",
                            name_str
                        );
                    }
                }
                Err(_) => panic!("`configopt` parsing `structopt` expected `expression` after `=`"),
            }
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            panic!("`configopt` does not have any `name(...)` attributes")
        } else {
            // Attributes represented with a sole identifier.
            Ok(match name_str.as_ref() {
                "nested" => ConfigOptAttr::Nested,
                s => panic!("`configopt` unrecognized sole identifier attribute {}", s),
            })
        }
    }
}

fn parse_configopt_attrs(attrs: &[Attribute]) -> Vec<ConfigOptAttr> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("configopt"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<ConfigOptAttr, Token![,]>::parse_terminated)
                .expect("`configopt` failed to parse `configopt` attributes")
        })
        .collect()
}

struct ParsedField<'a> {
    field: &'a Field,
    ident: &'a Ident,
    name: Option<String>,
    to_default: Option<Expr>,
    nested: bool,
}

impl<'a> ParsedField<'a> {
    fn new(field: &'a Field) -> Self {
        // We take into account
        let structopt_attrs = parse_structopt_attrs(&field.attrs);
        let configopt_attrs = parse_configopt_attrs(&field.attrs);
        Self {
            field,
            ident: field.ident.as_ref().expect("field name"),
            name: structopt_attrs.iter().find_map(|a| match &a {
                StructOptAttr::NameLitStr(name) => Some(name.clone()),
                _ => None,
            }),
            to_default: configopt_attrs.iter().find_map(|a| match &a {
                ConfigOptAttr::ToDefault(expr) => Some(expr.clone()),
                _ => None,
            }),
            nested: configopt_attrs.iter().any(|v| v == &ConfigOptAttr::Nested),
        }
    }

    fn name(&self, rename: CasingStyle) -> String {
        if let Some(arg_name) = &self.name {
            arg_name.clone()
        } else {
            let arg_name = self.ident.to_string();
            match rename {
                CasingStyle::Kebab => arg_name.to_kebab_case(),
                CasingStyle::Snake => arg_name.to_snake_case(),
                CasingStyle::ScreamingSnake => arg_name.to_screaming_snake_case(),
                CasingStyle::Camel => arg_name.to_camel_case(),
                CasingStyle::Pascal => arg_name.to_pascal_case(),
                CasingStyle::Verbatim => arg_name,
            }
        }
    }

    fn to_default(&self) -> TokenStream {
        let ident = self.ident;
        let span = self.field.span();
        if self.nested {
            // If this is a nested struct recursively call arg_default
            quote_spanned! {span=>
                self.#ident.arg_default(arg_path)
            }
        } else {
            // If this had a custom to_default, use that otherwise use ToString
            let to_default = if let Some(expr) = &self.to_default {
                quote! {
                    #expr(&v)
                }
            } else {
                quote! {
                    ::std::ffi::OsString::from(ToString::to_string(&v)),
                }
            };
            // Code to join a Vec<OsString> into a OsString
            let join_os_strs = quote! {
                let mut result = ::std::ffi::OsString::new();
                for (i, v) in vec.iter().enumerate() {
                    if i != 0 {
                        result.push(",");
                    }
                    result.push(&v);
                }
                result
            };
            // Based on the type of the field convert it to a String. Everything is wrapped
            // in an Option because this is always run on a `Partial` type.
            match Ty::from_syn_ty(&self.field.ty) {
                Ty::Bool | Ty::Other => quote_spanned! {span=>
                    self.#ident
                        .as_ref()
                        .map(|v| #to_default)
                },
                Ty::Vec => quote_spanned! {span=>
                    self.#ident
                        .as_ref()
                        .map(|vec| {
                            let vec = vec.iter()
                                .map(|v| #to_default)
                                .collect::<Vec<_>>();
                            #join_os_strs
                        })
                },
                Ty::Option => quote_spanned! {span=>
                    self.#ident
                        .as_ref()
                        .and_then(|o| o.as_ref().map(|v| #to_default))
                },
                Ty::OptionOption => quote_spanned! {span=>
                    self.#ident
                        .as_ref()
                        .and_then(|oo|
                            oo.as_ref().and_then(|o| o.as_ref().map(|v| #to_default)))
                },
                Ty::OptionVec => quote_spanned! {span=>
                    self.#ident
                        .as_ref()
                        .and_then(|o| o.as_ref().map(|vec| {
                            let vec = vec.iter()
                                .map(|v| #to_default)
                                .collect::<Vec<_>>();
                            #join_os_strs
                        }))
                },
            }
        }
    }
}

pub fn match_arms(data: &Data, rename: CasingStyle) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(FieldsNamed { named, .. }) => named
                .into_iter()
                .map(|field| {
                    let parsed_field = ParsedField::new(field);
                    let arg_name = parsed_field.name(rename);
                    let to_default = parsed_field.to_default();
                    quote! {
                        #arg_name => #to_default,
                    }
                })
                .collect(),
            Fields::Unnamed(_) => panic!("`ConfigOpt` cannot be derived for unnamed struct"),
            Fields::Unit => panic!("`ConfigOpt` cannot be derived for unit structs"),
        },
        Data::Enum(_) => panic!("`ConfigOpt` cannot be derived for enums"),
        Data::Union(_) => panic!("`ConfigOpt` cannot be derived for unions"),
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
// This is taken directly from `structopt`
enum Ty {
    Bool,
    Vec,
    Option,
    OptionOption,
    OptionVec,
    Other,
}

impl Ty {
    fn from_syn_ty(ty: &syn::Type) -> Self {
        use Ty::*;

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
                only_one(args.args.iter()).and_then(|genneric| {
                    if let GenericArgument::Type(ty) = genneric {
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

pub fn subty_if_name<'a>(ty: &'a syn::Type, name: &str) -> Option<&'a syn::Type> {
    subty_if(ty, |seg| seg.ident == name)
}

pub fn is_simple_ty(ty: &syn::Type, name: &str) -> bool {
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
