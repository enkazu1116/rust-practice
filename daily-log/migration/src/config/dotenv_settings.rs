use crate::config::configuration_error::ConfigurationError;
use std::env;

/**
 * 環境変数から取得する情報
 */
pub struct DotenvConfig {
    pub project_id: String,
    pub secret_id: String,
    pub host: String,
}

/**
 * 環境変数ファイルから情報を取得
 */
pub fn load_dotenv() -> Result<DotenvConfig, ConfigurationError> {
    dotenvy::dotenv().ok();

    let project_id = match env::var("GCP_PROJECT_ID") {
        Ok(value) if !value.is_empty() => value,
        _ => return Err(ConfigurationError::MissingEnv("GCP_PROJECT_ID")),
    };

    let secret_id = match env::var("DB_SECRET_ID") {
        Ok(value) if !value.is_empty() => value,
        _ => return Err(ConfigurationError::MissingEnv("DB_SECRET_ID")),
    };

    let host = match env::var("DB_HOST") {
        Ok(value) if !value.is_empty() => value,
        _ => return Err(ConfigurationError::MissingEnv("DB_HOST")),
    };

    Ok(DotenvConfig {
        project_id,
        secret_id,
        host,
    })
}
