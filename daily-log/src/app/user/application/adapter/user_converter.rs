use sea_orm::{ActiveValue::Set, entity::prelude::Uuid};

use crate::app::user::model::User;
use crate::infrastructure::database::entity::users::ActiveModel;

pub struct UserConverter;

pub fn to_entity(user: User) -> ActiveModel {
    ActiveModel {
        id: Set(Uuid::from_u128(u128::from(user.id))),
        user_name: Set(user.personal_information.name.user_name()),
        email: Set(user.personal_information.email.as_str().to_string()),
        password: Set(user.authentication.password.as_str().to_string()),
    }
}
