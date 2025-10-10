//! Article deletion directive and handler.
//!
//! Implements the article deletion use case as a Directive (command).
//! Only the article author can delete their own article.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of article deletion.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteArticleDirective {
    pub slug: std::string::String,
    pub author_id: std::string::String,
}

impl hexser::Directive for DeleteArticleDirective {
    fn validate(&self) -> hexser::HexResult<()> {
        if self.slug.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Slug cannot be empty")
                    .with_field("slug")
            );
        }
        std::result::Result::Ok(())
    }
}

pub struct DeleteArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<DeleteArticleDirective> for DeleteArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    fn handle(&self, directive: DeleteArticleDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;

        let mut repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::article_repository::ArticleFilter::BySlug(directive.slug.clone());
        let article = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("Article", &directive.slug)
                    .with_next_step("Check the article slug and try again")
            })?;

        if article.author_id != directive.author_id {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Only the author can delete this article")
                    .with_field("author_id")
                    .with_next_step("Ensure you are logged in as the article author")
            );
        }

        let deleted = hexser::ports::repository::QueryRepository::delete_where(&mut *repo, &filter)?;

        if deleted == 0 {
            return std::result::Result::Err(
                hexser::Hexserror::adapter("E_DELETE", "Failed to delete article")
                    .with_next_step("Try again or contact support")
            );
        }

        std::result::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_delete_article_directive_validation() {
        let directive = super::DeleteArticleDirective {
            slug: std::string::String::from("test-article"),
            author_id: std::string::String::from("author1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_delete_article_directive_empty_slug() {
        let directive = super::DeleteArticleDirective {
            slug: std::string::String::from(""),
            author_id: std::string::String::from("author1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }
}
