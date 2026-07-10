use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::{Organization, Project};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct Team {
    pub id: RecordId,
    pub name: String,
    pub organization: Organization,
    pub parent: Box<Option<Team>>,
    pub projects: Vec<Project>,
    pub created_at: DateTime<Utc>,
}
