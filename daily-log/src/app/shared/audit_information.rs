use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct AuditInformation {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl AuditInformation {
    pub fn new(
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
