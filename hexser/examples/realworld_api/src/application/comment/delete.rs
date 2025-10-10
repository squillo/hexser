//! Delete comment directive and handler.
//!
//! Implements the comment deletion use case as a Directive (command).
//! Only the comment author can delete their own comment.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of comment deletion.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteCommentDirective {
    pub comment_id: std::string::String,
    pub author_id: std::string::String,
}

impl hexser::Directive for DeleteCommentDirective {
    fn validate(&self) -> hexser::HexResult<()> {
        if self.comment_id.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Comment ID cannot be empty")
                    .with_field("comment_id")
            );
        }
        std::result::Result::Ok(())
    }
}

pub struct DeleteCommentHandler<R>
where
    R: crate::ports::comment_repository::CommentRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<DeleteCommentDirective> for DeleteCommentHandler<R>
where
    R: crate::ports::comment_repository::CommentRepository,
{
    fn handle(&self, directive: DeleteCommentDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;

        let mut repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::comment_repository::CommentFilter::ById(directive.comment_id.clone());
        let comment = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("Comment", &directive.comment_id)
                    .with_next_step("Check the comment ID and try again")
            })?;

        if comment.author_id != directive.author_id {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Only the author can delete this comment")
                    .with_field("author_id")
                    .with_next_step("Ensure you are logged in as the comment author")
            );
        }

        let deleted = hexser::ports::repository::QueryRepository::delete_where(&mut *repo, &filter)?;

        if deleted == 0 {
            return std::result::Result::Err(
                hexser::Hexserror::adapter("E_DELETE", "Failed to delete comment")
                    .with_next_step("Try again or contact support")
            );
        }

        std::result::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_delete_comment_directive_validation() {
        let directive = super::DeleteCommentDirective {
            comment_id: std::string::String::from("comment1"),
            author_id: std::string::String::from("user1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_delete_comment_directive_empty_id() {
        let directive = super::DeleteCommentDirective {
            comment_id: std::string::String::from(""),
            author_id: std::string::String::from("user1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }
}
