use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

use crate::validators::{Args, FieldValidator};

pub struct NotNullOrEmpty;

impl FieldValidator for NotNullOrEmpty {
    fn attr_name(&self) -> &'static str {
        "non_null_or_empty"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<Args, darling::Error> = FromMeta::from_meta(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args.error_code.unwrap_or_else(|| {
                    format!("400_{}_NOT_NULL_OR_EMPTY", field_name.to_uppercase())
                });
                let error_message = args
                    .error_message
                    .unwrap_or_else(|| format!("{} cannot be null or empty", field_name));

                quote! {
                    if !::pass_me::NotNullOrEmpty::is_not_empty(&self.#field_ident) {
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
                panic!(
                    "Failed to parse non_null_or_empty attributes: {}",
                    e.to_string()
                );
            }
        }
    }
}
