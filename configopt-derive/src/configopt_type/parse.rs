#[macro_use]
mod attribute_trimmer;
pub mod configopt_fields_attr_parser;
mod configopt_parser;
mod serde_parser;
mod structopt_parser;

use configopt_parser::ConfigOptAttr;
use heck::{CamelCase, KebabCase, MixedCase, ShoutySnakeCase, SnakeCase};
use proc_macro2::{Span, TokenStream};
use proc_macro_roids::IdentExt;
use serde_parser::SerdeAttr;
use std::{convert::Infallible, str::FromStr};
use structopt_parser::StructOptAttr;
use syn::{parse_quote, spanned::Spanned, Attribute, Expr, Field, Fields, Ident, Type, Variant};

pub use serde_parser::trim_attr as trim_serde_attr;
pub use structopt_parser::{
    rename_all as structopt_rename_all, trim_attr as trim_structopt_attr, StructOptTy,
};

pub fn configopt_ident(ident: &Ident) -> Ident {
    ident.prepend("ConfigOpt")
}

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

impl CasingStyle {
    pub fn rename(self, s: impl AsRef<str>) -> String {
        let s = s.as_ref();
        match self {
            CasingStyle::Kebab => s.to_kebab_case(),
            CasingStyle::Snake => s.to_snake_case(),
            CasingStyle::ScreamingSnake => s.to_shouty_snake_case(),
            CasingStyle::Camel => s.to_mixed_case(),
            CasingStyle::Pascal => s.to_camel_case(),
            CasingStyle::Verbatim => String::from(s),
        }
    }
}

pub fn inner_ty(ty: &mut Type) -> &mut Ident {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last_mut() {
                &mut segment.ident
            } else {
                panic!(
                    "`#[configopt]` could not find a last segment in the type path to make partial"
                );
            }
        }
        _ => {
            panic!("`#[configopt]` only supports types specified by a path");
        }
    }
}

pub fn has_configopt_fields(parsed: &[ParsedField]) -> bool {
    parsed.iter().any(|f| f.ident() == "generate_config")
}

#[derive(Clone)]
pub struct ParsedField {
    ident: Ident,
    structopt_ty: StructOptTy,
    configopt_inner_ty: Ident,
    span: Span,
    structopt_flatten: bool,
    serde_flatten: bool,
    subcommand: bool,
    no_wrap: bool,
    structopt_rename: CasingStyle,
    structopt_name: String,
    serde_name: String,
    to_os_string: Option<Expr>,
}

