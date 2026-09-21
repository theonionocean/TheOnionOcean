use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident};

use crate::validators::{FieldValidator, MinLengthArgs};

pub struct MinLength;

impl FieldValidator for MinLength {
    fn attr_name(&self) -> &'static str {
        "min_length"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<MinLengthArgs, darling::Error> = FromMeta::from_meta(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args
                    .error_code
                    .unwrap_or_else(|| format!("400_{}_MIN_LENGTH", field_name.to_uppercase()));
                let error_message = args.error_message.unwrap_or_else(|| {
                    format!("{} must be at least {} characters", field_name, args.min)
                });

                let min = args.min;

                if min < 1 {
                    return syn::Error::new_spanned(attr, "min_length must be at least 1")
                        .to_compile_error();
                }

                quote! {
                    if !::pass_me::Length::is_length(&self.#field_ident, #min, usize::MAX) {
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
                panic!("Failed to parse min_length attributes: {}", e.to_string());
            }
        }
    }
}
