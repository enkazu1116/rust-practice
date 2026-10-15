use crate::app::user::model::User;

/**
 * ユーザーの永続化
 */
pub trait UserRepository {
    async fn create(&self, user: User) -> Result<(), RepositoryError>;
}

#[derive(Debug)]
pub struct RepositoryError;

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to create user")
    }
}

impl std::error::Error for RepositoryError {}
