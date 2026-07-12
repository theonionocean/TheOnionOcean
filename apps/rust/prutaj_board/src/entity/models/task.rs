use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};
use ulid::Ulid;

use crate::{Project, User};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct Task {
    pub id: RecordId,
    pub title: String,
    pub description: String,
    pub status: Option<String>,
    pub assigned_to: Option<User>,
    pub project: Project,
    pub created_by: User,
    pub modified_by: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        status: Option<String>,
        assigned_to: Option<User>,
        project: Project,
        created_by: User,
        modified_by: User,
    ) -> Self {
        Self {
            id: RecordId::new("task", Ulid::new().to_string()),
            title,
            description,
            status,
            assigned_to,
            project,
            created_by,
            modified_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
