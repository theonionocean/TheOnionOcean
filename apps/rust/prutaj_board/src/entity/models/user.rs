use chrono::{DateTime, Utc};
use macros::AuditibleEntity;
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    types::{RecordId, SurrealValue},
    Surreal,
};
use surrealdb_extensions::{CreateRecord, DeleteRecord, ReadRecord, SurrealDbError, UpdateRecord};
use utoipa::ToSchema;

const TABLE_NAME: &str = "user";

#[derive(SurrealValue, Serialize, Deserialize, Debug, Clone, ToSchema, AuditibleEntity)]
pub struct User {
    #[schema(value_type = Option<String>)]
    pub id: Option<RecordId>,
    pub name: String,
    pub email: String,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub modified_by: String,
    pub modified_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            ..Default::default()
        }
    }

    pub async fn create(self, db: &Surreal<Client>) -> Result<User, SurrealDbError> {
        let result = self.create_record(db, TABLE_NAME).await?;
        Ok(result)
    }

    pub async fn find(db: &Surreal<Client>, id: &str) -> Result<User, SurrealDbError> {
        let result = User::read_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }

    pub async fn update(
        self,
        db: &Surreal<Client>,
        id: &str,
        content: User,
    ) -> Result<User, SurrealDbError> {
        let result = content
            .clone()
            .update_record(db, TABLE_NAME, id, content)
            .await?;
        Ok(result)
    }

    pub async fn delete(self, db: &Surreal<Client>, id: &str) -> Result<(), SurrealDbError> {
        let result = self.delete_record(db, TABLE_NAME, id).await?;
        Ok(result)
    }
}
