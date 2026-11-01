use std::time::Duration;

use db_config::DbConfig;
use sea_orm::{ConnectOptions, Database, DbConn, DbErr};

/// DB とのコネクションを生成する。
///
/// 接続情報は Secret Manager から読み、スキーマは検索パスに設定する。
pub async fn establish_connection() -> Result<DbConn, DbErr> {
    let config = DbConfig::load()
        .await
        .map_err(|err| DbErr::Custom(err.to_string()))?;

    let mut opt = ConnectOptions::new(config.url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path(config.schema);

    Database::connect(opt).await
}
