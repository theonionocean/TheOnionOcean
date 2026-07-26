use std::future::Future;

use surrealdb::{engine::remote::ws::Client, types::SurrealValue, Surreal};

use crate::SurrealDbError;

pub trait UpdateRecord<T>: Sized
where
    T: SurrealValue + Send,
{
    fn update_record(
        self,
        db: &Surreal<Client>,
        record_type: &str,
        id: &str,
        content: T,
    ) -> impl Future<Output = Result<T, SurrealDbError>> + Send;
}

impl<T> UpdateRecord<T> for T
where
    T: SurrealValue + Send,
{
    async fn update_record(
        self,
        db: &Surreal<Client>,
        record_type: &str,
        id: &str,
        content: T,
    ) -> Result<Self, SurrealDbError> {
        let result = db
            .update((record_type, id))
            .content(content)
            .await?
            .ok_or(SurrealDbError::InternalError(record_type.to_string()))?;

        Ok(result)
    }
}
