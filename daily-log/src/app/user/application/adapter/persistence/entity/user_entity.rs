use chrono::{DateTime, Utc};
use rusty_ulid::Ulid;

/**
 * ユーザーテーブルの1行
 */
#[derive(Debug)]
pub struct UserEntity {
    pub id: Ulid,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
