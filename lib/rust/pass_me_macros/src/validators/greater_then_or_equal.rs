use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Lit};

use crate::validators::{FieldValidator, NumberValueArgs};

// TODO: Move to parser module
fn lit_display(lit: &Lit) -> String {
    match lit {
        Lit::Int(i) => i.base10_digits().to_string(),
        Lit::Float(f) => f.base10_digits().to_string(),
        Lit::Bool(b) => b.value.to_string(),
        other => quote!(#other).to_string(),
    }
}

// TODO: Move to parser module
fn lit_as_tokens(lit: &Lit) -> TokenStream {
    match lit {
        Lit::Int(i) => quote! { #i },
        Lit::Float(f) => quote! { #f },
        Lit::Bool(b) => quote! { #b },
        // TODO: Change panic to graceful error handling
        other => panic!(
            "greater_then_or_equal value must be a number, got: {}",
            quote!(#other)
        ),
    }
}

pub struct GreaterThenOrEqual;

impl FieldValidator for GreaterThenOrEqual {
    fn attr_name(&self) -> &'static str {
        "greater_then_or_equal"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<NumberValueArgs, darling::Error> = FromMeta::from_meta(&attr.meta);
        match args {
            Ok(args) => {
                let error_code = args.error_code.unwrap_or_else(|| {
                    format!("400_{}_GREATER_THAN_OR_EQUAL", field_name.to_uppercase())
                });
                let value_display = lit_display(&args.value);
                let error_message = args.error_message.unwrap_or_else(|| {
                    format!(
                        "{} must be greater than or equal to {}",
                        field_name, value_display
                    )
                });

                let value = lit_as_tokens(&args.value);

                quote! {
                    if !::pass_me::NumberValueGreaterThanOrEqual::is_greater_than_or_equal(&self.#field_ident, &#value) {
                        errors.push(::pass_me::ValidationError {
                            field: #field_name,
                            error_message: #error_message.to_string(),
                            code: #error_code.to_string(),
                        });
                    }
                }
            }
            Err(e) => {
                panic!("Failed to parse greater_then_or_equal args: {}", e);
            }
        }
    }
}
