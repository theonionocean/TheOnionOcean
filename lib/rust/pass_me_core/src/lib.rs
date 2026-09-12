mod error;
mod validators;

pub use error::ValidationError;
pub use validators::{Equal, Length, NotNullOrEmpty};

pub trait UniquenessChecker {
    fn is_unique(
        &self,
        field: &str,
        value: &str,
    ) -> impl std::future::Future<Output = bool> + Send;
}
