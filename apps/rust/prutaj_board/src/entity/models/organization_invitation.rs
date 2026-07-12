use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, SurrealValue};
use ulid::Ulid;

use crate::{OrganizationRole, Team, User};

#[derive(SurrealValue, Serialize, Deserialize)]
pub struct OrganizationInvitation {
    pub id: RecordId,
    pub organization_role: OrganizationRole,
    pub email: String,
    pub token: String,
    pub expires: DateTime<Utc>,
    pub teams: Vec<Team>,
    pub used_at: Option<DateTime<Utc>>,
    pub used_by: Option<User>,
    pub created_by: User,
}

impl OrganizationInvitation {
    pub fn new(
        organization_role: OrganizationRole,
        email: String,
        expires: DateTime<Utc>,
        teams: Vec<Team>,
        created_by: User,
    ) -> Self {
        Self {
            id: RecordId::new("organization_invitation", Ulid::new().to_string()),
            organization_role,
            email,
            token: Ulid::new().to_string(),
            expires,
            teams,
            used_at: None,
            used_by: None,
            created_by,
        }
    }
}
