use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    Expr, Ident, Token,
};

#[derive(PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum ConfigOptFieldsAttr {
    Hidden(Expr),
}

impl Parse for ConfigOptFieldsAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            input.parse::<Token![=]>()?; // skip '='

            #[allow(clippy::match_wild_err_arm)]
            match input.parse::<Expr>() {
                Ok(expr) => {
                    if name_str == "hidden" {
                        Ok(ConfigOptFieldsAttr::Hidden(expr))
                    } else {
                        panic!("`configopt_fields` unrecognized `{} = ...`", name_str);
                    }
                }
                Err(_) => panic!(
                    "`configopt_field` expected `expression` after `{} = ...`",
                    name_str
                ),
            }
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            panic!(
                "`configopt_fields` does not have attribute `{}(...)`",
                name_str
            )
        } else {
            // Attributes represented with a sole identifier.
            panic!(
                "`configopt_fields` unrecognized sole identifier attribute {}",
                name_str
            )
        }
    }
}

pub fn parse(token_stream: TokenStream) -> Vec<ConfigOptFieldsAttr> {
    Parser::parse(
        <Punctuated<ConfigOptFieldsAttr, Token![,]>>::parse_terminated,
        token_stream,
    )
    .expect("`configopt_fields` failed to parse attrs")
    .into_iter()
    .collect()
}
