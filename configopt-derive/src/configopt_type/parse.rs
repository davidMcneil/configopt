mod configopt_parser;
mod structopt_parser;

use configopt_parser::ConfigOptAttr;
use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use proc_macro_roids::IdentExt;
use std::{convert::Infallible, str::FromStr};
use structopt_parser::StructOptAttr;
use syn::{parse_quote, spanned::Spanned, Expr, Field, Fields, Ident, Type, Variant};

pub use structopt_parser::{
    rename_all as structopt_rename_all, trim_structopt_default_value_attr, StructOptTy,
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
    fn rename(self, s: &str) -> String {
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
    structopt_ty: StructOptTy,
    configopt_inner_ty: Ident,
    span: Span,
    flatten: bool,
    subcommand: bool,
    structopt_name: String,
    serde_name: String,
    to_default: Option<Expr>,
}

impl ParsedField {
    pub fn new(field: &Field, structopt_rename: CasingStyle, serde_rename: CasingStyle) -> Self {
        let ident = field.ident.clone().expect("field ident to exist");
        let ty = &field.ty;
        let mut_ty = &mut field.ty.clone();
        let inner_ty = inner_ty(&mut mut_ty.clone()).clone();
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
            structopt_ty: StructOptTy::from_syn_ty(&ty),
            configopt_inner_ty: configopt_ident(&inner_ty),
            span: field.span(),
            structopt_name,
            serde_name,
            flatten: structopt_attrs.iter().any(|a| match a {
                StructOptAttr::Flatten => true,
                _ => false,
            }),
            subcommand: structopt_attrs.iter().any(|a| match a {
                StructOptAttr::Subcommand => true,
                _ => false,
            }),
            to_default: configopt_attrs.into_iter().find_map(|a| match a {
                ConfigOptAttr::ToDefault(expr) => Some(expr),
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

    pub fn configopt_inner_ty(&self) -> &Ident {
        &self.configopt_inner_ty
    }

    pub fn flatten(&self) -> bool {
        self.flatten
    }

    pub fn subcommand(&self) -> bool {
        self.subcommand
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

#[derive(Clone, Copy)]
pub enum FieldType {
    Named,
    Unnamed,
    Unit,
}

impl From<&Fields> for FieldType {
    fn from(fields: &Fields) -> Self {
        match fields {
            Fields::Named(_) => Self::Named,
            Fields::Unnamed(_) => Self::Unnamed,
            Fields::Unit => Self::Unit,
        }
    }
}

pub struct ParsedVariant {
    full_configopt_ident: TokenStream,
    span: Span,
    field_type: FieldType,
}

impl ParsedVariant {
    pub fn new(type_ident: &Ident, variant: &Variant) -> Self {
        let variant_ident = &variant.ident;
        let configopt_type_ident = configopt_ident(&type_ident);
        let full_configopt_ident = parse_quote! {#configopt_type_ident::#variant_ident};

        Self {
            full_configopt_ident,
            span: variant.span(),
            field_type: (&variant.fields).into(),
        }
    }

    pub fn full_configopt_ident(&self) -> &TokenStream {
        &self.full_configopt_ident
    }

    pub fn field_type(&self) -> FieldType {
        self.field_type
    }
}

impl Spanned for ParsedVariant {
    fn span(&self) -> Span {
        self.span
    }
}
