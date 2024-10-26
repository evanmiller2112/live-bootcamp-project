use std::collections::HashMap;
use std::error::Error;
use crate::domain::{
    user::{User},
};
use crate::services::hashmap_user_store::UserStoreError::UserNotFound;
use tokio;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}



// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            // make new user
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    pub fn get_user(&self, email: String) -> Result<&User, UserStoreError> {
        match self.users.get(&email) {
            Some(user) => Ok(user),
            None => Err(UserStoreError::UserNotFound)
        }
    }
    pub fn validate_user(&self, email: String, password: String) -> Result<(), UserStoreError> {
        match self.users.get(&email) {
            Some(user) => if password == user.password { 
                Ok(()) 
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
            None => Err(UserStoreError::UserNotFound)
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "test@test.com".to_owned(),
            password: "password123".to_owned(),
            requires_2FA: true,
        };

        let result = user_store.add_user(user.clone());
        assert!(result.is_ok());

        let result = user_store.add_user(user);
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store= HashmapUserStore::default();
        let user = User {
            email: "test@test.com".to_owned(),
            password: "password123".to_owned(),
            requires_2FA: true,
        };
        let result = user_store.add_user(user.clone());
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "test@test.com".to_owned(),
            password: "password123".to_owned(),
            requires_2FA: false,
        };
        user_store.add_user(user.clone());
        
        let result = user_store.validate_user(
            "test@test.com".to_owned(), 
            "password123".to_owned());
        assert!(result.is_ok());
        let result = user_store.validate_user(
            "test@test.com".to_owned(), 
            "password1234".to_owned());
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));
    }
}