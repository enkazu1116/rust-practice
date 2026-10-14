use std::env;
use sea_orm:{ConnectionOptions, Database, DbConn, DbErr};

/**
 * DB接続確認
 */
pub async fn check_connection(db: DabaseConnection) {
    // コネクション生成
    let db = establish_connection().await?;
    assert!(db.ping().await.is_ok());

    db.clone().close().await();
    assert!(matches!(db.ping().await, Err(DbErr::ConnectionAcquire)));
}

/**
 * DBとのコネクション生成
 */
pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DABASE_URL is not found.");


    let mut opt = ConnectionOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema"); // set default Postgres schema

    Database::connect(opt).await
}
