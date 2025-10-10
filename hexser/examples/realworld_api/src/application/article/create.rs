//! Article creation directive and handler.
//!
//! Implements the article creation use case as a Directive (command).
//! Validates input and persists new article via repository.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of article creation.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateArticleDirective {
    pub title: std::string::String,
    pub description: std::string::String,
    pub body: std::string::String,
    pub author_id: std::string::String,
    pub tags: std::vec::Vec<std::string::String>,
}

impl hexser::Directive for CreateArticleDirective {
    fn validate(&self) -> hexser::HexResult<()> {
        if self.title.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Title cannot be empty")
                    .with_field("title")
            );
        }
        if self.description.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Description cannot be empty")
                    .with_field("description")
            );
        }
        if self.body.is_empty() {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Body cannot be empty")
                    .with_field("body")
            );
        }
        std::result::Result::Ok(())
    }
}

pub struct CreateArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<CreateArticleDirective> for CreateArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    fn handle(&self, directive: CreateArticleDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;

        let article_id = std::format!("article-{}", uuid::Uuid::new_v4());

        let article = crate::domain::article::Article::new(
            article_id,
            directive.title,
            directive.description,
            directive.body,
            directive.author_id,
            directive.tags,
        );

        let mut repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        hexser::ports::Repository::save(&mut *repo, article)?;
        std::result::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_article_directive_validation() {
        let directive = super::CreateArticleDirective {
            title: std::string::String::from("Test Article"),
            description: std::string::String::from("A test description"),
            body: std::string::String::from("Article body content"),
            author_id: std::string::String::from("author1"),
            tags: vec![std::string::String::from("rust")],
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_create_article_directive_empty_title() {
        let directive = super::CreateArticleDirective {
            title: std::string::String::from(""),
            description: std::string::String::from("A test description"),
            body: std::string::String::from("Article body content"),
            author_id: std::string::String::from("author1"),
            tags: vec![],
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }
}
