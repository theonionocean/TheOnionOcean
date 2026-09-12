mod equal;
mod not_equal;
mod not_null_or_empty;
mod unique;

use proc_macro2::TokenStream;
use syn::{Attribute, Ident, Lit};

use darling::FromMeta;

pub use equal::Equal;
pub use not_equal::NotEqual;
pub use not_null_or_empty::NotNullOrEmpty;
pub use unique::Unique;

pub fn all_validators() -> Vec<Box<dyn FieldValidator>> {
    vec![
        Box::new(NotNullOrEmpty),
        Box::new(Unique),
        Box::new(Equal),
        Box::new(NotEqual),
    ]
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

#[derive(FromMeta)]
pub struct EqualArgs {
    error_code: Option<String>,
    error_message: Option<String>,
    value: Lit,
}
