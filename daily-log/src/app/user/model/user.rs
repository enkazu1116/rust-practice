use rusty_ulid::Ulid;
use serde::{Deserialize, Serialize};

use crate::object::{UserDiscriminator, UserName};

/**
 * ユーザーモデルの構造体
 * サロゲートキー、個人情報、認証情報、監査情報を合成したモデル
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Ulid,
    pub personal_information: PersonalInformation,
    pub authentication: Authentication,
    pub audit: Audit,
}

/**
 * 構造体のコンストラクタメソッド
 * インスタンスを生成
 */
impl User {
    pub fn new(
        id: Ulid,
        personal_info: PersonalInformation,
        auth: Authentication,
        audit: Audit,
    ) -> Self {
        Self {
            id,
            personal_info,
            auth,
            audit,
        }
    }
}