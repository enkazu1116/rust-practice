/**
 * UserRepository
 * ユーザーリポジトリのDB操作を抽象化
 */
pub trait UserRepository {
    async fn create(user: User) -> Result<(), Error>;
}