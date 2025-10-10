//! Article update directive and handler.
//!
//! Implements the article update use case as a Directive (command).
//! Allows the author to update title, description, and body.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of article update.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UpdateArticleDirective {
    pub slug: std::string::String,
    pub author_id: std::string::String,
    pub title: std::option::Option<std::string::String>,
    pub description: std::option::Option<std::string::String>,
    pub body: std::option::Option<std::string::String>,
}

impl hexser::Directive for UpdateArticleDirective {
    fn validate(&self) -> hexser::HexResult<()> {
        if let std::option::Option::Some(ref title) = self.title {
            if title.is_empty() {
                return std::result::Result::Err(
                    hexser::Hexserror::validation("Title cannot be empty")
                        .with_field("title")
                );
            }
        }
        if let std::option::Option::Some(ref description) = self.description {
            if description.is_empty() {
                return std::result::Result::Err(
                    hexser::Hexserror::validation("Description cannot be empty")
                        .with_field("description")
                );
            }
        }
        if let std::option::Option::Some(ref body) = self.body {
            if body.is_empty() {
                return std::result::Result::Err(
                    hexser::Hexserror::validation("Body cannot be empty")
                        .with_field("body")
                );
            }
        }
        std::result::Result::Ok(())
    }
}

pub struct UpdateArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<UpdateArticleDirective> for UpdateArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    fn handle(&self, directive: UpdateArticleDirective) -> hexser::HexResult<()> {
        hexser::Directive::validate(&directive)?;

        let mut repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::article_repository::ArticleFilter::BySlug(directive.slug.clone());
        let mut article = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("Article", &directive.slug)
                    .with_next_step("Check the article slug and try again")
            })?;

        if article.author_id != directive.author_id {
            return std::result::Result::Err(
                hexser::Hexserror::validation("Only the author can update this article")
                    .with_field("author_id")
                    .with_next_step("Ensure you are logged in as the article author")
            );
        }

        article.update(directive.title, directive.description, directive.body);

        hexser::ports::Repository::save(&mut *repo, article)?;
        std::result::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_update_article_directive_validation() {
        let directive = super::UpdateArticleDirective {
            slug: std::string::String::from("test-article"),
            author_id: std::string::String::from("author1"),
            title: std::option::Option::Some(std::string::String::from("Updated Title")),
            description: std::option::Option::None,
            body: std::option::Option::None,
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_update_article_directive_empty_title() {
        let directive = super::UpdateArticleDirective {
            slug: std::string::String::from("test-article"),
            author_id: std::string::String::from("author1"),
            title: std::option::Option::Some(std::string::String::from("")),
            description: std::option::Option::None,
            body: std::option::Option::None,
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }

    #[test]
    fn test_update_article_directive_no_updates() {
        let directive = super::UpdateArticleDirective {
            slug: std::string::String::from("test-article"),
            author_id: std::string::String::from("author1"),
            title: std::option::Option::None,
            description: std::option::Option::None,
            body: std::option::Option::None,
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }
}
