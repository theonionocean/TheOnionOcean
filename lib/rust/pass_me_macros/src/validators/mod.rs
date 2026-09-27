mod email;
mod equal;
mod greater_then;
mod greater_then_or_equal;
mod length;
mod less_then;
mod less_then_or_equal;
mod matches;
mod max_length;
mod min_length;
mod must;
mod not_equal;
mod not_null_or_empty;

use proc_macro2::TokenStream;
use syn::{Attribute, Ident, Lit, Meta};

use darling::FromMeta;

pub use email::Email;
pub use equal::Equal;
pub use greater_then::GreaterThen;
pub use greater_then_or_equal::GreaterThenOrEqual;
pub use length::Length;
pub use less_then::LessThen;
pub use less_then_or_equal::LessThenOrEqual;
pub use matches::Matches;
pub use max_length::MaxLength;
pub use min_length::MinLength;
pub use must::Must;
pub use not_equal::NotEqual;
pub use not_null_or_empty::NotNullOrEmpty;

pub fn all_validators() -> Vec<Box<dyn FieldValidator>> {
    vec![
        Box::new(NotNullOrEmpty),
        Box::new(Must),
        Box::new(Equal),
        Box::new(NotEqual),
        Box::new(Length),
        Box::new(MaxLength),
        Box::new(MinLength),
        Box::new(LessThen),
        Box::new(LessThenOrEqual),
        Box::new(GreaterThen),
        Box::new(GreaterThenOrEqual),
        Box::new(Matches),
        Box::new(Email),
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

#[derive(FromMeta)]
pub struct MinLengthArgs {
    error_code: Option<String>,
    error_message: Option<String>,
    min: usize,
}

#[derive(FromMeta)]
pub struct NumberValueArgs {
    error_code: Option<String>,
    error_message: Option<String>,
    value: Lit,
}

#[derive(FromMeta)]
pub struct MatchesArgs {
    error_code: Option<String>,
    error_message: Option<String>,
    pattern: String,
}
