mod error;

pub use error::ValidationError;

pub trait UniquenessChecker {
    fn is_unique(
        &self,
        field: &str,
        value: &str,
    ) -> impl std::future::Future<Output = bool> + Send;
}
