use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::{Organization, Project};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct Team {
    pub id: Option<RecordId>,
    pub name: String,
    pub organization: Organization,
    pub parent: Box<Option<Team>>,
    pub projects: Vec<Project>,
    pub created_at: DateTime<Utc>,
}

impl Team {
    pub fn new(name: String, organization: Organization) -> Self {
        Self {
            id: None,
            name,
            organization,
            parent: Box::new(None),
            projects: Vec::new(),
            created_at: Utc::now(),
        }
    }
}
