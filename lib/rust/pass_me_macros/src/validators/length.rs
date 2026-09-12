use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

use crate::validators::{FieldValidator, LengthArgs};

pub struct Length;

impl FieldValidator for Length {
    fn attr_name(&self) -> &'static str {
        "length"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<LengthArgs, darling::Error> = FromMeta::from_meta(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args
                    .error_code
                    .unwrap_or_else(|| format!("400_{}_LENGTH", field_name.to_uppercase()));
                let error_message =
                    args.error_message
                        .unwrap_or_else(|| match (args.min, args.max) {
                            (Some(min), Some(max)) => {
                                format!(
                                    "{} must be between {} and {} characters",
                                    field_name, min, max
                                )
                            }
                            (Some(min), None) => {
                                format!("{} must be at least {} characters", field_name, min)
                            }
                            (None, Some(max)) => {
                                format!("{} must be at most {} characters", field_name, max)
                            }
                            (None, None) => {
                                // TODO: Change panic to graceful error handling
                                panic!("length validator requires at least one of min or max");
                            }
                        });

                if let (Some(min), Some(max)) = (args.min, args.max) {
                    if min > max {
                        return syn::Error::new_spanned(
                            attr,
                            format!("length min ({min}) cannot be greater than max ({max})"),
                        )
                        .to_compile_error();
                    }
                }

                let min = args.min.unwrap_or(0);
                let max = args.max.unwrap_or(usize::MAX);

                quote! {
                    if !::pass_me::Length::is_length(&self.#field_ident, #min, #max) {
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
                panic!("Failed to parse length attributes: {}", e.to_string());
            }
        }
    }
}
