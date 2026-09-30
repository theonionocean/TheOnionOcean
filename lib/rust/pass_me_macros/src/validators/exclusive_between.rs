use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Lit};

use crate::validators::{BetweenNumbersArgs, FieldValidator};

// TODO: Move to parser module
fn lit_display(lit: &Lit) -> String {
    match lit {
        Lit::Int(i) => i.base10_digits().to_string(),
        Lit::Float(f) => f.base10_digits().to_string(),
        other => quote!(#other).to_string(),
    }
}

// TODO: Move to parser module
fn lit_as_tokens(lit: &Lit) -> TokenStream {
    match lit {
        Lit::Int(i) => quote! { #i },
        Lit::Float(f) => quote! { #f },
        // TODO: Change panic to graceful error handling
        other => panic!(
            "exclusive_between value must be a number, got: {}",
            quote!(#other)
        ),
    }
}

pub struct ExclusiveBetween;

impl FieldValidator for ExclusiveBetween {
    fn attr_name(&self) -> &'static str {
        "exclusive_between"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<BetweenNumbersArgs, darling::Error> = FromMeta::from_meta(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args.error_code.unwrap_or_else(|| {
                    format!("400_{}_EXCLUSIVE_BETWEEN", field_name.to_uppercase())
                });
                let min_display = lit_display(&args.min);
                let max_display = lit_display(&args.max);
                let error_message = args.error_message.unwrap_or_else(|| {
                    format!(
                        "{} must be between {} and {}",
                        field_name, min_display, max_display
                    )
                });
                let min = lit_as_tokens(&args.min);
                let max = lit_as_tokens(&args.max);

                quote! {
                    if !::pass_me::ExclusiveBetween::is_exclusive_between(&self.#field_ident, &#min, &#max) {
                        errors.push(::pass_me::ValidationError {
                            field: #field_name,
                            error_message: #error_message.to_string(),
                            code: #error_code.to_string(),
                        });
                    }
                }
            }
            Err(e) => {
                panic!("Failed to parse exclusive_between args: {}", e);
            }
        }
    }
}
