use std::future::Future;

use surrealdb::{engine::remote::ws::Client, types::SurrealValue, Surreal};

use crate::SurrealDbError;

pub trait DeleteRecord<T>: Sized
where
    T: SurrealValue + Send,
{
    fn delete_record(
        self,
        db: &Surreal<Client>,
        record_type: &str,
        id: &str,
    ) -> impl Future<Output = Result<(), SurrealDbError>> + Send;
}

impl<T> DeleteRecord<T> for T
where
    T: SurrealValue + Send,
{
    async fn delete_record(
        self,
        db: &Surreal<Client>,
        record_type: &str,
        id: &str,
    ) -> Result<(), SurrealDbError> {
        db.delete::<Option<T>>((record_type, id))
            .await?
            .ok_or(SurrealDbError::InternalError(record_type.to_string()))?;

        Ok(())
    }
}
