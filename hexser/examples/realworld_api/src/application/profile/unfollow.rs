//! Unfollow user directive and handler.
//!
//! Implements the unfollow user use case as a Directive (command).
//! Allows a user to unfollow another user's profile.
//!
//! Revision History
//! - 2025-10-09T23:49:00Z @AI: Initial implementation of unfollow user.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UnfollowUserDirective {
    pub follower_id: std::string::String,
    pub followee_username: std::string::String,
}

impl hexser::Directive for UnfollowUserDirective {
    fn validate(&self) -> hexser::HexResult<()> {
        if self.follower_id.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Follower ID cannot be empty")
                    .with_field("follower_id")
            );
        }
        if self.followee_username.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Followee username cannot be empty")
                    .with_field("followee_username")
            );
        }
        std::result::Result::Ok(())
    }
}

pub struct UnfollowUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<UnfollowUserDirective> for UnfollowUserHandler<R>
where
    R: crate::ports::user_repository::UserRepository,
{
    fn handle(&self, directive: UnfollowUserDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;

        let mut repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let follower_filter = crate::ports::user_repository::UserFilter::ById(directive.follower_id.clone());
        let mut follower = hexser::ports::repository::QueryRepository::find_one(&*repo, &follower_filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("User", &directive.follower_id)
                    .with_next_step("Verify the follower user ID exists")
            })?;

        let followee_filter = crate::ports::user_repository::UserFilter::ByUsername(directive.followee_username.clone());
        let followee = hexser::ports::repository::QueryRepository::find_one(&*repo, &followee_filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("User", &directive.followee_username)
                    .with_next_step("Check the username and try again")
            })?;

        follower.unfollow(&followee.id);
        hexser::ports::Repository::save(&mut *repo, follower)?;

        std::result::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unfollow_user_directive_validation() {
        let directive = super::UnfollowUserDirective {
            follower_id: std::string::String::from("user1"),
            followee_username: std::string::String::from("user2"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_unfollow_user_directive_empty_followee() {
        let directive = super::UnfollowUserDirective {
            follower_id: std::string::String::from("user1"),
            followee_username: std::string::String::from(""),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }
}
