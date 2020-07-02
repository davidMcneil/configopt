pub mod generate;
pub mod parse;

use generate::default_config_files::Attribute as DefaultConfigFilesAttribute;
use parse::{CasingStyle, ParsedField, ParsedVariant};
use proc_macro2::TokenStream;
use proc_macro_roids::DeriveInputExt;
use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Data, DeriveInput, Fields, Ident, Token};

pub enum ConfigOptConstruct {
    Struct(Ident, Option<DefaultConfigFilesAttribute>, Vec<ParsedField>),
    Enum(Ident, Vec<ParsedVariant>),
}

impl ConfigOptConstruct {
    pub fn convert_and_parse(original_type: DeriveInput) -> (DeriveInput, ConfigOptConstruct) {
        let ident = original_type.ident.clone();
        let mut configopt_type = original_type;

        // Change the ident to a configopt ident
        configopt_type.ident = parse::configopt_ident(&configopt_type.ident);

        // Check if we have a default config file
        let default_config_file = configopt_type
            .tag_parameter(&parse_quote!(configopt), &parse_quote!(default_config_file))
            .map(|a| a.into());

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

        // We implicitly retain these attributes
        retained_attrs.push(parse_quote! {structopt});

        // Get the derives for the configopt type
        let mut derives = configopt_type
            .tag_parameters(&parse_quote!(configopt), &parse_quote!(derive))
            .into_iter()
            .collect::<Punctuated<_, Token![,]>>();

        parse::retain_attrs(&mut configopt_type.attrs, &retained_attrs);

        // Determine the global rename casing style for structopt and serde
        let structopt_rename = parse::structopt_rename_all(&configopt_type.attrs)
            // Structopt defaults to kebab case if no `rename_all` attribute is specified
            .unwrap_or(CasingStyle::Kebab);
        // TODO: Actually lookup the serde name
        let serde_rename = CasingStyle::Verbatim;

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
                                ParsedField::new(
                                    field,
                                    structopt_rename,
                                    serde_rename,
                                    &retained_attrs,
                                )
                            })
                            .collect::<Vec<_>>();
                        ConfigOptConstruct::Struct(ident, default_config_file, parsed_fields)
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
                    parse::retain_attrs(&mut variant.attrs, &retained_attrs);
                    parsed_variants.push(ParsedVariant::new(
                        &ident,
                        variant,
                        structopt_rename,
                        serde_rename,
                        &retained_attrs,
                    ));
                }
                ConfigOptConstruct::Enum(ident, parsed_variants)
            }
            Data::Union(_) => panic!("`ConfigOpt` cannot be derived for unions"),
        };

        // Add the derives
        derives.push(parse_quote! {StructOpt});
        // TODO: Remove this requirement
        derives.push(parse_quote! {serde::Deserialize});
        configopt_type.append_derives(derives);

        (configopt_type, configopt_construct)
    }

    pub fn expand(&self) -> TokenStream {
        let lints = generate::lints();
        let ident = self.ident();
        let other = parse_quote! {other};
        let configopt_ident = parse::configopt_ident(ident);
        match self {
            Self::Struct(_, default_config_file, parsed_fields) => {
                use generate::core::struct_type;

                let configopt_patch = struct_type::patch(&parsed_fields);
                let configopt_take = struct_type::take(&parsed_fields);
                let configopt_patch_for = struct_type::patch_for(&parsed_fields);
                let configopt_take_for = struct_type::take_for(&parsed_fields);
                let configopt_is_empty = struct_type::is_empty(&parsed_fields);
                let configopt_is_complete = struct_type::is_complete(&parsed_fields);
                let configopt_is_convertible = struct_type::is_convertible(&parsed_fields);
                let configopt_from = struct_type::from(&parsed_fields, &other);
                let configopt_try_from = struct_type::try_from(&parsed_fields);
                let default_config_files =
                    generate::default_config_files::generate(default_config_file.as_ref());
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

                        /// Take each field from `self` and set it in `other`
                        pub fn take_for(&mut self, other: &mut #ident) {
                            #configopt_take_for
                        }

                        /// For each field in `other` if it is `None`, take the value from `self` and set it in `other`
                        pub fn patch_for(&mut self, other: &mut #ident) {
                            #configopt_patch_for
                        }

                        /// Check if all fields of `self` are `None`
                        #[allow(clippy::eq_op)]
                        pub fn is_empty(&self) -> bool {
                            #configopt_is_empty
                        }

                        /// Check if all fields of `self` are `Some` applied recursively
                        #[allow(clippy::eq_op)]
                        pub fn is_complete(&self) -> bool {
                            #configopt_is_complete
                        }

                        /// Check if `self` can be converted into a full version
                        #[allow(clippy::eq_op)]
                        pub fn is_convertible(&self) -> bool {
                            #configopt_is_convertible
                        }

                        #default_config_files
                    }

                    #lints
                    impl ::std::convert::From<#ident> for #configopt_ident {
                        fn from(other: #ident) -> Self {
                            #configopt_from
                        }
                    }

                    #lints
                    impl ::std::convert::TryFrom<#configopt_ident> for #ident {
                        type Error = #configopt_ident;
                        fn try_from(configopt: #configopt_ident) -> ::std::result::Result<Self, Self::Error> {
                            use ::std::convert::TryInto;

                            if !configopt.is_convertible() {
                                return Err(configopt);
                            }
                            #configopt_try_from
                        }
                    }

                    #lints
                    impl ::std::convert::TryFrom<&::std::path::Path> for #configopt_ident {
                        type Error = ::configopt::Error;

                        fn try_from(path: &::std::path::Path) -> ::std::result::Result<Self, Self::Error> {
                            ::configopt::from_toml_file(path)
                        }
                    }

                    #lints
                    impl<T: ::std::convert::AsRef<::std::path::Path>> ::std::convert::TryFrom<&[T]> for #configopt_ident {
                        type Error = ::configopt::Error;

                        fn try_from(paths: &[T]) -> ::std::result::Result<Self, Self::Error> {
                            let mut result = #configopt_ident::default();
                            for path in paths {
                                match #configopt_ident::try_from(path.as_ref()) {
                                    Ok(mut from_default_config_file) => {
                                        result.take(&mut from_default_config_file);
                                    },
                                    Err(e) if e.config_file_not_found() => {
                                        // If we could not find the config file do nothing.
                                    },
                                    Err(e) => return Err(e),
                                }
                            }
                            Ok(result)
                        }
                    }

                    #lints
                    impl ::configopt::ConfigOptArgToOsString for #configopt_ident {
                        fn arg_to_os_string(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
                            let full_arg_path = arg_path;
                            if let Some((arg_name, arg_path)) = full_arg_path.split_first() {
                                #configopt_defaults_field_match
                            } else {
                                None
                            }
                        }
                    }

                    #lints
                    impl ::configopt::IgnoreHelp for #configopt_ident {}

                    #lints
                    impl ::configopt::ConfigOptType for #configopt_ident {
                        fn maybe_config_file(&self) -> Option<String> {
                            #handle_config_files_generate
                            None
                        }

                        fn patch_with_config_files(&mut self) -> ::configopt::Result<&mut #configopt_ident> {
                            #handle_config_files_patch
                        }

                        fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String {
                            let app = #ident::clap();
                            #toml_config_generator_with_prefix
                        }
                    }

                    #lints
                    impl ::configopt::ConfigOpt for #ident {
                        type ConfigOptType = #configopt_ident;

                        fn patch(&mut self, other: &mut Self::ConfigOptType) {
                            other.patch_for(self);
                        }

                        fn take(&mut self, other: &mut Self::ConfigOptType) {
                            other.take_for(self);
                        }
                    }
                }
            }
            Self::Enum(_, parsed_variants) => {
                use generate::core::enum_type;

                let configopt_patch = enum_type::patch(&parsed_variants);
                let configopt_take = enum_type::take(&parsed_variants);
                let configopt_patch_for = enum_type::patch_for(&parsed_variants);
                let configopt_take_for = enum_type::take_for(&parsed_variants);
                let configopt_is_empty = enum_type::is_empty(&parsed_variants);
                let configopt_is_complete = enum_type::is_complete(&parsed_variants);
                let configopt_is_convertible = enum_type::is_convertible(&parsed_variants);
                let configopt_from = enum_type::from(&parsed_variants);
                let configopt_try_from = enum_type::try_from(&parsed_variants);
                let handle_config_files_generate =
                    generate::handle_config_files::generate_for_enum(parsed_variants);
                let handle_config_files_patch =
                    generate::handle_config_files::patch_for_enum(parsed_variants);
                let configopt_defaults_variant =
                    generate::configopt_defaults::for_enum(&parsed_variants);

                quote! {

                    #lints
                    impl #configopt_ident {
                        /// Take each field from `other` and set it in `self`
                        pub fn take(&mut self, other: &mut #configopt_ident) {
                            match (self, other) {
                                #configopt_take
                                _ => {}
                            }
                        }

                        /// For each field in `self` if it is `None`, take the value from `other` and set it in `self`
                        pub fn patch(&mut self, other: &mut #configopt_ident) {
                            match (self, other) {
                                #configopt_patch
                                _ => {}
                            }
                        }

                        /// Take each field from `self` and set it in `other`
                        pub fn take_for(&mut self, other: &mut #ident) {
                            match (self, other) {
                                #configopt_take_for
                                _ => {}
                            }
                        }

                        /// For each field in `other` if it is `None`, take the value from `self` and set it in `other`
                        pub fn patch_for(&mut self, other: &mut #ident) {
                            match (self, other) {
                                #configopt_patch_for
                                _ => {}
                            }
                        }

                        /// Check if all fields of `self` are `None` applied recursively
                        #[allow(clippy::eq_op)]
                        pub fn is_empty(&self) -> bool {
                            match self {
                                #configopt_is_empty
                            }
                        }

                        /// Check if all fields of `self` are `Some` applied recursively
                        #[allow(clippy::eq_op)]
                        pub fn is_complete(&self) -> bool {
                            match self {
                                #configopt_is_complete
                            }
                        }

                        /// Check if `self` can be converted into a full version
                        #[allow(clippy::eq_op)]
                        pub fn is_convertible(&self) -> bool {
                            match self {
                                #configopt_is_convertible
                            }
                        }
                    }

                    #lints
                    impl ::std::convert::From<#ident> for #configopt_ident {
                        fn from(other: #ident) -> Self {
                            match other {
                                #configopt_from
                            }
                        }
                    }

                    #lints
                    impl ::std::convert::TryFrom<#configopt_ident> for #ident {
                        type Error = #configopt_ident;
                        fn try_from(configopt: #configopt_ident) -> ::std::result::Result<Self, Self::Error> {
                            use ::std::convert::TryInto;

                            if !configopt.is_convertible() {
                                return Err(configopt);
                            }
                            match configopt {
                                #configopt_try_from
                            }
                        }
                    }

                    #lints
                    impl ::configopt::ConfigOptArgToOsString for #configopt_ident {
                        fn arg_to_os_string(&self, arg_path: &[String]) -> Option<::std::ffi::OsString> {
                            match self {
                                #configopt_defaults_variant
                                _ => None,
                            }
                        }
                    }

                    #lints
                    impl ::configopt::IgnoreHelp for #configopt_ident {}

                    #lints
                    impl ::configopt::ConfigOptType for #configopt_ident {
                        fn maybe_config_file(&self) -> Option<String> {
                            match self {
                                #handle_config_files_generate
                                _ => {}
                            }
                            None
                        }


                        fn patch_with_config_files(&mut self) -> ::configopt::Result<&mut #configopt_ident> {
                            match self {
                                #handle_config_files_patch
                                _ => {}
                            }
                            Ok(self)
                        }

                        fn toml_config_with_prefix(&self, serde_prefix: &[String]) -> String {
                            todo!()
                        }
                    }

                    #lints
                    impl ::configopt::ConfigOpt for #ident {
                        type ConfigOptType = #configopt_ident;

                        fn patch(&mut self, other: &mut Self::ConfigOptType) {
                            other.patch_for(self);
                        }

                        fn take(&mut self, other: &mut Self::ConfigOptType) {
                            other.take_for(self);
                        }
                    }
                }
            }
        }
    }

    fn ident(&self) -> &Ident {
        match self {
            Self::Struct(ident, _, _) => ident,
            Self::Enum(ident, _) => ident,
        }
    }
}
