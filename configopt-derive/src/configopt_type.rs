pub mod generate;
pub mod parse;

use parse::{CasingStyle, ParsedField, ParsedVariant, StructOptTy};
use proc_macro2::TokenStream;
use proc_macro_roids::DeriveInputExt;
use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, Attribute, Data, DeriveInput, Field, Fields, Ident, Token,
};

pub enum ConfigOptConstruct {
    Struct(Ident, Vec<ParsedField>),
    Enum(Ident, Vec<ParsedVariant>),
}

impl ConfigOptConstruct {
    pub fn convert_and_parse(original_type: DeriveInput) -> (DeriveInput, ConfigOptConstruct) {
        let ident = original_type.ident.clone();
        let mut configopt_type = original_type;

        // Change the ident to a configopt ident
        configopt_type.ident = parse::configopt_ident(&configopt_type.ident);

        // Get a list of attributes to retain on the configopt type
        let mut retained_attrs = configopt_type
            .tag_parameters(&parse_quote!(configopt), &parse_quote!(attrs))
            .into_iter()
            .map(|meta| {
                proc_macro_roids::nested_meta_to_path(&meta)
                    .expect("#[configopt(attrs(..))] expected a path not a Rust literal")
                    .get_ident()
                    .cloned()
                    .expect("#[configopt(attrs(..))] expected an ident")
            })
            .collect::<Vec<_>>();
        retained_attrs.push(parse_quote! {structopt});

        // Get the derives for the configopt type
        let mut derives = configopt_type
            .tag_parameters(&parse_quote!(configopt), &parse_quote!(derive))
            .into_iter()
            .collect::<Punctuated<_, Token![,]>>();

        retain_attrs(&mut configopt_type.attrs, &retained_attrs);

        // Determine the global rename casing style for structopt and serde
        let structopt_rename = parse::structopt_rename_all(&configopt_type.attrs)
            // Structopt defaults to kebab case if no `rename_all` attribute is specified
            .unwrap_or(CasingStyle::Kebab);
        let serde_rename = CasingStyle::Verbatim; // TODO

        // Make all fields configopt
        let configopt_construct = match &mut configopt_type.data {
            Data::Struct(data) => {
                // Only structs can derive default
                derives.push(parse_quote! {Default});

                match &mut data.fields {
                    Fields::Named(fields) => {
                        let parsed_fields = fields
                            .named
                            .iter_mut()
                            .map(|field| {
                                convert_and_parse_field(
                                    field,
                                    structopt_rename,
                                    serde_rename,
                                    &retained_attrs,
                                )
                            })
                            .collect::<Vec<_>>();
                        ConfigOptConstruct::Struct(ident, parsed_fields)
                    }
                    Fields::Unnamed(_) => {
                        panic!("`ConfigOpt` cannot be derived for unnamed struct")
                    }
                    Fields::Unit => panic!("`ConfigOpt` cannot be derived for unit structs"),
                }
            }
            Data::Enum(data) => {
                let mut parsed_variants = Vec::new();
                for variant in &mut data.variants {
                    retain_attrs(&mut variant.attrs, &retained_attrs);

                    match &mut variant.fields {
                        Fields::Named(fields) => {
                            for field in &mut fields.named {
                                convert_and_parse_field(
                                    field,
                                    structopt_rename,
                                    serde_rename,
                                    &retained_attrs,
                                );
                            }
                        }
                        Fields::Unnamed(fields) => {
                            if fields.unnamed.len() > 1 {
                                panic!("`ConfigOpt` cannot be derived on unnamed enums with a length greater than 1");
                            }
                            // Modify the type with the configopt type prefix
                            let field = &mut fields.unnamed[0];
                            let ty = parse::inner_ty(&mut field.ty);
                            *ty = parse::configopt_ident(ty);
                        }
                        Fields::Unit => {}
                    }
                    parsed_variants.push(ParsedVariant::new(&ident, variant));
                }
                ConfigOptConstruct::Enum(ident, parsed_variants)
            }
            Data::Union(_) => panic!("`ConfigOpt` cannot be derived for unions"),
        };

        // Add the derives
        derives.push(parse_quote! {StructOpt});
        derives.push(parse_quote! {Deserialize}); // TODO: Remove this requirement
        configopt_type.append_derives(derives);

        (configopt_type, configopt_construct)
    }

