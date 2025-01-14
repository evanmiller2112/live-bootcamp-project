#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            requires_2fa: false,
        }
    }
}