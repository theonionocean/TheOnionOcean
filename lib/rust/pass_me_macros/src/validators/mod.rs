mod equal;
mod length;
mod max_length;
mod not_equal;
mod not_null_or_empty;
mod unique;

use proc_macro2::TokenStream;
use syn::{Attribute, Ident, Lit, Meta};

use darling::FromMeta;

pub use equal::Equal;
pub use length::Length;
pub use max_length::MaxLength;
pub use not_equal::NotEqual;
pub use not_null_or_empty::NotNullOrEmpty;
pub use unique::Unique;

pub fn all_validators() -> Vec<Box<dyn FieldValidator>> {
    vec![
        Box::new(NotNullOrEmpty),
        Box::new(Unique),
        Box::new(Equal),
        Box::new(NotEqual),
        Box::new(Length),
        Box::new(MaxLength),
    ]
}

pub trait FieldValidator {
    fn attr_name(&self) -> &'static str;
    fn is_stateful(&self) -> bool {
        false
    }
    fn generate(&self, attr: &Attribute, field_ident: &Ident, field_name: &str) -> TokenStream;
}

pub fn parse_meta_args<T: FromMeta + Default>(meta: &Meta) -> Result<T, darling::Error> {
    match meta {
        Meta::Path(_) => Ok(T::default()),
        other => T::from_meta(other),
    }
}

#[derive(Default, FromMeta)]
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

#[derive(FromMeta)]
pub struct LengthArgs {
    error_code: Option<String>,
    error_message: Option<String>,
    min: Option<usize>,
    max: Option<usize>,
}

#[derive(FromMeta)]
pub struct MaxLengthArgs {
    error_code: Option<String>,
    error_message: Option<String>,
    max: usize,
}
