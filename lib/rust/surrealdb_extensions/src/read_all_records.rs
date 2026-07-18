use std::future::Future;

use surrealdb::{engine::remote::ws::Client, types::SurrealValue, Surreal};

use crate::SurrealDbError;

pub trait ReadAllRecords<T>: Sized
where
    T: SurrealValue + Send,
{
    fn read_all_records(
        db: &Surreal<Client>,
        record_type: &str,
    ) -> impl Future<Output = Result<Vec<T>, SurrealDbError>> + Send;
}

impl<T> ReadAllRecords<T> for T
where
    T: SurrealValue + Send,
{
    async fn read_all_records(
        db: &Surreal<Client>,
        record_type: &str,
    ) -> Result<Vec<T>, SurrealDbError> {
        let result = db.select(record_type).await?;

        Ok(result)
    }
}
