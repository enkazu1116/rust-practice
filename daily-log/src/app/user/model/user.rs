use rusty_ulid::Ulid;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::object::{UserDiscriminator, UserName};

/**
 * ユーザーEntityの構造体
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Ulid,
    pub discriminator: UserDiscriminator,
    pub name: UserName,
    pub email: EmailAddress,
    pub password: Password,
}

/**
 * 構造体のメソッド
 * インスタンスを生成
 */
impl User {
    pub fn new(
        id: Ulid,
        discriminator: UserDiscriminator,
        name: UserName,
        email: EmailAddress,
        password: Password,
    ) -> Self {
        Self {
            id,
            discriminator,
            name,
            email,
            password,
        }
    }
}