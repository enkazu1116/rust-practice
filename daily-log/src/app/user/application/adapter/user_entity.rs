use chrono::{DateTime, Utc};
use rusty_ulid::Ulid;
user sea_orm::entity::prelude::*;

use crate::app::user::model::User;

/**
 * ユーザーテーブルの1行
 */
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Eq)]
pub struct UserEntity {
    pub id: Ulid,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/**
 * ユーザーモデルを、テーブルの列に対応するエンティティへ変換する
 */
impl From<User> for UserEntity {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            user_name: user.personal_information.name.user_name(),
            email: user.personal_information.email.as_str().to_string(),
            password: user.authentication.password.as_str().to_string(),
            created_at: user.audit.created_at,
            updated_at: user.audit.updated_at,
            deleted_at: user.audit.deleted_at,
        }
    }
}
