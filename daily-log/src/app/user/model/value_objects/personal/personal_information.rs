use serde::{Deserialize, Serialize};

use super::{EmailAddress, UserName};

/**
 * 個人情報の概念モデル
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalInformation {
    pub name: UserName,
    pub email: EmailAddress,
}

/**
 * 構造体のコンストラクタメソッド
 */
impl PersonalInformation {
    pub fn new(name: UserName, email: EmailAddress) -> Self {
        Self { name, email }
    }
}
