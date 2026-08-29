use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

use crate::validators::{Args, FieldValidator};

pub struct NonNullOrEmpty;

impl FieldValidator for NonNullOrEmpty {
    fn attr_name(&self) -> &'static str {
        "non_null_or_empty"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<Args, darling::Error> = FromMeta::from_meta(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args.error_code.unwrap_or_else(|| {
                    format!("400_{}_NON_NULL_OR_EMPTY", field_name.to_uppercase())
                });
                let error_message = args
                    .error_message
                    .unwrap_or_else(|| format!("{} cannot be null or empty", field_name));

                quote! {
                    if #field_ident.is_none() || #field_ident.is_empty() {
                        errors.push(ValidationError { field: #field_name, error_message: #error_message, code: #error_code });
                    }
                }
            }
            Err(e) => {
                panic!(
                    "Failed to parse non_null_or_empty attributes: {}",
                    e.to_string()
                );
            }
        }
    }
}
