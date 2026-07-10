use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};

use crate::{Team, User};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct Organization {
    pub id: RecordId,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub legal_name: Option<String>,
    pub logo: Option<String>,
    pub owner: User,
    pub slug: String,
    pub teams: Vec<Team>,
}
