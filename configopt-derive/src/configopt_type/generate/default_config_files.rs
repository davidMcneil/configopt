use proc_macro2::TokenStream;
use quote::quote;
use syn::{NestedMeta, Path};

pub enum Attribute {
    Lit(String),
    Path(Path),
}

impl From<NestedMeta> for Attribute {
    fn from(m: NestedMeta) -> Self {
        match m {
            syn::NestedMeta::Lit(syn::Lit::Str(m)) => Self::Lit(m.value()),
            syn::NestedMeta::Meta(syn::Meta::Path(path)) => Self::Path(path),
            _ => panic!("`configopt(default_config_file)` expected string literal or path"),
        }
    }
}

pub fn generate(attribute: Option<&Attribute>) -> TokenStream {
    let default_config_files = if let Some(attribute) = attribute {
        match attribute {
            Attribute::Lit(lit) => {
                quote! {
                    /// Get the default config files
                    pub fn default_config_files() -> Vec<::std::path::PathBuf> {
                        vec![::std::path::PathBuf::from(#lit)]
                    }
                }
            }
            Attribute::Path(path) => {
                quote! {
                    /// Get the default config files
                    pub fn default_config_files() -> Vec<::std::path::PathBuf> {
                        #path()
                    }
                }
            }
        }
    } else {
        quote! {
            /// Get the default config files
            pub fn default_config_files() -> Vec<::std::path::PathBuf> {
                Vec::new()
            }
        }
    };
    quote! {
        #default_config_files

        pub fn from_default_config_files() -> ::std::result::Result<Self, ::configopt::Error> {
            use ::std::convert::TryFrom;
            Self::try_from(Self::default_config_files().as_slice())
        }
    }
}
