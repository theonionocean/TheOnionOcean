mod not_null_or_empty;
mod unique;

use proc_macro2::TokenStream;
use syn::{Attribute, Ident};

use darling::FromMeta;

pub use not_null_or_empty::NotNullOrEmpty;
pub use unique::Unique;

pub fn all_validators() -> Vec<Box<dyn FieldValidator>> {
    vec![Box::new(NotNullOrEmpty), Box::new(Unique)]
}

pub trait FieldValidator {
    fn attr_name(&self) -> &'static str;
    fn is_stateful(&self) -> bool {
        false
    }
    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream;
}

#[derive(FromMeta)]
pub struct Args {
    error_code: Option<String>,
    error_message: Option<String>,
}
