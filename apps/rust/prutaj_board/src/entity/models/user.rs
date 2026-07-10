use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct User {
    pub id: RecordId,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}
