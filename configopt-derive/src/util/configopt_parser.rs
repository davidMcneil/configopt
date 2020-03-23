use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Expr, Ident, Token,
};

#[derive(PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum ConfigOptAttr {
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

            #[allow(clippy::match_wild_err_arm)]
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

pub fn parse_attrs(attrs: &[Attribute]) -> Vec<ConfigOptAttr> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("configopt"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<ConfigOptAttr, Token![,]>::parse_terminated)
                .expect("`configopt` failed to parse `configopt` attributes")
        })
        .collect()
}
