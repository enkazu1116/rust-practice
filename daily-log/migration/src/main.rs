use std::{env, fs, path::PathBuf};

use sea_orm_migration::{
    prelude::*,
    sea_orm::{ConnectOptions, ConnectionTrait, Database},
};

const DB_NAME: &str = "simple-compute";
const DB_SCHEMA: &str = "simple-compute";

fn load_db_password() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../infra/terraform.tfvars");
    let text = fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "terraform.tfvars を読めません ({}): {err}",
            path.display()
        )
    });

    for line in text.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("db_password") else {
            continue;
        };
        let rest = rest.trim().trim_start_matches('=').trim();
        return rest.trim_matches('"').to_string();
    }

    panic!("terraform.tfvars に db_password がありません");
}

fn encode_userinfo(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

fn database_url(password: &str) -> String {
    format!(
        "postgresql://admin_user:{}@127.0.0.1:5432/{DB_NAME}",
        encode_userinfo(password)
    )
}

#[tokio::main]
async fn main() {
    let url = database_url(&load_db_password());

    let db = Database::connect(&url)
        .await
        .expect("データベース simple-compute への接続に失敗しました");
    db.execute_unprepared("CREATE SCHEMA IF NOT EXISTS \"simple-compute\"")
        .await
        .expect("スキーマ simple-compute の作成に失敗しました");
    db.close().await.expect("接続のクローズに失敗しました");

    let url_for_cli = url;
    cli::run_cli_with_custom_connection(migration::Migrator, async move || {
        let mut options = ConnectOptions::new(url_for_cli);
        options.set_schema_search_path(DB_SCHEMA);
        Database::connect(options).await
    })
    .await;
}
