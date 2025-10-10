//! Get current user query and handler.
//!
//! Implements retrieving the currently authenticated user's information.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of get current user query.

#[derive(hexser::HexQuery, std::clone::Clone, std::fmt::Debug)]
pub struct GetCurrentUserQuery {
    pub user_id: std::string::String,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetCurrentUserResponse {
    pub user_id: std::string::String,
    pub email: std::string::String,
    pub username: std::string::String,
    pub token: std::string::String,
    pub bio: std::option::Option<std::string::String>,
    pub image: std::option::Option<std::string::String>,
}

pub struct GetCurrentUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::QueryHandler<GetCurrentUserQuery, GetCurrentUserResponse> for GetCurrentUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn handle(&self, query: GetCurrentUserQuery) -> hexser::HexResult<GetCurrentUserResponse> {
        let repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::user_repository::UserFilter::ById(query.user_id.clone());
        let user = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("User", &query.user_id)
                    .with_next_step("Verify authentication token is valid")
            })?;

        let token = Self::generate_token(&user.id);

        std::result::Result::Ok(GetCurrentUserResponse {
            user_id: user.id,
            email: user.email,
            username: user.username,
            token,
            bio: user.bio,
            image: user.image,
        })
    }
}

impl<R> GetCurrentUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn generate_token(user_id: &str) -> std::string::String {
        std::format!("jwt_token_for_{}", user_id)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_current_user_query_creation() {
        let query = super::GetCurrentUserQuery {
            user_id: std::string::String::from("user1"),
        };
        std::assert_eq!(query.user_id, "user1");
    }

    #[test]
    fn test_get_current_user_response_creation() {
        let response = super::GetCurrentUserResponse {
            user_id: std::string::String::from("1"),
            email: std::string::String::from("test@example.com"),
            username: std::string::String::from("testuser"),
            token: std::string::String::from("token123"),
            bio: std::option::Option::Some(std::string::String::from("My bio")),
            image: std::option::Option::None,
        };
        std::assert_eq!(response.username, "testuser");
        std::assert!(response.bio.is_some());
    }
}
