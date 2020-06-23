macro_rules! attribute_trimmer {
    ($path:expr, $fields_to_trim:expr) => {
        fn trim_fields(input: ParseStream) -> syn::Result<TokenStream> {
            let name: Ident = input.parse()?;
            let name_str = name.to_string();
            let should_trim = $fields_to_trim.contains(&name_str.as_str());

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
                            panic!("`configopt` parsing trimmer expected `string literal` or `expression` after `=`, err: {}", e)
                        }
                    }
                };
                if should_trim {
                    quote! {}
                } else {
                    quote! {#name = #token_stream}
                }
            } else if input.peek(syn::token::Paren) {
                // `name(...)` attributes.
                let nested;
                parenthesized!(nested in input);
                let token_stream: TokenStream = nested.parse()?;
                if should_trim {
                    quote! {}
                } else {
                    quote! {#name(#token_stream)}
                }
            } else {
                // Attributes represented with a sole identifier.
                if should_trim {
                    quote! {}
                } else {
                    quote! {#name}
                }
            })
        }

        fn trimmer(input: ParseStream) -> syn::Result<Punctuated<TokenStream, Token![,]>> {
            Ok(input
                .parse_terminated::<_, Token![,]>(trim_fields)?
                .into_iter()
                .filter(|p| !p.is_empty())
                .collect())
        }

        pub fn trim_attr(attr: &mut Attribute) {
            if !attr.path.is_ident($path) {
                return;
            }
            let tokens = attr
                .parse_args_with(trimmer)
                .expect("`ConfigOpt` failed to trim attributes");
            attr.tokens = quote! {(#tokens)};
        }
    }
}
