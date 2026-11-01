use db_config::DbConfig;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectOptions, ConnectionTrait, Database},
};

#[tokio::main]
async fn main() {
    let config = DbConfig::load()
        .await
        .expect("データベース設定を Secret Manager から読めませんでした");

    let db = Database::connect(&config.url)
        .await
        .expect("データベースへの接続に失敗しました");
    db.execute_unprepared(&format!(
        "CREATE SCHEMA IF NOT EXISTS \"{}\"",
        config.schema
    ))
    .await
    .expect("スキーマの作成に失敗しました");
    db.close().await.expect("接続のクローズに失敗しました");

    let url_for_cli = config.url;
    let schema = config.schema;
    cli::run_cli_with_custom_connection(migration::Migrator, async move || {
        let mut options = ConnectOptions::new(url_for_cli);
        options.set_schema_search_path(schema);
        Database::connect(options).await
    })
    .await;
}