    pub fn expand(&self) -> TokenStream {
        let lints = generate::lints();
        let ident = self.ident();
        let other = parse_quote! {other};
        let configopt_ident = parse::configopt_ident(ident);
        match self {
            Self::Struct(_, parsed_fields) => {
                let configopt_take = generate::core::take(&parsed_fields, &other);
                let configopt_patch = generate::core::patch(&parsed_fields, &other);
                // let configopt_merge = merge(&parsed_fields);
                // let configopt_clear = clear(&parsed_fields);
                // let configopt_is_empty = is_empty(&parsed_fields);
                // let configopt_is_complete = is_complete(&parsed_fields);
                // let configopt_from = from(&parsed_fields);
                // let configopt_try_from = try_from(&parsed_fields);
                let handle_config_files_generate =
                    generate::handle_config_files::generate_for_struct(parsed_fields.as_slice());
                let handle_config_files_patch = generate::handle_config_files::patch_for_struct(
                    parsed_fields.as_slice(),
                    &configopt_ident,
                );
                let toml_config_generator_with_prefix =
                    generate::toml_config::for_struct(&parsed_fields);
                let configopt_defaults_field_match =
                    generate::configopt_defaults::for_struct(&parsed_fields);
                quote! {
                    #lints
                    impl #configopt_ident {
                        /// Take each field from `other` and set it in `self`
                        pub fn take(&mut self, other: &mut #configopt_ident) {
                            #configopt_take
                        }

                        /// For each field in `self` if it is `None`, take the value from `other` and set it in `self`
                        pub fn patch(&mut self, other: &mut #configopt_ident) {
                            #configopt_patch
                        }

                    //     /// Take each field from `self` and set it in `other`
                    //     pub fn merge(&mut self, other: &mut #ident) {
                    //         #configopt_merge
                    //     }

                    //     /// Clear all fields from `self`
                    //     pub fn clear(&mut self) {
                    //         #configopt_clear
                    //     }

                    //     /// Check if all fields of `self` are `None`
                    //     pub fn is_empty(&self) -> bool {
                    //         #configopt_is_empty
                    //     }

                    //     /// Check if all fields of `self` are `Some` applied recursively
                    //     pub fn is_complete(&self) -> bool {
                    //         #configopt_is_complete
                    //     }

                        pub fn maybe_generate_config_file_and_exit(&mut self) {
                            #handle_config_files_generate
                        }


                        pub fn patch_with_config_files(&mut self) -> std::result::Result<&mut #configopt_ident, ::std::io::Error> {
                            #handle_config_files_patch
                        }
                    }

                    // #lints
                    // impl ::std::convert::From<#ident> for #configopt_ident {
                    //     fn from(other: #ident) -> Self {
                    //         #configopt_from
                    //     }
                    // }

                    // #lints
                    // impl ::std::convert::TryFrom<#configopt_ident> for #ident {
                    //     type Error = #configopt_ident;
                    //     fn try_from(configopt: #configopt_ident) -> Result<Self, Self::Error> {
                    //         #configopt_try_from
                    //     }
                    // }

                    #lints
                    impl ::configopt::ConfigOpt for #configopt_ident {}

                    #lints
                    impl ::configopt::ConfigOpt for #ident {}

                    #lints
                    impl ::configopt::TomlConfigGenerator for #configopt_ident {
                        fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String {
                            use ::structopt::StructOpt;

                            let app = #ident::clap();
                            #toml_config_generator_with_prefix
                        }
                    }

                    #lints
                    impl ::configopt::ConfigOptDefaults for #configopt_ident {
                        fn arg_default(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
                            let previous_arg_path = arg_path;
                            if let Some((arg_name, arg_path)) = previous_arg_path.split_first() {
                                #configopt_defaults_field_match
                            } else {
                                None
                            }
                        }
                    }

