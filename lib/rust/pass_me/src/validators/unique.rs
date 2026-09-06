use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

use crate::validators::{Args, FieldValidator};

pub struct Unique;

impl FieldValidator for Unique {
    fn attr_name(&self) -> &'static str {
        "unique"
    }

    fn is_stateful(&self) -> bool {
        true
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<Args, darling::Error> = FromMeta::from_meta(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args
                    .error_code
                    .unwrap_or_else(|| format!("400_{}_UNIQUE", field_name.to_uppercase()));
                let error_message = args
                    .error_message
                    .unwrap_or_else(|| format!("{} must be unique", field_name));

                quote! {
                    if !checker.is_unique(#field_name, &self.#field_ident).await {
                        errors.push(::pass_me_core::ValidationError { field: #field_name, error_message: #error_message.to_string(), code: #error_code.to_string() });
                    }
                }
            }
            Err(e) => {
                panic!("Failed to parse unique attributes: {}", e.to_string());
            }
        }
    }
}
