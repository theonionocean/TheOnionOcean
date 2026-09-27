mod error;
mod validators;

pub use error::ValidationError;
pub use validators::{
    Equal, Length, Matches, NotNullOrEmpty, NumberValueGreaterThan, NumberValueGreaterThanOrEqual,
    NumberValueLessThan, NumberValueLessThanOrEqual,
};

pub trait MustChecker {
    fn must(&self, field: &str, value: &str) -> impl std::future::Future<Output = bool> + Send;
}
