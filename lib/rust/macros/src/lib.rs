use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AuditibleEntity)]
pub fn auditible_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn set_created(&mut self, by: impl Into<String>) {
                let now = ::chrono::Utc::now();
                let by = by.into();
                self.created_by = by.clone();
                self.created_at = now;
                self.modified_by = by;
                self.modified_at = now;
            }

            pub fn set_modified(&mut self, by: impl Into<String>) {
                self.modified_by = by.into();
                self.modified_at = ::chrono::Utc::now();
            }
        }
    }
    .into()
}
