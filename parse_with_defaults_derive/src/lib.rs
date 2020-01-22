#![feature(log_syntax)]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ParseWithDefaults)]
pub fn parse_with_defaults(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let partial_ident = format_ident!("Partial{}", ident);
    let partial_data = partial_data(&input.data);
    let partial_merge = partial_merge(&input.data);
    let partial_from = partial_from(&input.data);
    let stringy_ident = format_ident!("Stringy{}", ident);
    let stringy_data = stringy_data(&input.data);
    let stringy_from = stringy_from(&input.data);
    let stringy_populate_app_defaults = stringy_populate_app_defaults(&input.data);
    let expanded = quote! {
        #[derive(Debug, Default, Deserialize, Serialize)]
        struct #partial_ident {
            #partial_data
        }

        impl #partial_ident {
            fn merge(self, other: &mut #ident) {
                #partial_merge
            }
        }

        impl ::std::convert::From<#ident> for #partial_ident {
            fn from(other: #ident) -> Self {
                Self {
                    #partial_from
                }
            }
        }

        #[derive(Debug, Default)]
        struct #stringy_ident {
            #stringy_data
        }

        impl ::std::convert::From<&#partial_ident> for #stringy_ident {
            fn from(other: &#partial_ident) -> Self {
                Self {
                    #stringy_from
                }
            }
        }

        impl #stringy_ident {
            fn populate_app_defaults<'a>(&'a self, mut app: App<'a>, matches: &ArgMatches) -> App<'a> {
                #stringy_populate_app_defaults
                app
            }
        }

        impl #ident {
            fn parse_with_defaults(defaults: &#partial_ident) -> #ident {
                let mut app = #ident::into_app();
                let args = app.args.args.iter().cloned().collect::<Vec<_>>();

                // Make an app where all args are partial
                let mut partial_app = app.clone();
                for arg in &args {
                    partial_app =
                        partial_app.mut_arg(arg.name, |arg| arg.unset_setting(ArgSettings::Required));
                }

                let stringy_defaults = #stringy_ident::from(defaults);

                // Check which options have a default value due to passed in defaults
                let args = env::args()
                    .filter(|a| a != "-h" && a != "--h")
                    .collect::<Vec<_>>();
                let matches = partial_app.get_matches_from(args);

                // Populate app with the defaults and generate an #ident form the matches
                app = stringy_defaults.populate_app_defaults(app, &matches);
                let matches = app.get_matches();
                #ident::from_argmatches(&matches)
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn partial_data(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    let ty = &f.ty;
                    quote_spanned! {f.span()=>
                        #ident: Option<#ty>
                    }
                });
                quote! {
                    #(#recurse,)*
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    }
}

fn partial_merge(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote_spanned! {f.span()=>
                        if let Some(val) = self.#ident {
                            other.#ident = val;
                        }
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    }
}

fn partial_from(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote_spanned! {f.span()=>
                        #ident: Some(other.#ident)
                    }
                });
                quote! {
                    #(#recurse,)*
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    }
}

fn stringy_data(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote_spanned! {f.span()=>
                        #ident: Option<String>
                    }
                });
                quote! {
                    #(#recurse,)*
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    }
}

fn stringy_from(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote_spanned! {f.span()=>
                        #ident: other.#ident.as_ref().map(|v| v.to_string())
                    }
                });
                quote! {
                    #(#recurse,)*
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    }
}

fn stringy_populate_app_defaults(data: &Data) -> TokenStream {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let ident = &f.ident;
                    quote_spanned! {f.span()=>
                        if let Some(v) = &self.#ident {
                            if !matches.is_present(stringify!(#ident)) {
                                app = app.mut_arg(stringify!(#ident), |arg| {
                                    arg.default_value(v)
                                });
                            }
                        }
                    }
                });
                quote! {
                    #(#recurse)*
                }
            }
            Fields::Unnamed(_) => unimplemented!(),
            Fields::Unit => unimplemented!(),
        },
        Data::Enum(_) => unimplemented!(),
        Data::Union(_) => unimplemented!(),
    }
}
