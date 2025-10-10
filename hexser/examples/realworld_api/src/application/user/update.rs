//! Update user directive and handler.
//!
//! Implements updating user profile information including email, username,
//! password, bio, and image.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of update user directive.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateUserDirective {
    pub user_id: std::string::String,
    pub email: std::option::Option<std::string::String>,
    pub username: std::option::Option<std::string::String>,
    pub password: std::option::Option<std::string::String>,
    pub bio: std::option::Option<std::string::String>,
    pub image: std::option::Option<std::string::String>,
}

impl hexser::Directive for UpdateUserDirective {
    fn validate(&self) -> hexser::HexResult<()> {
        if let std::option::Option::Some(ref email) = self.email {
            if !email.contains('@') {
                return std::result::Result::Err(
                    hexser::Hexserror::validation("Invalid email format")
                        .with_field("email")
                        .with_next_step("Provide a valid email address")
                );
            }
        }
        if let std::option::Option::Some(ref username) = self.username {
            if username.is_empty() || username.len() < 3 {
                return std::result::Result::Err(
                    hexser::Hexserror::validation("Username must be at least 3 characters")
                        .with_field("username")
                );
            }
        }
        if let std::option::Option::Some(ref password) = self.password {
            if password.len() < 8 {
                return std::result::Result::Err(
                    hexser::Hexserror::validation("Password must be at least 8 characters")
                        .with_field("password")
                );
            }
        }
        std::result::Result::Ok(())
    }
}

pub struct UpdateUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<UpdateUserDirective> for UpdateUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn handle(&self, directive: UpdateUserDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;

        let mut repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::user_repository::UserFilter::ById(directive.user_id.clone());
        let mut user = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("User", &directive.user_id)
                    .with_next_step("Verify the user exists")
            })?;

        if let std::option::Option::Some(email) = directive.email {
            let email_filter = crate::ports::user_repository::UserFilter::ByEmail(email.clone());
            if let std::option::Option::Some(existing_user) = hexser::ports::repository::QueryRepository::find_one(&*repo, &email_filter)? {
                if existing_user.id != user.id {
                    return std::result::Result::Err(
                        hexser::Hexserror::conflict("Email already in use")
                            .with_field("email")
                            .with_next_step("Use a different email address")
                    );
                }
            }
            user.email = email;
        }

        if let std::option::Option::Some(username) = directive.username {
            let username_filter = crate::ports::user_repository::UserFilter::ByUsername(username.clone());
            if let std::option::Option::Some(existing_user) = hexser::ports::repository::QueryRepository::find_one(&*repo, &username_filter)? {
                if existing_user.id != user.id {
                    return std::result::Result::Err(
                        hexser::Hexserror::conflict("Username already taken")
                            .with_field("username")
                            .with_next_step("Choose a different username")
                    );
                }
            }
            user.username = username;
        }

        if let std::option::Option::Some(password) = directive.password {
            user.password_hash = Self::hash_password(&password);
        }

        if let std::option::Option::Some(bio) = directive.bio {
            user.bio = std::option::Option::Some(bio);
        }

        if let std::option::Option::Some(image) = directive.image {
            user.image = std::option::Option::Some(image);
        }

        hexser::ports::Repository::save(&mut *repo, user)?;
        std::result::Result::Ok(())
    }
}

impl<R> UpdateUserHandler<R>
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
    fn test_update_user_directive_validation() {
        let directive = super::UpdateUserDirective {
            user_id: std::string::String::from("user1"),
            email: std::option::Option::Some(std::string::String::from("new@example.com")),
            username: std::option::Option::None,
            password: std::option::Option::None,
            bio: std::option::Option::None,
            image: std::option::Option::None,
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_update_user_directive_invalid_email() {
        let directive = super::UpdateUserDirective {
            user_id: std::string::String::from("user1"),
            email: std::option::Option::Some(std::string::String::from("invalid-email")),
            username: std::option::Option::None,
            password: std::option::Option::None,
            bio: std::option::Option::None,
            image: std::option::Option::None,
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }

    #[test]
    fn test_update_user_directive_short_password() {
        let directive = super::UpdateUserDirective {
            user_id: std::string::String::from("user1"),
            email: std::option::Option::None,
            username: std::option::Option::None,
            password: std::option::Option::Some(std::string::String::from("short")),
            bio: std::option::Option::None,
            image: std::option::Option::None,
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }
}
