//! Get profile query and handler.
//!
//! Implements the fetch user profile use case as a Query (read operation).
//! Returns public profile information including follow status.
//!
//! Revision History
//! - 2025-10-09T23:49:00Z @AI: Initial implementation of get profile query.

#[derive(hexser::HexQuery, std::clone::Clone, std::fmt::Debug)]
pub struct GetProfileQuery {
    pub username: std::string::String,
    pub requester_id: std::option::Option<std::string::String>,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ProfileResponse {
    pub username: std::string::String,
    pub bio: std::option::Option<std::string::String>,
    pub image: std::option::Option<std::string::String>,
    pub following: bool,
}

impl ProfileResponse {
    pub fn from_user(user: crate::domain::user::User, is_following: bool) -> Self {
        Self {
            username: user.username,
            bio: user.bio,
            image: user.image,
            following: is_following,
        }
    }
}

pub struct GetProfileHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::QueryHandler<GetProfileQuery, ProfileResponse> for GetProfileHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn handle(&self, query: GetProfileQuery) -> hexser::HexResult<ProfileResponse> {
        let repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::user_repository::UserFilter::ByUsername(query.username.clone());
        let user = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("User", &query.username)
                    .with_next_step("Check the username and try again")
            })?;

        let is_following = if let std::option::Option::Some(requester_id) = query.requester_id {
            let requester_filter = crate::ports::user_repository::UserFilter::ById(requester_id);
            if let std::option::Option::Some(requester) = hexser::ports::repository::QueryRepository::find_one(&*repo, &requester_filter)? {
                requester.is_following(&user.id)
            } else {
                false
            }
        } else {
            false
        };

        std::result::Result::Ok(ProfileResponse::from_user(user, is_following))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_profile_query_creation() {
        let query = super::GetProfileQuery {
            username: std::string::String::from("testuser"),
            requester_id: std::option::Option::None,
        };
        std::assert_eq!(query.username, "testuser");
    }

    #[test]
    fn test_profile_response_from_user() {
        let user = crate::domain::user::User::new(
            std::string::String::from("1"),
            std::string::String::from("test@example.com"),
            std::string::String::from("testuser"),
            std::string::String::from("hashed"),
        );
        let response = super::ProfileResponse::from_user(user, true);
        std::assert_eq!(response.username, "testuser");
        std::assert_eq!(response.following, true);
    }
}
