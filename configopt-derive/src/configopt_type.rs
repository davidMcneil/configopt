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
    Struct(Ident, Option<String>, Vec<ParsedField>),
    Enum(Ident, Vec<ParsedVariant>),
}

impl ConfigOptConstruct {
    pub fn convert_and_parse(original_type: DeriveInput) -> (DeriveInput, ConfigOptConstruct) {
        let ident = original_type.ident.clone();
        let mut configopt_type = original_type;

        // Change the ident to a configopt ident
        configopt_type.ident = parse::configopt_ident(&configopt_type.ident);

        // Check if we have a default config file
        let default_config_file = if let Some(default_config_file) = configopt_type
            .tag_parameter(&parse_quote!(configopt), &parse_quote!(default_config_file))
        {
            match default_config_file {
                syn::NestedMeta::Lit(syn::Lit::Str(default_config_file)) => {
                    Some(default_config_file.value())
                }
                _ => panic!("`configopt(default_config_file)` expected string literal"),
            }
        } else {
            None
        };

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

        retain_attrs(&mut configopt_type.attrs, &retained_attrs);

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
                                convert_and_parse_field(
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
        // TODO: Remove this requirement
        derives.push(parse_quote! {Deserialize});
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

                let configopt_take = struct_type::take(&parsed_fields, &other);
                let configopt_patch = struct_type::patch(&parsed_fields, &other);
                let configopt_take_for = struct_type::take_for(&parsed_fields, &other);
                let configopt_patch_for = struct_type::patch_for(&parsed_fields, &other);
                let configopt_is_empty = struct_type::is_empty(&parsed_fields);
                let configopt_is_complete = struct_type::is_complete(&parsed_fields);
                let configopt_is_convertible = struct_type::is_convertible(&parsed_fields);
                let configopt_from = struct_type::from(&parsed_fields, &other);
                let configopt_try_from = struct_type::try_from(&parsed_fields);
                let default_config_files =
                    generate::default_config_files::generate(default_config_file.as_deref());
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
                            other.patch_for(self);
                        }
                    }
                }
            }
            Self::Enum(_, parsed_variants) => {
                use generate::core::enum_type;

                let configopt_take = enum_type::take(&parsed_variants);
                let configopt_patch = enum_type::patch(&parsed_variants);
                let configopt_take_for = enum_type::take_for(&parsed_variants);
                let configopt_patch_for = enum_type::patch_for(&parsed_variants);
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
                                _ => {
                                    panic!("TODO: `take` for enum is not fully implemented");
                                }
                            }
                        }

                        /// For each field in `self` if it is `None`, take the value from `other` and set it in `self`
                        pub fn patch(&mut self, other: &mut #configopt_ident) {
                            match (self, other) {
                                #configopt_patch
                                _ => {
                                    panic!("TODO: `patch` for enum is not fully implemented");
                                }
                            }
                        }

                        /// Take each field from `self` and set it in `other`
                        pub fn take_for(&mut self, other: &mut #ident) {
                            match (self, other) {
                                #configopt_take_for
                                _ => {
                                    panic!("TODO: `take_for` for enum is not fully implemented");
                                }
                            }
                        }

                        /// For each field in `other` if it is `None`, take the value from `self` and set it in `other`
                        pub fn patch_for(&mut self, other: &mut #ident) {
                            match (self, other) {
                                #configopt_patch_for
                                _ => {
                                    panic!("TODO: `patch_for` for enum is not fully implemented");
                                }
                            }
                        }

                        /// Check if all fields of `self` are `Some` applied recursively
                        pub fn is_complete(&self) -> bool {
                            match self {
                                #configopt_is_complete
                                _ => {
                                    panic!("TODO: `is_complete` for enum is not fully implemented");
                                }
                            }
                        }

                        /// Check if `self` can be converted into a full version
                        pub fn is_convertible(&self) -> bool {
                            match self {
                                #configopt_is_convertible
                                _ => {
                                    panic!("TODO: `is_convertible` for enum is not fully implemented");
                                }
                            }
                        }
                    }

                    #lints
                    impl ::std::convert::From<#ident> for #configopt_ident {
                        fn from(other: #ident) -> Self {
                            match other {
                                #configopt_from
                                _ => {
                                    panic!("TODO: `from` for enum is not fully implemented");
                                }
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
                                _ => {
                                    panic!("TODO: `try_from` for enum is not fully implemented");
                                }
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
                            other.patch_for(self);
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

// Only retain attributes we have explicitly opted to preserve
fn retain_attrs(attrs: &mut Vec<Attribute>, retained_attrs: &[Ident]) {
    attrs.retain(|a| retained_attrs.iter().any(|i| a.path.is_ident(i)));
    for attr in attrs {
        parse::trim_structopt_attr(attr);
        parse::trim_serde_attr(attr);
    }
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
    if parsed_field.structopt_flatten() || parsed_field.subcommand() {
        *parse::inner_ty(ty) = parsed_field.configopt_inner_ty().clone();
    }

    retain_attrs(&mut field.attrs, &retained_attrs);

    let structopt_ty = parsed_field.structopt_ty();

    // If this field was a `Vec` we need to add a default value to allow deserializing the
    // `ConfigOpt` type from an empty input.
    if let StructOptTy::Vec = structopt_ty {
        if retained_attrs.iter().any(|a| a == "serde") {
            field.attrs.push(parse_quote! {#[serde(default)]})
        }
    }

    // If the field is not already, wrap its type in an `Option`. This guarantees that the
    // `ConfigOpt` struct can be parsed regardless of complete CLI input.
    if let StructOptTy::Bool | StructOptTy::Other = structopt_ty {
        // If it was a flattened field all of its fields will be optional so it does not need to
        // be wrapped in an `Option`
        if !parsed_field.structopt_flatten() {
            field.ty = parse_quote!(Option<#ty>);
        }
        // If this field was a `bool` we need to add a default of `true` now that it is wrapped in
        // an `Option`. This preserves the same behavior as if we just had a `bool`, but allows us
        // to detect if the `bool` even has a value. Essentially, it adds a third state of not set
        // (None) to this field.
        if let StructOptTy::Bool = parsed_field.structopt_ty() {
            field
                .attrs
                .push(parse_quote! {#[structopt(default_value = "true")]})
        }
    }

    parsed_field
}
