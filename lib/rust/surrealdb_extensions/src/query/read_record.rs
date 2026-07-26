use std::future::Future;

use surrealdb::{engine::remote::ws::Client, types::SurrealValue, Surreal};

use crate::SurrealDbError;

pub trait ReadRecord: Sized + SurrealValue + Send {
    fn read_record(
        db: &Surreal<Client>,
        record_type: &str,
        id: &str,
    ) -> impl Future<Output = Result<Self, SurrealDbError>> + Send;
}

impl<T> ReadRecord for T
where
    T: SurrealValue + Send,
{
    async fn read_record(
        db: &Surreal<Client>,
        record_type: &str,
        id: &str,
    ) -> Result<Self, SurrealDbError> {
        let result = db
            .select((record_type, id))
            .await?
            .ok_or(SurrealDbError::InternalError(record_type.to_string()))?;

        Ok(result)
    }
}
