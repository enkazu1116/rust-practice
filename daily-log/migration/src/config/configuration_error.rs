use thiserror::Error;

/**
 * 設定情報の読み込みに失敗した場合のエラー列挙
 */
#[derive(Debug, Error)]
pub enum ConfigurationError {
    #[error("環境変数 {0} が取得できませんでした。")]
    MissingEnv(&'static str),
    #[error("Secret Manager からシークレットを読めませんでした: {0}")]
    MissingSecretManager(String),
    #[error("シークレットのペイロードを読めませんでした。")]
    UnableToReadSecretPayload,
    #[error("シークレットの JSON を解釈できませんでした。")]
    UnableToParseSecretJson,
    #[error("{0} が空、または不正です")]
    InvalidField(&'static str),
    #[error("スキーマ名は英数字、アンダースコア、ハイフンだけを使えます")]
    InvalidSchema,

    #[error("不明なエラーが発生しました。")]
    Unknown,
}
