use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

use crate::validators::{parse_meta_args, Args, FieldValidator};

pub struct Must;

impl FieldValidator for Must {
    fn attr_name(&self) -> &'static str {
        "must"
    }

    fn is_stateful(&self) -> bool {
        true
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<Args, darling::Error> = parse_meta_args(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args
                    .error_code
                    .unwrap_or_else(|| format!("400_{}_MUST", field_name.to_uppercase()));
                let error_message = args
                    .error_message
                    .unwrap_or_else(|| format!("{} must be unique", field_name));

                quote! {
                    if !checker.must(#field_name, &self.#field_ident).await {
                        errors.push(::pass_me::ValidationError { field: #field_name, error_message: #error_message.to_string(), code: #error_code.to_string() });
                    }
                }
            }
            Err(e) => {
                // TODO: Change panic to graceful error handling
                panic!("Failed to parse must attributes: {}", e.to_string());
            }
        }
    }
}
