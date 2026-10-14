
/**
 * 認証情報の概念モデル
 * 
 * パスワードを保持する。
 */
#[derive(Debug)]
pub struct Authentication {
    pub password: Password,
}

/**
 * 検証が通った認証情報のみをインスタンス化
 */
impl Authentication {
    pub fn new(password: String) -> Result<Self, &'static str> {
        if password.is_empty() {
            return Err("Password is required");
        }

        if password.chars().count() < 8 {
            return Err("Password must be at least 8 characters long");
        }

        Self { password }
    }
}