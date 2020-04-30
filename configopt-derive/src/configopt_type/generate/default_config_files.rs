use proc_macro2::TokenStream;
use quote::quote;

pub fn generate(maybe_default_config_file: Option<&str>) -> TokenStream {
    if let Some(default_config_file) = maybe_default_config_file {
        quote! {
            /// Get the default config files
            pub fn default_config_files() -> Vec<::std::path::PathBuf> {
                vec![::std::path::PathBuf::from(#default_config_file)]
            }
        }
    } else {
        quote! {
            /// Get the default config files
            pub fn default_config_files() -> Vec<::std::path::PathBuf> {
                Vec::new()
            }
        }
    }
}
