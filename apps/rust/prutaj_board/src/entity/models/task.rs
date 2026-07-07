use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::{Project, User};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct Task {
    pub id: RecordId,
    pub title: String,
    pub description: String,
    pub status: String,
    pub assigned_to: Option<User>,
    pub project: Project,
    pub created_by: User,
    pub modified_by: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
