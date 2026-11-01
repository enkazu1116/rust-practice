use google_cloud_secretmanager_v1::client::SecretManagerService;
use serde::Deserialize;

use crate::config::configuration_error::ConfigurationError;
use crate::config::database_config::DatabaseConfig;
use crate::config::dotenv_settings::load_dotenv;

/**
 * Secret Managerから取得するシークレットドキュメント
 */
#[derive(Deserialize)]
struct DatabaseSecrets {
    username: String,
    password: String,
    database: String,
    schema: String,
    instance_connection_name: String,
    port: u16,
}

/**
 * 取得した情報の検証
 */
fn validate(document: &DatabaseSecrets) -> Result<(), ConfigurationError> {
    // 取得したフィールドが一つでも空の場合はエラーとする
    if document.username.is_empty()
        || document.password.is_empty()
        || document.database.is_empty()
        || document.instance_connection_name.is_empty()
    {
        return Err(ConfigurationError::InvalidField(
            "username, password, database, instance_connection_name",
        ));
    }

    // スキーマの値・文字種の検証をそれぞれで行う。
    if document.schema.is_empty() {
        return Err(ConfigurationError::InvalidField("schema"));
    }
    if !document
        .schema
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(ConfigurationError::InvalidSchema);
    }

    Ok(())
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

/**
 * シークレットマネージャーから情報を取得
 */
impl DatabaseConfig {
    #[allow(non_snake_case)]
    pub async fn loadConfig() -> Result<Self, ConfigurationError> {
        let dotenv_config = load_dotenv()?;

        // 使用するシークレットマネージャーのクライアントを生成
        let secret_manager = SecretManagerService::builder()
            .build()
            .await
            .map_err(|err| ConfigurationError::MissingSecretManager(err.to_string()))?;

        let response = secret_manager
            .access_secret_version()
            .set_name(format!(
                "projects/{}/secrets/{}/versions/latest",
                dotenv_config.project_id, dotenv_config.secret_id
            ))
            .send()
            .await
            .map_err(|err| ConfigurationError::MissingSecretManager(err.to_string()))?;

        let payload = response
            .payload
            .ok_or(ConfigurationError::UnableToReadSecretPayload)?;
        let text = std::str::from_utf8(&payload.data)
            .map_err(|_| ConfigurationError::UnableToReadSecretPayload)?;
        let document: DatabaseSecrets =
            serde_json::from_str(text).map_err(|_| ConfigurationError::UnableToParseSecretJson)?;

        validate(&document)?;
        Ok(Self {
            url: format!(
                "postgresql://{}:{}@{}:{}/{}",
                encode_userinfo(&document.username),
                encode_userinfo(&document.password),
                dotenv_config.host,
                document.port,
                encode_userinfo(&document.database),
            ),
            schema: document.schema,
        })
    }
}
