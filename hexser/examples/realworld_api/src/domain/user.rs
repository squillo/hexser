//! Defines the core User entity for the domain layer.
//!
//! The User entity represents a registered user in the system, containing their
//! identity, authentication details, and profile information. Users can follow
//! other users and favorite articles.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of User entity.

#[derive(hexser::HexDomain, hexser::HexEntity, std::clone::Clone, std::fmt::Debug)]
pub struct User {
    pub id: std::string::String,
    pub email: std::string::String,
    pub username: std::string::String,
    pub password_hash: std::string::String,
    pub bio: std::option::Option<std::string::String>,
    pub image: std::option::Option<std::string::String>,
    pub followed_users: std::vec::Vec<std::string::String>,
}

impl User {
    pub fn new(
        id: std::string::String,
        email: std::string::String,
        username: std::string::String,
        password_hash: std::string::String,
    ) -> Self {
        Self {
            id,
            email,
            username,
            password_hash,
            bio: std::option::Option::None,
            image: std::option::Option::None,
            followed_users: std::vec::Vec::new(),
        }
    }

    pub fn follow(&mut self, user_id: std::string::String) {
        if !self.followed_users.contains(&user_id) {
            self.followed_users.push(user_id);
        }
    }

    pub fn unfollow(&mut self, user_id: &str) {
        self.followed_users.retain(|id| id != user_id);
    }

    pub fn is_following(&self, user_id: &str) -> bool {
        self.followed_users.contains(&std::string::String::from(user_id))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_user_creation() {
        let user = super::User::new(
            std::string::String::from("1"),
            std::string::String::from("test@example.com"),
            std::string::String::from("testuser"),
            std::string::String::from("hashed_password"),
        );
        std::assert_eq!(user.id, "1");
        std::assert_eq!(user.email, "test@example.com");
        std::assert_eq!(user.username, "testuser");
    }

    #[test]
    fn test_user_follow() {
        let mut user = super::User::new(
            std::string::String::from("1"),
            std::string::String::from("test@example.com"),
            std::string::String::from("testuser"),
            std::string::String::from("hashed_password"),
        );
        user.follow(std::string::String::from("2"));
        std::assert!(user.is_following("2"));
        std::assert!(!user.is_following("3"));
    }

    #[test]
    fn test_user_unfollow() {
        let mut user = super::User::new(
            std::string::String::from("1"),
            std::string::String::from("test@example.com"),
            std::string::String::from("testuser"),
            std::string::String::from("hashed_password"),
        );
        user.follow(std::string::String::from("2"));
        std::assert!(user.is_following("2"));
        user.unfollow("2");
        std::assert!(!user.is_following("2"));
    }
}
