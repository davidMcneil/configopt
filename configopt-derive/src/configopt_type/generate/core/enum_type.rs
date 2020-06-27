use super::struct_type;
use crate::configopt_type::parse::{FieldType, ParsedField, ParsedVariant};
use proc_macro2::TokenStream;
use proc_macro_roids::IdentExt;
use quote::quote;
use syn::{punctuated::Punctuated, Token};

fn comma_separated_fields(
    prefix: &str,
    fields: &[ParsedField],
    mutable: bool,
) -> Punctuated<TokenStream, Token![,]> {
    fields
        .iter()
        .map(|f| {
            let ident = f.ident();
            if prefix.is_empty() {
                if mutable {
                    quote! {ref mut #ident}
                } else {
                    quote! {#ident}
                }
            } else {
                let prefixed_ident = f.ident().prepend(prefix);
                if mutable {
                    quote! {#ident: ref mut #prefixed_ident}
                } else {
                    quote! {#ident: #prefixed_ident}
                }
            }
        })
        .collect()
}

pub(crate) fn patch(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        (#full_configopt_ident(self_variant), #full_configopt_ident(other_variant)) => {
                            self_variant.patch(other_variant);
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        (#full_configopt_ident, #full_configopt_ident) => {}
                    }
                }
                FieldType::Named(fields) => {
                    let fields = fields.as_slice();
                    let self_fields_match =
                        comma_separated_fields("self_", fields, true);
                    let other_fields_match =
                        comma_separated_fields("other_", fields, true);
                    let inner =
                        struct_type::patch_with_prefix("self_", "other_", true, fields);
                    quote! {
                        (#full_configopt_ident{#self_fields_match}, #full_configopt_ident{#other_fields_match}) => {
                            #inner
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn take(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        (#full_configopt_ident(self_variant), #full_configopt_ident(other_variant)) => {
                            self_variant.take(other_variant);
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        (#full_configopt_ident, #full_configopt_ident) => {}
                    }
                }
                FieldType::Named(fields) => {
                    let fields = fields.as_slice();
                    let self_fields_match =
                        comma_separated_fields("self_", fields, true);
                    let other_fields_match =
                        comma_separated_fields("other_", fields, true);
                    let inner =
                        struct_type::take_with_prefix("self_", "other_", true, fields);
                    quote! {
                        (#full_configopt_ident{#self_fields_match}, #full_configopt_ident{#other_fields_match}) => {
                            #inner
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn patch_for(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            let full_ident = variant.full_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        (#full_configopt_ident(self_variant), #full_ident(other_variant)) => {
                            self_variant.patch_for(other_variant);
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        (#full_configopt_ident, #full_ident) => {}
                    }
                }
                FieldType::Named(fields) => {
                    let fields = fields.as_slice();
                    let self_fields_match =
                        comma_separated_fields("self_", fields, true);
                    let other_fields_match =
                        comma_separated_fields("other_", fields, true);
                    let inner =
                        struct_type::patch_for_with_prefix("self_", "other_", true, fields);
                    quote! {
                        (#full_configopt_ident{#self_fields_match}, #full_ident{#other_fields_match}) => {
                            #inner
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn take_for(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            let full_ident = variant.full_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        (#full_configopt_ident(self_variant), #full_ident(other_variant)) => {
                            self_variant.take_for(other_variant);
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        (#full_configopt_ident, #full_ident) => {}
                    }
                }
                FieldType::Named(fields) => {
                    let fields = fields.as_slice();
                    let self_fields_match =
                        comma_separated_fields("self_", fields, true);
                    let other_fields_match =
                        comma_separated_fields("other_", fields, true);
                    let inner =
                        struct_type::patch_for_with_prefix("self_", "other_", true, fields);
                    quote! {
                        (#full_configopt_ident{#self_fields_match}, #full_ident{#other_fields_match}) => {
                            #inner
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn is_empty(variants: &[ParsedVariant]) -> TokenStream {
    // Handle the case of an empty enum
    if variants.is_empty() {
        return quote! {
            _ => true
        };
    }
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        #full_configopt_ident(inner) => {
                            inner.is_empty()
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        #full_configopt_ident => {
                            false
                        }
                    }
                }
                FieldType::Named(fields) => {
                    let fields = fields.as_slice();
                    let fields_match = comma_separated_fields("", fields, false);
                    let inner = struct_type::is_empty_with_prefix("", fields);
                    quote! {
                        #full_configopt_ident {#fields_match} => {
                            #inner
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn is_complete(variants: &[ParsedVariant]) -> TokenStream {
    // Handle the case of an empty enum
    if variants.is_empty() {
        return quote! {
            _ => true
        };
    }
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        #full_configopt_ident(inner) => {
                            inner.is_complete()
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        #full_configopt_ident => {
                            true
                        }
                    }
                }
                FieldType::Named(fields) => {
                    let fields = fields.as_slice();
                    let fields_match = comma_separated_fields("", fields, false);
                    let inner = struct_type::is_complete_with_prefix("", fields);
                    quote! {
                        #full_configopt_ident {#fields_match} => {
                            #inner
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn is_convertible(variants: &[ParsedVariant]) -> TokenStream {
    if variants.is_empty() {
        return quote! {
            _ => true
        };
    }
    variants
        .iter()
        .map(|variant| {
            let full_configopt_ident = variant.full_configopt_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        (#full_configopt_ident(inner)) => {
                            inner.is_convertible()
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        #full_configopt_ident => {
                            true
                        }
                    }
                }
                FieldType::Named(fields) => {
                    let fields = fields.as_slice();
                    let fields_match = comma_separated_fields("", fields, false);
                    let inner = struct_type::is_convertible_with_prefix("", fields);
                    quote! {
                        #full_configopt_ident {#fields_match} => {
                            #inner
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn from(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| {
            let full_ident = variant.full_ident();
            let full_configopt_ident = variant.full_configopt_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        #full_ident(inner) => {
                            #full_configopt_ident(inner.into())
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        #full_ident => {
                            #full_configopt_ident
                        }
                    }
                }
                FieldType::Named(_) => {
                    quote! {
                        #full_ident {..} => {
                            todo!()
                        }
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn try_from(variants: &[ParsedVariant]) -> TokenStream {
    variants
        .iter()
        .map(|variant| {
            let full_ident = variant.full_ident();
            let full_configopt_ident = variant.full_configopt_ident();
            match variant.field_type() {
                FieldType::Unnamed => {
                    quote! {
                        #full_configopt_ident(inner) => {
                            Ok(#full_ident(inner.try_into().ok().unwrap()))
                        }
                    }
                }
                FieldType::Unit => {
                    quote! {
                        #full_configopt_ident => {
                            Ok(#full_ident)
                        }
                    }
                }
                FieldType::Named(_) => {
                    quote! {
                        #full_configopt_ident {..} => {
                            todo!()
                        }
                    }
                }
            }
        })
        .collect()
}
