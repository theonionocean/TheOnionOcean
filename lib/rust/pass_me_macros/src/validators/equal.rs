use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Ident, Lit};

use crate::validators::{EqualArgs, FieldValidator};

pub struct Equal;

fn lit_display(lit: &Lit) -> String {
    match lit {
        Lit::Str(s) => s.value(),
        Lit::Int(i) => i.base10_digits().to_string(),
        Lit::Float(f) => f.base10_digits().to_string(),
        Lit::Bool(b) => b.value.to_string(),
        other => quote!(#other).to_string(),
    }
}

fn lit_as_tokens(lit: &Lit) -> TokenStream {
    match lit {
        Lit::Str(s) => {
            let value = s.value();
            quote! { #value.to_string() }
        }
        Lit::Int(i) => quote! { #i },
        Lit::Float(f) => quote! { #f },
        Lit::Bool(b) => quote! { #b },
        // TODO: Change panic to graceful error handling
        other => panic!(
            "equal value must be a string or number, got: {}",
            quote!(#other)
        ),
    }
}

// TODO: Implement stateful validation
// TODO: Implement validation for string (ignore case, trim whitespace, etc.)
impl FieldValidator for Equal {
    fn attr_name(&self) -> &'static str {
        "equal"
    }

    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream {
        let args: Result<EqualArgs, darling::Error> = FromMeta::from_meta(&attr.meta);

        match args {
            Ok(args) => {
                let error_code = args
                    .error_code
                    .unwrap_or_else(|| format!("400_{}_EQUAL", field_name.to_uppercase()));
                let value_display = lit_display(&args.value);
                let error_message = args.error_message.unwrap_or_else(|| {
                    format!("{} must be equal to {}", field_name, value_display)
                });
                let value = lit_as_tokens(&args.value);

                quote! {
                    if !::pass_me::Equal::is_equal(&self.#field_ident, &#value) {
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
                panic!("Failed to parse equal attributes: {}", e.to_string());
            }
        }
    }
}
