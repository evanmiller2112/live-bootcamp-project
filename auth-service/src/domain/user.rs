#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email: email,
            password: password,
            requires_2fa: false,
        }
    }
}