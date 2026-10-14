use crate::app::user::application::adapter::persistence::entity::UserEntity;
use crate::app::user::model::User;

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
