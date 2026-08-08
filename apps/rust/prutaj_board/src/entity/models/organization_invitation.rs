use chrono::{DateTime, Utc};
use macros::AuditibleEntity;
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};
use ulid::Ulid;

use crate::{OrganizationRole, Team, User};

#[derive(SurrealValue, Serialize, Deserialize, AuditibleEntity)]
pub struct OrganizationInvitation {
    pub id: Option<RecordId>,
    pub organization_role: OrganizationRole,
    pub email: String,
    pub token: String,
    pub expires: DateTime<Utc>,
    pub teams: Vec<Team>,
    pub used_at: Option<DateTime<Utc>>,
    pub used_by: Option<User>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub modified_by: String,
    pub modified_at: DateTime<Utc>,
}

impl OrganizationInvitation {
    pub fn new(
        organization_role: OrganizationRole,
        email: String,
        expires: DateTime<Utc>,
        teams: Vec<Team>,
    ) -> Self {
        Self {
            id: None,
            organization_role,
            email,
            token: Ulid::generate().to_string(),
            expires,
            teams,
            used_at: None,
            used_by: None,
            created_by: "Anonymous".to_string(),
            created_at: Utc::now(),
            modified_by: "Anonymous".to_string(),
            modified_at: Utc::now(),
        }
    }
}
