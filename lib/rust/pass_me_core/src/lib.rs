mod error;
mod validators;

pub use error::ValidationError;
pub use validators::NotNullOrEmpty;

pub trait UniquenessChecker {
    fn is_unique(
        &self,
        field: &str,
        value: &str,
    ) -> impl std::future::Future<Output = bool> + Send;
}
