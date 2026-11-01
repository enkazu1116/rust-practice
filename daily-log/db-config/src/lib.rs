use std::{env, fmt};

use google_cloud_secretmanager_v1::client::SecretManagerService;
use serde::Deserialize;

/// Secret Manager の接続情報と、実行環境のホストから組み立てた DB 設定。
///
/// URL にはパスワードが含まれるため、`Debug` では出さない。
pub struct DbConfig {
    pub url: String,
    pub schema: String,
}

impl fmt::Debug for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbConfig")
            .field("schema", &self.schema)
            .field("url", &"[redacted]")
            .finish()
    }
}

#[derive(Debug)]
pub enum DbConfigError {
    MissingEnv(&'static str),
    SecretManager(String),
    SecretPayload,
    SecretJson,
    InvalidField(&'static str),
    InvalidSchema,
}

impl fmt::Display for DbConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnv(name) => write!(f, "{name} が設定されていません"),
            Self::SecretManager(err) => {
                write!(
                    f,
                    "Secret Manager からシークレットを読めませんでした: {err}"
                )
            }
            Self::SecretPayload => write!(f, "シークレットのペイロードを読めません"),
            Self::SecretJson => write!(f, "シークレットの JSON を解釈できません"),
            Self::InvalidField(name) => write!(f, "{name} が空、または不正です"),
            Self::InvalidSchema => write!(
                f,
                "スキーマ名は英数字、アンダースコア、ハイフンだけを使えます"
            ),
        }
    }
}

impl std::error::Error for DbConfigError {}

#[derive(Deserialize)]
struct SecretDocument {
    username: String,
    password: String,
    database: String,
    schema: String,
    instance_connection_name: String,
    port: u16,
}

impl DbConfig {
    /// `.env` のプロジェクトとシークレット ID で Secret Manager を読み、接続 URL を組み立てる。
    ///
    /// ホストは `DB_HOST`。未設定なら Cloud SQL Auth Proxy 向けに `127.0.0.1`。
    pub async fn load() -> Result<Self, DbConfigError> {
        dotenvy::dotenv().ok();

        let project_id = required_env("GCP_PROJECT_ID")?;
        let secret_id = required_env("DB_SECRET_ID")?;
        let host = env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        if host.is_empty() {
            return Err(DbConfigError::InvalidField("DB_HOST"));
        }

        let client = SecretManagerService::builder()
            .build()
            .await
            .map_err(|err| DbConfigError::SecretManager(err.to_string()))?;

        let response = client
            .access_secret_version()
            .set_name(format!(
                "projects/{project_id}/secrets/{secret_id}/versions/latest"
            ))
            .send()
            .await
            .map_err(|err| DbConfigError::SecretManager(err.to_string()))?;

        let payload = response.payload.ok_or(DbConfigError::SecretPayload)?;
        let text = std::str::from_utf8(&payload.data).map_err(|_| DbConfigError::SecretPayload)?;
        let document: SecretDocument =
            serde_json::from_str(text).map_err(|_| DbConfigError::SecretJson)?;
        document.check()?;

        Ok(Self {
            url: database_url(
                &document.username,
                &document.password,
                &host,
                document.port,
                &document.database,
            ),
            schema: document.schema,
        })
    }
}

impl SecretDocument {
    fn check(&self) -> Result<(), DbConfigError> {
        require_non_empty(&self.username, "username")?;
        require_non_empty(&self.password, "password")?;
        require_non_empty(&self.database, "database")?;
        require_non_empty(&self.instance_connection_name, "instance_connection_name")?;
        if self.port == 0 {
            return Err(DbConfigError::InvalidField("port"));
        }
        validate_schema(&self.schema)
    }
}

fn required_env(name: &'static str) -> Result<String, DbConfigError> {
    match env::var(name) {
        Ok(value) if !value.is_empty() => Ok(value),
        _ => Err(DbConfigError::MissingEnv(name)),
    }
}

fn require_non_empty(value: &str, field: &'static str) -> Result<(), DbConfigError> {
    if value.is_empty() {
        Err(DbConfigError::InvalidField(field))
    } else {
        Ok(())
    }
}

fn validate_schema(schema: &str) -> Result<(), DbConfigError> {
    if !schema.is_empty()
        && schema
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        Ok(())
    } else {
        Err(DbConfigError::InvalidSchema)
    }
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

fn database_url(username: &str, password: &str, host: &str, port: u16, database: &str) -> String {
    format!(
        "postgresql://{}:{}@{}:{}/{}",
        encode_userinfo(username),
        encode_userinfo(password),
        host,
        port,
        encode_userinfo(database)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_password_and_builds_url() {
        let url = database_url("admin_user", "p@ss", "127.0.0.1", 5432, "simple-compute");
        assert_eq!(
            url,
            "postgresql://admin_user:p%40ss@127.0.0.1:5432/simple-compute"
        );
    }

    #[test]
    fn accepts_schema_names_used_by_terraform() {
        assert!(validate_schema("simple-compute").is_ok());
        assert!(validate_schema("app_schema").is_ok());
    }

    #[test]
    fn rejects_schema_names_that_cannot_be_quoted_safely() {
        assert!(validate_schema("").is_err());
        assert!(validate_schema("bad\"schema").is_err());
        assert!(validate_schema("bad schema").is_err());
    }
}
