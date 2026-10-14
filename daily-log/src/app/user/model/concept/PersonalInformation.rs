use regex::Regex;

use crate::object::{PersonalInformation};

// ユーザー名
const REGEXP_USER_NAME: &str = r"^[A-Za-z\p{Hiragana}\p{Katakana}\p{Han}々ー]+$";

// メールアドレス
const REGEXP_EMAIL: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

/**
 * ユーザー名
 * 苗字と名前を合成した値をユーザー名とする
 * 
 * ユーザーモデルでは、苗字と名前をもとに作成したことを意識せず使用するために
 * 概念で先に定義・使用する際には正しく合成されたもののみが扱えるようにする。
 */
pub struct UserName {
    first_name: String,
    last_name: String,
}

/**
 * ユーザー名を作成
 * 
 * 作成段階で不正な値や正規化パターンに一致しないものはエラーとする。
 */
impl UserName {
    pub fn new(first_name: String, last_name: String) -> Result<Self, &'static str> {
        if first_name.is_empty() || last_name.is_empty({
            return Err("FirstName and LastName are required");
        }
        // ユーザー名を生成
        let full_name = format!("{} {}", first_name.trim(), last_name.trim());

        // 正規化パターンに一致しない場合は、日本語か英語の文字のみが扱えるようにする。
        let pattern = Regex::new(REGEXP_USER_NAME).expect("Failed to compile regex");
        if !pattern.is_match(&full_name) {
            return Err("UserName must be Japanese or English letters");
        }
        
        if full_name.chars().count() > 30 {
            return Err("FullName is too long");
        }

        Ok(Self { full_name })
    }
}

/**
 * Emailアドレス
 */
pub struct EmailAddress(String);
impl EmailAddress {
    pub fn new(email: String) -> Result<Self, &`static str> {
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

/**
 * 個人情報の概念モデル
 */
#[derive(Debug)]
pub struct PersonalInformation {
    pub name: UserName,
    pub email: EmailAddress,
}

/**
 * 構造体のコンストラクタメソッド
 */
impl PersonalInformation {
    pub fn new(name: String, email: String, password: String) -> Self {
        Self { name, email, password }
    }
}