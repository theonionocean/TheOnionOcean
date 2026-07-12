use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};
use ulid::Ulid;

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct User {
    pub id: RecordId,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: RecordId::new("user", Ulid::new().to_string()),
            name,
            email,
            created_at: Utc::now(),
        }
    }
}
