use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{Attribute, Ident};

use crate::validators::{FieldValidator, MatchesArgs};

pub struct Matches;

impl FieldValidator for Matches {
    fn attr_name(&self) -> &'static str {
        "matches"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<MatchesArgs, darling::Error> = FromMeta::from_meta(&attr.meta);
        match args {
            Ok(args) => {
                if let Err(e) = Regex::new(&args.pattern) {
                    // TODO: Change panic to graceful error handling
                    panic!(
                        "Invalid matches pattern for field '{}': {}",
                        field_name, e
                    );
                }

                let error_code = args
                    .error_code
                    .unwrap_or_else(|| format!("400_{}_MATCHES", field_name.to_uppercase()));
                let error_message = args
                    .error_message
                    .unwrap_or_else(|| format!("{} must match {}", field_name, args.pattern));
                let pattern = args.pattern;

                quote! {
                    if !::pass_me::Matches::is_matches(&self.#field_ident, &#pattern) {
                        errors.push(::pass_me::ValidationError {
                            field: #field_name,
                            error_message: #error_message.to_string(),
                            code: #error_code.to_string(),
                        });
                    }
                }
            }
            Err(e) => {
                // TODO: Change panic to graceful error handling
                panic!("Failed to parse matches attributes: {}", e.to_string());
            }
        }
    }
}
