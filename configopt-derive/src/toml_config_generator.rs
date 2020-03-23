use super::util::ParsedField;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub fn config(fields: &[ParsedField]) -> TokenStream {
    let field_tokens = fields.iter().map(|field| {
        let ident = field.ident();
        let span = field.span();
        let serde_name = field.serde_name();
        if field.nested() {
            let partial_ty = field.partial_inner_ty();
            quote_spanned! {span=>
                let default = &#partial_ty::default();
                let val = &self.#ident.as_ref().unwrap_or(default);
                let mut new_prefix = serde_prefix.to_vec();
                new_prefix.push(String::from(#serde_name));
                result = format!("{}{}", result, val.toml_config_with_prefix(&new_prefix));
            }
        } else {
            let structopt_name = field.structopt_name();
            quote_spanned! {span=>
                let key = if serde_prefix.len() == 0 {
                    String::from(#serde_name)
                } else {
                    format!("{}.{}", serde_prefix.join("."), #serde_name)
                };
                // Pull out the comment from the clap::App
                let mut comment = String::new();
                for arg in &app.p.flags {
                    let b = &arg.b;
                    if !b.is_set(::structopt::clap::ArgSettings::Hidden) && #structopt_name == b.name {
                        comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                        break;
                    }
                }
                if comment.is_empty() {
                    for arg in &app.p.opts {
                        let b = &arg.b;
                        if !b.is_set(::structopt::clap::ArgSettings::Hidden) && #structopt_name == b.name {
                            comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                            break;
                        }
                    }
                }
                if comment.is_empty() {
                    for (_, arg) in &app.p.positionals {
                        let b = &arg.b;
                        if !b.is_set(::structopt::clap::ArgSettings::Hidden) && #structopt_name == b.name {
                            comment = String::from(b.long_help.unwrap_or_else(|| b.help.unwrap_or("")));
                            break;
                        }
                    }
                }
                if !comment.is_empty() {
                    comment = comment.lines().map(|l| format!("# {}\n", l)).collect::<String>();
                }
                match toml::Value::try_from(&self.#ident) {
                    Ok(val) => {
                        result = format!("{}{}{} = {}\n\n", result, comment, key, val);
                    }
                    Err(toml::ser::Error::UnsupportedNone) => {
                        result = format!("{}{}## {} =\n\n", result, comment, key);
                    }
                    _ => {}
                }
            }
        }
    });
    quote! {
        let mut result = String::new();
        #(#field_tokens)*
        result
    }
}
