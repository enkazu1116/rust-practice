use std::{env, fmt};

/**
 * Secret Managerから取得するDB設定
 * デバッグ用コード。接続を確認できたら削除するコードである。
 */
pub struct DatabaseConfig {
    pub url: String,
    pub schema: String,
}

/**
 * DB接続情報を可視化し、失敗原因の調査解析に使用する。
 */
impl fmt::Debug for DatabaseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbConfig")
            .field("schema", &self.schema)
            // URLにはパスワードが含まれるため、[redacted]として表示する
            .field("url", &"[redacted]")
            .finish()
    }
}