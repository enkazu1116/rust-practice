
pub struct UserConverter;

pub fn to_entity(user: User) -> UserEntity {
    UserEntity {
        id: user.id,
        user_name: user.personal_information.name.user_name(),
        email: user.personal_information.email.as_str().to_string(),
        password: user.authentication.password.as_str().to_string(),
    }
}