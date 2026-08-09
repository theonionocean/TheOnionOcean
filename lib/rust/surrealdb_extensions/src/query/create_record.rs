use std::future::Future;

use macros_abstraction::AuditableEntity;
use surrealdb::{engine::remote::ws::Client, types::SurrealValue, Surreal};

use crate::SurrealDbError;

pub trait CreateRecord<T>: Sized
where
    T: SurrealValue + AuditableEntity + Send,
{
    fn create_record(
        self,
        db: &Surreal<Client>,
        record_type: &str,
    ) -> impl Future<Output = Result<T, SurrealDbError>> + Send;
}

impl<T> CreateRecord<T> for T
where
    T: SurrealValue + AuditableEntity + Send,
{
    async fn create_record(
        self,
        db: &Surreal<Client>,
        record_type: &str,
    ) -> Result<Self, SurrealDbError> {
        let result = db
            .create((record_type, ulid::Ulid::generate().to_string()))
            .content(self)
            .await?
            .ok_or(SurrealDbError::InternalError(record_type.to_string()))?;

        Ok(result)
    }
}