                    #lints
                    impl ::std::convert::TryFrom<&::std::path::Path> for #configopt_ident {
                        type Error = ::std::io::Error;

                        fn try_from(path: &::std::path::Path) -> ::std::result::Result<Self, Self::Error> {
                            let contents = ::std::fs::read_to_string(path)?;
                            Ok(::toml::from_str(&contents)?)
                        }
                    }

                    #lints
                    impl<T: ::std::convert::AsRef<::std::path::Path>> ::std::convert::TryFrom<&[T]> for #configopt_ident {
                        type Error = ::std::io::Error;

                        fn try_from(paths: &[T]) -> ::std::result::Result<Self, Self::Error> {
                            let mut result = #configopt_ident::default();
                            for path in paths {
                                let mut other = #configopt_ident::try_from(path.as_ref())?;
                                result.take(&mut other);
                            }
                            Ok(result)
                        }
                    }
                }
            }
            Self::Enum(_, parsed_variants) => {
                let handle_config_files_generate =
                    generate::handle_config_files::generate_for_enum(parsed_variants);
                let handle_config_files_patch =
                    generate::handle_config_files::patch_for_enum(parsed_variants);
                let configopt_defaults_variant =
                    generate::configopt_defaults::for_enum(&parsed_variants);
                quote! {
                    #lints
                    impl #configopt_ident {
                        pub fn maybe_generate_config_file_and_exit(&mut self) {
                            match self {
                                #handle_config_files_generate
                                _ => {}
                            }
                        }


                        pub fn patch_with_config_files(&mut self) -> std::result::Result<&mut #configopt_ident, ::std::io::Error> {
                            match self {
                                #handle_config_files_patch
                                _ => {}
                            }
                            Ok(self)
                        }
                    }

                    #lints
                    impl ::configopt::ConfigOpt for #configopt_ident {}

                    #lints
                    impl ::configopt::ConfigOpt for #ident {}

                    #lints
                    impl ::configopt::ConfigOptDefaults for #configopt_ident {
                        fn arg_default(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
                            match self {
                                #configopt_defaults_variant
                                _ => None,
                            }
                        }
                    }
                }
            }
        }
    }

    fn ident(&self) -> &Ident {
        match self {
            Self::Struct(ident, _) => ident,
            Self::Enum(ident, _) => ident,
        }
    }
}

// Only retain attributes we have explicitly opted to preserve
fn retain_attrs(attrs: &mut Vec<Attribute>, retained_attrs: &[Ident]) {
    attrs.retain(|a| retained_attrs.iter().any(|i| a.path.is_ident(i)));
}

fn convert_and_parse_field(
    field: &mut Field,
    structopt_rename: CasingStyle,
    serde_rename: CasingStyle,
    retained_attrs: &[Ident],
) -> ParsedField {
    let parsed_field = ParsedField::new(field, structopt_rename, serde_rename);

    let ty = &mut field.ty;

    // If the field is flattened or a subcommand, modify the type with the configopt type prefix
    if parsed_field.flatten() || parsed_field.subcommand() {
        *parse::inner_ty(ty) = parsed_field.configopt_inner_ty().clone();
    }

    retain_attrs(&mut field.attrs, &retained_attrs);

    // If the field is not already, wrap its type in an `Option`. This guarantees that the
    // `ConfigOpt` struct can be parsed regardless of complete CLI input.
    if let StructOptTy::Bool | StructOptTy::Other = parsed_field.structopt_ty() {
        // If it was a flattened field all of its fields will be optional so it does not need to
        // be wrapped in an `Option`
        if !parsed_field.flatten() {
            field.ty = parse_quote!(Option<#ty>);
        }
    }

    parsed_field
}
