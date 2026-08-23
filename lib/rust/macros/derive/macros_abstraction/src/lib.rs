pub use macros_derive::AuditibleEntity;

pub trait AuditableEntity: Sized {
    fn set_created(self, by: impl Into<String>) -> Self;
    fn set_modified(self, by: impl Into<String>) -> Self;
}
