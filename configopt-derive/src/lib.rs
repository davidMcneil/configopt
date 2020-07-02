extern crate proc_macro;

mod configopt_type;

use configopt_type::generate;
use configopt_type::ConfigOptConstruct;
use proc_macro_roids::FieldsNamedAppend;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

#[proc_macro_derive(ConfigOpt, attributes(configopt))]
pub fn configopt_derive(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as DeriveInput);

    let (configopt_type, construct) = ConfigOptConstruct::convert_and_parse(ast);
    let expanded = construct.expand();
    let lints = generate::lints();

    let expanded = quote! {
        #lints
        #configopt_type

        #expanded
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn configopt_fields(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    use crate::configopt_type::parse::configopt_fields_attr_parser::{
        self as attr_parser, ConfigOptFieldsAttr,
    };

    let attrs = attr_parser::parse(attr);
    let hidden = attrs
        .into_iter()
        .find_map(|a| match a {
            ConfigOptFieldsAttr::Hidden(expr) => Some(expr),
        })
        .unwrap_or_else(|| parse_quote! {false});
    let mut ast = parse_macro_input!(item as DeriveInput);

    let additional_fields = parse_quote!({
        /// Paths to config files to read
        #[structopt(long = "config-files", hidden = #hidden)]
        #[serde(skip)]
        config_files: Vec<::std::path::PathBuf>,
        /// Generate a TOML config
        #[structopt(long = "generate-config", hidden = #hidden)]
        #[serde(skip)]
        generate_config: bool,
    });
    ast.append_named(additional_fields);

    proc_macro::TokenStream::from(quote! {#ast})
}
