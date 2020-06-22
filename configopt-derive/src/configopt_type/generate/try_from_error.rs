use proc_macro2::TokenStream;
use proc_macro_roids::IdentExt;
use quote::quote;
use syn::Ident;

// TODO: use this more sophisticated error type
#[allow(dead_code)]
pub fn generate(ident: &Ident) -> (Ident, TokenStream) {
    let error_ident = ident.prepend("TryFrom").append("Error"); // TryFrom#configopt_identError
    let ident_str = ident.to_string();
    let token_stream = quote! {
        struct #error_ident {
            configopt: #ident,
            missing_field: String,
        }

        impl #error_ident {
            fn new(configopt: #ident, missing_field: &str) -> Self {
                Self {
                    configopt,
                    missing_field: String::from(missing_field),
                }
            }
        }

        impl ::std::fmt::Debug for #error_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(#ident_str)
                    .field("missing_field", &self.missing_field)
                    .finish()
            }
        }


        impl ::std::fmt::Display for #error_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "failed converting `{}` to base type missing field '{}'", #ident_str, self.missing_field)
            }
        }

        impl ::std::error::Error for #error_ident {}
    };
    (error_ident, token_stream)
}
