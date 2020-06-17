use proc_macro2::TokenStream;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Expr, Ident, LitStr, Token,
};

#[derive(PartialEq)]
pub enum SerdeAttr {
    Flatten,
    // We only care about some of the serde attributes
    Unknown,
}

impl Parse for SerdeAttr {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let name_str = name.to_string();

        if input.peek(Token![=]) {
            // `name = value` attributes.
            input.parse::<Token![=]>()?; // skip '='

            if input.peek(LitStr) {
                input.parse::<LitStr>()?;
            } else if let Err(e) = input.parse::<Expr>() {
                panic!("`configopt` parsing `serde` expected `string literal` or `expression` after `=`, err: {}", e)
            }
            Ok(SerdeAttr::Unknown)
        } else if input.peek(syn::token::Paren) {
            // `name(...)` attributes.
            let nested;
            // Even though we do not do anything here we still need to consume the tokens from the ParseStream
            parenthesized!(nested in input);
            nested.parse::<TokenStream>()?;
            Ok(SerdeAttr::Unknown)
        } else {
            // Attributes represented with a sole identifier.
            Ok(match name_str.as_ref() {
                "flatten" => SerdeAttr::Flatten,
                _ => SerdeAttr::Unknown,
            })
        }
    }
}

pub fn parse_attrs(attrs: &[Attribute]) -> Vec<SerdeAttr> {
    attrs
        .iter()
        .filter(|attr| attr.path.is_ident("serde"))
        .flat_map(|attr| {
            attr.parse_args_with(Punctuated::<SerdeAttr, Token![,]>::parse_terminated)
                .expect("`configopt` failed to parse `serde` attributes")
        })
        .collect()
}
