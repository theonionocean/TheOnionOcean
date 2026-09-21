mod validators;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::{Data::Struct, Fields::Named};

use validators::all_validators;

#[proc_macro_derive(
    Validate,
    attributes(
        non_null_or_empty,
        unique,
        equal,
        not_equal,
        length,
        max_length,
        min_length
    )
)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // TODO: Change panic to graceful error handling
    let Struct(data) = &input.data else {
        panic!("Validate can only be derived for structs");
    };
    let Named(fields) = &data.fields else {
        panic!("Validate can only be derived for structs with named fields");
    };

    let registry = all_validators();
    let mut sync_checks = Vec::new();
    let mut stateful_checks = Vec::new();

    for field in &fields.named {
        let ident = field.ident.as_ref();

        match ident {
            Some(ident) => {
                let field_name = ident.to_string();

                for attr in &field.attrs {
                    if let Some(validator) = registry
                        .iter()
                        .find(|v| attr.path().is_ident(v.attr_name()))
                    {
                        let check = validator.generate(&attr, ident, &field_name);
                        if validator.is_stateful() {
                            stateful_checks.push(check);
                        } else {
                            sync_checks.push(check);
                        }
                    }
                }
            }
            None => {
                // TODO: Change panic to graceful error handling
                panic!("Validate can only be derived for structs with named fields");
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn validate(&self) -> Vec<::pass_me::ValidationError> {
                let mut errors = Vec::new();
                #( #sync_checks )*
                errors
            }
            pub async fn validate_with<C: ::pass_me::UniquenessChecker>(
                &self,
                checker: &C,
            ) -> Vec<::pass_me::ValidationError> {
                let mut errors = self.validate();
                #( #stateful_checks )*
                errors
            }
        }
    };

    expanded.into()
}
