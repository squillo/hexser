//! Add comment directive and handler.
//!
//! Implements the comment creation use case as a Directive (command).
//! Validates input and persists new comment via repository.
//!
//! Revision History
//! - 2025-10-09T23:49:00Z @AI: Initial implementation of add comment.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct AddCommentDirective {
    pub body: std::string::String,
    pub article_id: std::string::String,
    pub author_id: std::string::String,
}

impl hexser::Directive for AddCommentDirective {
    fn validate(&self) -> hexser::HexResult<()> {
        if self.body.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Comment body cannot be empty")
                    .with_field("body")
            );
        }
        if self.article_id.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Article ID cannot be empty")
                    .with_field("article_id")
            );
        }
        if self.author_id.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Author ID cannot be empty")
                    .with_field("author_id")
            );
        }
        std::result::Result::Ok(())
    }
}

pub struct AddCommentHandler<R>
where
    R: crate::ports::comment_repository::CommentRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<AddCommentDirective> for AddCommentHandler<R>
where
    R: crate::ports::comment_repository::CommentRepository,
{
    fn handle(&self, directive: AddCommentDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;

        let comment_id = std::format!("comment-{}", uuid::Uuid::new_v4());

        let comment = crate::domain::comment::Comment::new(
            comment_id,
            directive.body,
            directive.article_id,
            directive.author_id,
        );

        let mut repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        hexser::ports::Repository::save(&mut *repo, comment)?;
        std::result::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_comment_directive_validation() {
        let directive = super::AddCommentDirective {
            body: std::string::String::from("Great article!"),
            article_id: std::string::String::from("article1"),
            author_id: std::string::String::from("user1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_add_comment_directive_empty_body() {
        let directive = super::AddCommentDirective {
            body: std::string::String::from(""),
            article_id: std::string::String::from("article1"),
            author_id: std::string::String::from("user1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }
}
