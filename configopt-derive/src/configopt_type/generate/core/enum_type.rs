use crate::configopt_type::parse::{FieldType, ParsedVariant};
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn is_complete(variants: &[ParsedVariant]) -> TokenStream {
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
                FieldType::Named => {
                    quote! {
                        // TODO
                    }
                }
            }
        })
        .collect()
}

pub(crate) fn is_convertible(variants: &[ParsedVariant]) -> TokenStream {
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
                FieldType::Named => {
                    quote! {
                        // TODO
                    }
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
                FieldType::Named => {
                    quote! {
                        // TODO
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
                FieldType::Named => {
                    quote! {
                        // TODO
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
                FieldType::Named => {
                    quote! {
                        // TODO
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
                FieldType::Named => {
                    quote! {
                        // TODO
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
                FieldType::Named => {
                    quote! {
                        // TODO
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
                FieldType::Named => {
                    quote! {
                        // TODO
                    }
                }
            }
        })
        .collect()
}
