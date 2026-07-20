use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    types::{RecordId, SurrealValue},
    Surreal,
};
use surrealdb_extensions::{CreateRecord, SurrealDbError, UpdateRecord};

const TABLE_NAME: &str = "user";

#[derive(SurrealValue, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<RecordId>,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: None,
            name,
            email,
            created_at: Utc::now(),
        }
    }

    pub async fn create(self, db: &Surreal<Client>) -> Result<User, SurrealDbError> {
        let result = self.create_record(db, TABLE_NAME).await?;
        Ok(result)
    }

    pub async fn update(self, db: &Surreal<Client>, id: &str) -> Result<User, SurrealDbError> {
        let result = self.clone().update_record(db, TABLE_NAME, id, self).await?;
        Ok(result)
    }
}
