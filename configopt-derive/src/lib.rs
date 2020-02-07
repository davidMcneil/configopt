extern crate proc_macro;

use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use std::{convert::Infallible, str::FromStr};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Data, DeriveInput, Expr, Field, Fields, FieldsNamed, Ident, Lit, LitStr, Meta,
    MetaNameValue, NestedMeta, Path, Token,
};

#[proc_macro_derive(ConfigOptDefaults, attributes(configopt_defaults))]
pub fn configopt_defaults_derive(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as DeriveInput);
    let ident = if let Some(ident) = get_configopt_defaults_attr_name_str_value(&ast.attrs, "type")
    {
        Ident::new(&ident, Span::call_site())
    } else {
        ast.ident
    };
    let rename_type = get_structopt_rename_all(&ast.attrs)
        // Structopt defaults to kebab case if no `rename_all` attribute is specified
        .unwrap_or(CasingStyle::Kebab);
    let arg_default_match_arms = arg_default_match_arms(&ast.data, rename_type);
    let expanded = quote! {
        impl ConfigOptDefaults for #ident {
            fn arg_default(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
                if let Some((arg_name, arg_path)) = arg_path.split_first() {
                    match arg_name.as_str() {
                        #arg_default_match_arms
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

#[derive(Clone, Copy)]
enum CasingStyle {
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

                match &*name_str.to_string() {
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
                    Err(_) => {
                        panic!("`configopt` parsing `structopt` expected `string literal` or `expression` after `=`")
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

// We need a custom parser to handle structopt attributes
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

fn get_structopt_rename_all(attrs: &[Attribute]) -> Option<CasingStyle> {
    parse_structopt_attrs(attrs)
        .into_iter()
        .find_map(|a| match a {
            StructOptAttr::RenameAll(style) => Some(style),
            _ => None,
        })
}

fn get_structopt_name(attrs: &[Attribute]) -> Option<String> {
    parse_structopt_attrs(attrs)
        .into_iter()
        .find_map(|a| match a {
            StructOptAttr::NameLitStr(name) => Some(name),
            _ => None,
        })
}

fn get_meta_name_str_value(meta: &Meta, name: &str) -> Option<String> {
    match meta {
        Meta::Path(_) => {}
        Meta::List(list) => {
            for nested_meta in list.nested.iter() {
                match nested_meta {
                    NestedMeta::Meta(meta) => {
                        if let Some(value) = get_meta_name_str_value(meta, name) {
                            return Some(value);
                        }
                    }
                    NestedMeta::Lit(_) => {}
                }
            }
        }
        Meta::NameValue(MetaNameValue { path, lit, .. }) => {
            if path.is_ident(name) {
                match lit {
                    Lit::Str(s) => {
                        return Some(s.value());
                    }
                    _ => {}
                }
            }
        }
    };
    None
}

fn get_namespace_attr_name_str_value(
    attrs: &[Attribute],
    namespace: &Path,
    name: &str,
) -> Option<String> {
    for attr in attrs {
        if &attr.path != namespace {
            continue;
        }
        if let Ok(meta) = attr.parse_meta() {
            if let Some(value) = get_meta_name_str_value(&meta, name) {
                return Some(value);
            }
        }
    }
    None
}

fn get_configopt_defaults_attr_name_str_value(attrs: &[Attribute], name: &str) -> Option<String> {
    let namespace = parse_quote!(configopt_defaults);
    get_namespace_attr_name_str_value(attrs, &namespace, name)
}

fn arg_name(field: &Field, rename: CasingStyle) -> String {
    if let Some(arg_name) = get_structopt_name(&field.attrs) {
        arg_name
    } else {
        let arg_name = field.ident.as_ref().expect("field name").to_string();
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

fn arg_default_match_arms(data: &Data, rename: CasingStyle) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(FieldsNamed { named, .. }) => named
                .into_iter()
                .map(|field| {
                    let ident = field.ident.as_ref().expect("field name");
                    let arg_name = arg_name(field, rename);
                    let span = field.span();
                    quote_spanned! {span=>
                        #arg_name => self.#ident.arg_default(arg_path),
                    }
                })
                .collect(),
            Fields::Unnamed(_) => panic!("`Partial` cannot be derived for unnamed struct"),
            Fields::Unit => panic!("`Partial` cannot be derived for unit structs"),
        },
        Data::Enum(_) => panic!("`Partial` cannot be derived for enums"),
        Data::Union(_) => panic!("`Partial` cannot be derived for unions"),
    }
}
