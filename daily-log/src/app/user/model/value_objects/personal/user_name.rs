use regex::Regex;

/// 苗字・名前として許可する文字
const REGEXP_USER_NAME: &str = r"^[A-Za-z\p{Hiragana}\p{Katakana}\p{Han}々ー]+$";

/**
 * ユーザー名
 * 苗字と名前を合成した値をユーザー名とする
 *
 * ユーザーモデルでは、苗字と名前をもとに作成したことを意識せず使用するために
 * 概念で先に定義・使用する際には正しく合成されたもののみが扱えるようにする。
 */
#[derive(Debug)]
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
        let first_name = first_name.trim();
        let last_name = last_name.trim();

        if first_name.is_empty() || last_name.is_empty() {
            return Err("FirstName and LastName are required");
        }

        // 結合すると空白が入るため、苗字と名前をそれぞれ照合する。
        let pattern = Regex::new(REGEXP_USER_NAME).expect("Failed to compile regex");
        if !pattern.is_match(first_name) || !pattern.is_match(last_name) {
            return Err("UserName must be Japanese or English letters");
        }

        let full_name = format!("{first_name} {last_name}");
        if full_name.chars().count() > 30 {
            return Err("FullName is too long");
        }

        Ok(Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        })
    }

    /// 苗字と名前を空白1つでつないだ、データベースに保存するユーザー名
    pub fn user_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}
