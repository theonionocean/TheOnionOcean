use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::User;

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<RecordId>,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub created_by: User,
    pub modified_by: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        name: String,
        description: String,
        slug: String,
        created_by: User,
        modified_by: User,
    ) -> Self {
        Self {
            id: None,
            name,
            description,
            slug,
            created_by,
            modified_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
