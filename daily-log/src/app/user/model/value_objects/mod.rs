#[path = "personal/mod.rs"]
mod personal;

#[path = "auth/Authentication.rs"]
mod authentication;

pub use authentication::Authentication;
pub use personal::{EmailAddress, PersonalInformation, UserName};
