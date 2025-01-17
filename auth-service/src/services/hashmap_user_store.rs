use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::User;
use crate::domain::UserStoreError;
use crate::domain::UserStore;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}


impl HashmapUserStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[async_trait::async_trait]
impl UserStore for HashmapUserStore {

    
     async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

     async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        if self.users.contains_key(email) {
            Ok(self.users.get(email).unwrap().clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

     async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        if self.users.contains_key(email) {
            if self.users.get(email).unwrap().password == password {
                Ok(())
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut hashmap_user_store = HashmapUserStore::default();
        let user = User {
            email: get_random_email().to_string(),
            password: "test123badpass".to_string(),
            requires_2fa: true,
        };
        hashmap_user_store.add_user(user.clone()).await.unwrap();
        assert_eq!(hashmap_user_store.users.get(&user.email).unwrap(), &user);
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut hashmap_user_store = HashmapUserStore::default();
        let user = User {
            email: get_random_email(),
            password: "test123badpass".to_string(),
            requires_2fa: true,
        };
        hashmap_user_store.add_user(user.clone()).await.unwrap();
        assert_eq!(hashmap_user_store.get_user(&user.email).await.unwrap(), user);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut hashmap_user_store = HashmapUserStore::default();
        let user = User {
            email: get_random_email(),
            password: "test123badpass".to_string(),
            requires_2fa: true,
        };
        hashmap_user_store.add_user(user.clone()).await.unwrap();
        assert_eq!(hashmap_user_store.validate_user(
            &user.email,
            "test123badpass").await.unwrap(),
            ());
        assert_eq!(hashmap_user_store.validate_user(
            &user.email,
            "test123badpass2").await.unwrap_err(),
            UserStoreError::InvalidCredentials);
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}