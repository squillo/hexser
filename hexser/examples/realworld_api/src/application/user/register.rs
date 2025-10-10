//! User registration directive and handler.
//!
//! Implements the user registration use case as a Directive (command).
//! Validates input, hashes password, and persists new user via repository.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of user registration.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RegisterUserDirective {
    pub email: std::string::String,
    pub username: std::string::String,
    pub password: std::string::String,
}

impl hexser::Directive for RegisterUserDirective {
    fn validate(&self) -> hexser::HexResult<()> {
        if !self.email.contains('@') {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Invalid email format")
                    .with_field("email")
                    .with_next_step("Provide a valid email address")
            );
        }
        if self.username.is_empty() || self.username.len() < 3 {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Username must be at least 3 characters")
                    .with_field("username")
            );
        }
        if self.password.len() < 8 {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Password must be at least 8 characters")
                    .with_field("password")
            );
        }
        std::result::Result::Ok(())
    }
}

pub struct RegisterUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<RegisterUserDirective> for RegisterUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn handle(&self, directive: RegisterUserDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;

        let mut repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let email_filter = crate::ports::user_repository::UserFilter::ByEmail(directive.email.clone());
        if hexser::ports::repository::QueryRepository::find_one(&*repo, &email_filter)?.is_some() {
            return std::result::Result::Err(
                hexser::Hexserror::conflict("Email already registered")
                    .with_field("email")
                    .with_next_step("Use a different email or login with existing account")
            );
        }

        let username_filter = crate::ports::user_repository::UserFilter::ByUsername(directive.username.clone());
        if hexser::ports::repository::QueryRepository::find_one(&*repo, &username_filter)?.is_some() {
            return std::result::Result::Err(
                hexser::Hexserror::conflict("Username already taken")
                    .with_field("username")
                    .with_next_step("Choose a different username")
            );
        }

        let user_id = std::format!("user-{}", uuid::Uuid::new_v4());
        let password_hash = Self::hash_password(&directive.password);

        let user = crate::domain::user::User::new(
            user_id,
            directive.email,
            directive.username,
            password_hash,
        );

        hexser::ports::Repository::save(&mut *repo, user)?;
        std::result::Result::Ok(())
    }
}

impl<R> RegisterUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn hash_password(password: &str) -> std::string::String {
        std::format!("hashed_{}", password)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_register_directive_validation() {
        let directive = super::RegisterUserDirective {
            email: std::string::String::from("test@example.com"),
            username: std::string::String::from("testuser"),
            password: std::string::String::from("password123"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_register_directive_invalid_email() {
        let directive = super::RegisterUserDirective {
            email: std::string::String::from("invalid-email"),
            username: std::string::String::from("testuser"),
            password: std::string::String::from("password123"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }

    #[test]
    fn test_register_directive_short_username() {
        let directive = super::RegisterUserDirective {
            email: std::string::String::from("test@example.com"),
            username: std::string::String::from("ab"),
            password: std::string::String::from("password123"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }

    #[test]
    fn test_register_directive_short_password() {
        let directive = super::RegisterUserDirective {
            email: std::string::String::from("test@example.com"),
            username: std::string::String::from("testuser"),
            password: std::string::String::from("short"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }
}
