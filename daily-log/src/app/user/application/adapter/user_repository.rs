use super::user_converter::to_entity;
use crate::app::user::model::User;
use crate::app::user::port::{RepositoryError, UserRepository};
use sea_orm::{ActiveModelTrait, DatabaseConnection};

/**
 * UserRepository の永続化実装
 */
pub struct DbUserRepository {
    db: DatabaseConnection,
}

impl DbUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl UserRepository for DbUserRepository {
    /**
     * ユーザーレコードの作成
     * ユーザーモデルからEntityへの変換を行い、Entityを永続化する。
     *
     * @param user ユーザーモデル
     * @return 成功時は Ok(())、失敗時は RepositoryError
     */
    async fn create(&self, user: User) -> Result<(), RepositoryError> {
        let entity = to_entity(user);

        entity
            .insert(&self.db)
            .await
            .map_err(|err| RepositoryError::DatabaseError(err.to_string()))?;
        Ok(())
    }
}
