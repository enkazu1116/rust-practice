use crate::app::user::application::usecase;
use crate::app::user::model::User;
use crate::app::user::port::{RepositoryError, UserRepository};

/**
 * ユーザー作成の入力を usecase へ渡す
 */
pub async fn create_user(
    repository: &impl UserRepository,
    user: User,
) -> Result<(), RepositoryError> {
    usecase::create_user(repository, user).await
}
