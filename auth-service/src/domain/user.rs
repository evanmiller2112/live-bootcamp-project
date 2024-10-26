use std::error::Error;

// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct User {
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) requires_2FA: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2FA: bool) -> Self {
        User {
            email,
            password,
            requires_2FA,
        }
    }
}