use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

use crate::validators::{FieldValidator, MaxLengthArgs};

pub struct MaxLength;

impl FieldValidator for MaxLength {
    fn attr_name(&self) -> &'static str {
        "max_length"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<MaxLengthArgs, darling::Error> = FromMeta::from_meta(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args
                    .error_code
                    .unwrap_or_else(|| format!("400_{}_MAX_LENGTH", field_name.to_uppercase()));
                let error_message = args.error_message.unwrap_or_else(|| {
                    format!("{} must be at most {} characters", field_name, args.max)
                });

                let max = args.max;

                quote! {
                    if !::pass_me::Length::is_length(&self.#field_ident, 0, #max) {
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
                panic!("Failed to parse max_length attributes: {}", e.to_string());
            }
        }
    }
}
