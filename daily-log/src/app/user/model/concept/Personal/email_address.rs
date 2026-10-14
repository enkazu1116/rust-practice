use regex::Regex;

const REGEXP_EMAIL: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

/**
 * Emailアドレス
 */
#[derive(Debug)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email: String) -> Result<Self, &'static str> {
        if email.is_empty() {
            return Err("Email is required");
        }

        // 正規化パターンに一致しない場合は、メールアドレスの形式が正しくない。
        let pattern = Regex::new(REGEXP_EMAIL).expect("Failed to compile regex");
        if !pattern.is_match(&email) {
            return Err("Invalid email address");
        }

        if email.chars().count() > 255 {
            return Err("Email is too long");
        }

        Ok(Self(email))
    }
}
