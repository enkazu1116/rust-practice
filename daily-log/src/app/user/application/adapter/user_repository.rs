use super::user_entity::UserEntity;
use crate::app::user::model::User;
use crate::app::user::port::{RepositoryError, UserRepository};

/**
 * UserRepository の永続化実装
 */
pub struct DbUserRepository;

impl UserRepository for DbUserRepository {
    async fn create(&self, user: User) -> Result<(), RepositoryError> {
        let _entity = UserEntity::from(user);
        Err(RepositoryError)
    }
}
