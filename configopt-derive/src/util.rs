mod configopt_parser;
mod structopt_parser;

use crate::partial;
use configopt_parser::ConfigOptAttr;
use inflector::Inflector;
use proc_macro2::Span;
use std::{convert::Infallible, str::FromStr};
use structopt_parser::StructOptAttr;
use syn::{parse_quote, spanned::Spanned, Data, Expr, Field, Fields, Ident, Type};

pub use structopt_parser::{rename_all as structopt_rename_all, StructOptTy};

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
    fn rename(&self, s: &str) -> String {
        match self {
            CasingStyle::Kebab => s.to_kebab_case(),
            CasingStyle::Snake => s.to_snake_case(),
            CasingStyle::ScreamingSnake => s.to_screaming_snake_case(),
            CasingStyle::Camel => s.to_camel_case(),
            CasingStyle::Pascal => s.to_pascal_case(),
            CasingStyle::Verbatim => String::from(s),
        }
    }
}

/// Check if a field is annotated with #[configopt(nested)]
pub fn has_configopt_nested_attr(field: &Field) -> bool {
    proc_macro_roids::contains_tag(
        &field.attrs,
        &parse_quote!(configopt),
        &parse_quote!(nested),
    )
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

pub struct ParsedField {
    ident: Ident,
    ty: Type,
    span: Span,
    nested: bool,
    structopt_name: String,
    serde_name: String,
    to_default: Option<Expr>,
}

impl ParsedField {
    fn new(field: &Field, structopt_rename: CasingStyle, serde_rename: CasingStyle) -> Self {
        let ident = field.ident.clone().expect("field ident to exist");
        let structopt_attrs = structopt_parser::parse_attrs(&field.attrs);
        let configopt_attrs = configopt_parser::parse_attrs(&field.attrs);

        let structopt_name = structopt_attrs
            .iter()
            .find_map(|a| match &a {
                StructOptAttr::NameLitStr(name) => Some(name.clone()),
                _ => None,
            })
            .unwrap_or_else(|| structopt_rename.rename(&ident.to_string()));

        let serde_name = serde_rename.rename(&ident.to_string());

        Self {
            ident,
            ty: field.ty.clone(),
            span: field.span(),
            structopt_name,
            serde_name,
            nested: has_configopt_nested_attr(field),
            to_default: configopt_attrs.into_iter().find_map(|a| match a {
                ConfigOptAttr::ToDefault(expr) => Some(expr),
                _ => None,
            }),
        }
    }

    pub fn parse_fields(
        data: &Data,
        structopt_rename: CasingStyle,
        serde_rename: CasingStyle,
    ) -> Vec<ParsedField> {
        match data {
            Data::Struct(data) => match &data.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|field| ParsedField::new(field, structopt_rename, serde_rename))
                    .collect::<Vec<_>>(),
                Fields::Unnamed(_) => panic!("`ConfigOpt` cannot be derived for unnamed struct"),
                Fields::Unit => panic!("`ConfigOpt` cannot be derived for unit structs"),
            },
            Data::Enum(_) => panic!("`ConfigOpt` cannot be derived for enums"),
            Data::Union(_) => panic!("`ConfigOpt` cannot be derived for unions"),
        }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn inner_ty(&self) -> Ident {
        inner_ty(&mut self.ty.clone()).clone()
    }

    pub fn partial_inner_ty(&self) -> Ident {
        partial::partial_ident(&self.inner_ty())
    }

    pub fn nested(&self) -> bool {
        self.nested
    }

    pub fn structopt_name(&self) -> &str {
        &self.structopt_name
    }

    pub fn serde_name(&self) -> &str {
        &self.serde_name
    }

    pub fn to_default(&self) -> Option<&Expr> {
        self.to_default.as_ref()
    }
}

impl Spanned for ParsedField {
    fn span(&self) -> Span {
        self.span
    }
}
