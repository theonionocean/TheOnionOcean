use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

#[proc_macro_derive(AuditibleEntity)]
pub fn auditible_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Error::new_spanned(
                    &input,
                    "AuditibleEntity only supports structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return Error::new_spanned(&input, "AuditibleEntity only supports structs")
                .to_compile_error()
                .into();
        }
    };

    let mut field_defaults = Vec::new();
    for field in fields {
        let Some(ident) = &field.ident else {
            return Error::new_spanned(field, "AuditibleEntity requires named fields")
                .to_compile_error()
                .into();
        };
        field_defaults.push(match ident.to_string().as_str() {
            "created_by" | "modified_by" => {
                quote! { #ident: "Anonymous".to_string() }
            }
            "created_at" | "modified_at" => {
                quote! { #ident: now }
            }
            _ => quote! { #ident: ::core::default::Default::default() },
        });
    }

    quote! {
        impl #impl_generics ::macros_abstraction::AuditableEntity for #name #ty_generics #where_clause {
            fn set_created(mut self, by: impl Into<String>) -> Self {
                let now = ::chrono::Utc::now();
                let by = by.into();
                self.created_by = by.clone();
                self.created_at = now;
                self.modified_by = by;
                self.modified_at = now;
                self
            }

            fn set_modified(mut self, by: impl Into<String>) -> Self {
                self.modified_by = by.into();
                self.modified_at = ::chrono::Utc::now();
                self
            }
        }

        impl #impl_generics ::core::default::Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                let now = ::chrono::Utc::now();
                Self {
                    #(#field_defaults,)*
                }
            }
        }
    }
    .into()
}
