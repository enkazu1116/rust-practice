use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

/**
 * ユーザー識別子の値オブジェクト
 */
#[derive(Clone, Debug, PartiaEq, Eq, Serialize, Deserialize)]
pub struct UserDiscriminator(String);

/**
 * 識別子の検証
 */
impl UserDiscriminator {
    pub fn new(discriminator: String) -> Result<Self, String> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9-]{2,23}$").unwrap());
        if RE.is_match(&discriminator) {
            Ok(Self(discriminator))
        } else {
            Err("Invalid discriminator".to_string())
        }
    }
}