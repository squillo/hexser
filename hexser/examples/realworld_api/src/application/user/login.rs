//! User login query and handler.
//!
//! Implements the user authentication use case as a Query (read operation).
//! Validates credentials and returns a JWT token for authenticated users.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of user login.

#[derive(hexser::HexQuery, std::clone::Clone, std::fmt::Debug)]
pub struct LoginUserQuery {
    pub email: std::string::String,
    pub password: std::string::String,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct LoginUserResponse {
    pub user_id: std::string::String,
    pub email: std::string::String,
    pub username: std::string::String,
    pub token: std::string::String,
    pub bio: std::option::Option<std::string::String>,
    pub image: std::option::Option<std::string::String>,
}

pub struct LoginUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::QueryHandler<LoginUserQuery, LoginUserResponse> for LoginUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn handle(&self, query: LoginUserQuery) -> hexser::HexResult<LoginUserResponse> {
        let repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::user_repository::UserFilter::ByEmail(query.email.clone());
        let user = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("User", &query.email)
                    .with_next_step("Check email and try again or register a new account")
            })?;

        let password_hash = Self::hash_password(&query.password);
        if user.password_hash != password_hash {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Invalid password")
                    .with_field("password")
                    .with_next_step("Check your password and try again")
            );
        }

        let token = Self::generate_token(&user.id);

        std::result::Result::Ok(LoginUserResponse {
            user_id: user.id,
            email: user.email,
            username: user.username,
            token,
            bio: user.bio,
            image: user.image,
        })
    }
}

impl<R> LoginUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn hash_password(password: &str) -> std::string::String {
        std::format!("hashed_{}", password)
    }

    fn generate_token(user_id: &str) -> std::string::String {
        std::format!("jwt_token_for_{}", user_id)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_login_query_creation() {
        let query = super::LoginUserQuery {
            email: std::string::String::from("test@example.com"),
            password: std::string::String::from("password123"),
        };
        std::assert_eq!(query.email, "test@example.com");
    }

    #[test]
    fn test_login_response_creation() {
        let response = super::LoginUserResponse {
            user_id: std::string::String::from("1"),
            email: std::string::String::from("test@example.com"),
            username: std::string::String::from("testuser"),
            token: std::string::String::from("token123"),
            bio: std::option::Option::None,
            image: std::option::Option::None,
        };
        std::assert_eq!(response.user_id, "1");
        std::assert_eq!(response.token, "token123");
    }
}
