use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::User;

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct Project {
    pub id: RecordId,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub created_by: User,
    pub modified_by: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
