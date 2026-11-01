use crate::app::user::model::User;
use crate::app::user::port::{RepositoryError, UserRepository};
use super::user_converter::to_entity;
use sea_orm::DatabaseConnection;

/**
 * UserRepository の永続化実装
 */
pub struct DbUserRepository;

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

        let result = entity.insert(db).await;
        if result.is_err() {
            return Err(RepositoryError::DatabaseError(result.err().unwrap()));
        }
        Ok(())
    }
}