impl ParsedField {
    pub fn new(
        field: &mut Field,
        structopt_rename: CasingStyle,
        serde_rename: CasingStyle,
        retained_attrs: &[Ident],
    ) -> Self {
        let ident = field.ident.clone().expect("field ident to exist");

        let configopt_attrs = configopt_parser::parse_attrs(&field.attrs);
        let no_wrap = configopt_attrs
            .iter()
            .any(|a| matches!(a, ConfigOptAttr::NoWrap));

        let structopt_ty = StructOptTy::from_syn_ty(&field.ty);
        let ty = &mut field.ty;
        let inner_ty = inner_ty(ty);
        let configopt_inner_ty = if no_wrap {
            inner_ty.clone()
        } else {
            configopt_ident(&inner_ty)
        };

        let structopt_attrs = structopt_parser::parse_attrs(&field.attrs);
        let serde_attrs = serde_parser::parse_attrs(&field.attrs);
        let serde_name = serde_rename.rename(&ident.to_string());
        let structopt_name = structopt_attrs
            .iter()
            .find_map(|a| match &a {
                StructOptAttr::NameLitStr(name) => Some(name.clone()),
                _ => None,
            })
            .unwrap_or_else(|| structopt_rename.rename(&ident.to_string()));
        let structopt_flatten = structopt_attrs.iter().any(|a| match a {
            StructOptAttr::Flatten => true,
            _ => false,
        });
        let subcommand = structopt_attrs.iter().any(|a| match a {
            StructOptAttr::Subcommand => true,
            _ => false,
        });

        // The below logic converts the field into a `ConfigOpt` field

        // If the field is flattened or a subcommand, modify the type with the configopt type prefix
        if structopt_flatten || subcommand {
            *inner_ty = configopt_inner_ty.clone();
        }

        retain_attrs(&mut field.attrs, &retained_attrs);

        // If this field was a `Vec` we need to add a default value to allow deserializing the
        // `ConfigOpt` type from an empty input.
        if let StructOptTy::Vec = structopt_ty {
            if retained_attrs.iter().any(|a| a == "serde") {
                field.attrs.push(parse_quote! {#[serde(default)]})
            }
        }

        // If the field is not already, wrap its type in an `Option`. This guarantees that the
        // `ConfigOpt` struct can be parsed regardless of complete CLI input.
        if let StructOptTy::Bool | StructOptTy::Other = structopt_ty {
            // If it was a flattened field all of its fields will be optional so it does not need to
            // be wrapped in an `Option`
            if !structopt_flatten {
                field.ty = parse_quote!(Option<#ty>);
            }
            // If this field was a `bool` we need to add a default of `true` now that it is wrapped in
            // an `Option`. This preserves the same behavior as if we just had a `bool`, but allows us
            // to detect if the `bool` even has a value. Essentially, it adds a third state of not set
            // (None) to this field.
            if let StructOptTy::Bool = structopt_ty {
                field
                    .attrs
                    .push(parse_quote! {#[structopt(default_value = "true")]})
            }
        }

        Self {
            ident,
            structopt_ty,
            configopt_inner_ty,
            span: field.span(),
            structopt_rename,
            structopt_name,
            serde_name,
            structopt_flatten,
            serde_flatten: serde_attrs.iter().any(|a| match a {
                SerdeAttr::Flatten => true,
                _ => false,
            }),
            subcommand,
            no_wrap,
            to_os_string: configopt_attrs.into_iter().find_map(|a| match a {
                ConfigOptAttr::ToOsString(expr) => Some(expr),
                _ => None,
            }),
        }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn structopt_ty(&self) -> &StructOptTy {
        &self.structopt_ty
    }

    #[allow(unused)]
    pub fn configopt_inner_ty(&self) -> &Ident {
        &self.configopt_inner_ty
    }

    pub fn structopt_flatten(&self) -> bool {
        self.structopt_flatten
    }

    pub fn serde_flatten(&self) -> bool {
        self.serde_flatten
    }

    pub fn subcommand(&self) -> bool {
        self.subcommand
    }

    pub fn no_wrap(&self) -> bool {
        self.no_wrap
    }

    pub fn structopt_rename(&self) -> CasingStyle {
        self.structopt_rename
    }

    pub fn structopt_name(&self) -> &str {
        &self.structopt_name
    }

    pub fn serde_name(&self) -> &str {
        &self.serde_name
    }

    pub fn to_os_string(&self) -> Option<&Expr> {
        self.to_os_string.as_ref()
    }
}

impl Spanned for ParsedField {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone)]
pub enum FieldType {
    Named(Vec<ParsedField>),
    Unnamed,
    Unit,
}

impl FieldType {
    fn new(
        fields: &mut Fields,
        structopt_rename: CasingStyle,
        serde_rename: CasingStyle,
        retained_attrs: &[Ident],
    ) -> Self {
        match fields {
            Fields::Named(named_fields) => Self::Named(
                named_fields
                    .named
                    .iter_mut()
                    .map(|f| ParsedField::new(f, structopt_rename, serde_rename, retained_attrs))
                    .collect(),
            ),
            Fields::Unnamed(_) => Self::Unnamed,
            Fields::Unit => Self::Unit,
        }
    }
}

pub struct ParsedVariant {
    full_ident: TokenStream,
    full_configopt_ident: TokenStream,
    span: Span,
    field_type: FieldType,
    structopt_name: String,
}

impl ParsedVariant {
    pub fn new(
        type_ident: &Ident,
        variant: &mut Variant,
        structopt_rename: CasingStyle,
        serde_rename: CasingStyle,
        retained_attrs: &[Ident],
    ) -> Self {
        let variant_ident = &variant.ident;
        let full_ident = parse_quote! {#type_ident::#variant_ident};
        let configopt_type_ident = configopt_ident(&type_ident);
        let full_configopt_ident = parse_quote! {#configopt_type_ident::#variant_ident};

        // The below logic converts the variant into a `ConfigOpt` variant
        let field_type = FieldType::new(
            &mut variant.fields,
            structopt_rename,
            serde_rename,
            retained_attrs,
        );
        if let Fields::Unnamed(fields) = &mut variant.fields {
            if fields.unnamed.len() > 1 {
                panic!(
                    "`ConfigOpt` cannot be derived on unnamed enums with a length greater than 1"
                );
            }
            // Modify the type with the configopt type prefix
            let field = &mut fields.unnamed[0];
            let ty = inner_ty(&mut field.ty);
            *ty = configopt_ident(ty);
        }

        Self {
            full_ident,
            full_configopt_ident,
            span: variant.span(),
            field_type,
            // TODO: Actually lookup the `structopt` name
            structopt_name: variant_ident.to_string().to_kebab_case(),
        }
    }

    pub fn full_ident(&self) -> &TokenStream {
        &self.full_ident
    }

    pub fn full_configopt_ident(&self) -> &TokenStream {
        &self.full_configopt_ident
    }

    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }

    pub fn structopt_name(&self) -> &str {
        &self.structopt_name
    }
}

impl Spanned for ParsedVariant {
    fn span(&self) -> Span {
        self.span
    }
}

// Only retain attributes we have explicitly opted to preserve
pub fn retain_attrs(attrs: &mut Vec<Attribute>, retained_attrs: &[Ident]) {
    attrs.retain(|a| retained_attrs.iter().any(|i| a.path.is_ident(i)));
    for attr in attrs {
        trim_structopt_attr(attr);
        trim_serde_attr(attr);
    }
}
