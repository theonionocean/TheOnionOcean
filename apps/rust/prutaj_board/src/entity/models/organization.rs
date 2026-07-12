use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};
use ulid::Ulid;

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

impl Organization {
    pub fn new(
        name: String,
        legal_name: Option<String>,
        logo: Option<String>,
        owner: User,
        slug: String,
    ) -> Self {
        Self {
            id: RecordId::new("organization", Ulid::new().to_string()),
            name,
            legal_name,
            logo,
            owner,
            slug,
            teams: Vec::new(),
            created_at: Utc::now(),
        }
    }
}
