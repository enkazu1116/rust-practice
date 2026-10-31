use crate::app::user::model::User;
use crate::app::user::port::{RepositoryError, UserRepository};

/**
 * ユーザーを作成する
 */
pub async fn create_user(
    repository: &impl UserRepository,
    user: User,
) -> Result<(), RepositoryError> {
    repository.create(user).await
}
