#[path = "personal/mod.rs"]
mod personal;

#[path = "auth/authentication.rs"]
mod authentication;

pub use authentication::Authentication;
pub use personal::{EmailAddress, PersonalInformation, UserName};
