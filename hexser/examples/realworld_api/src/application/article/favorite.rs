//! Article favorite/unfavorite directives and handlers.
//!
//! Implements marking and unmarking articles as favorites.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of favorite/unfavorite directives.

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct FavoriteArticleDirective {
    pub slug: std::string::String,
    pub user_id: std::string::String,
}

impl hexser::Directive for FavoriteArticleDirective {
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

pub struct FavoriteArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<FavoriteArticleDirective> for FavoriteArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    fn handle(&self, directive: FavoriteArticleDirective) -> hexser::HexResult<()> {
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

        article.favorite(directive.user_id);

        hexser::ports::Repository::save(&mut *repo, article)?;
        std::result::Result::Ok(())
    }
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct UnfavoriteArticleDirective {
    pub slug: std::string::String,
    pub user_id: std::string::String,
}

impl hexser::Directive for UnfavoriteArticleDirective {
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

pub struct UnfavoriteArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::DirectiveHandler<UnfavoriteArticleDirective> for UnfavoriteArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    fn handle(&self, directive: UnfavoriteArticleDirective) -> hexser::HexResult<()> {
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

        article.unfavorite(&directive.user_id);

        hexser::ports::Repository::save(&mut *repo, article)?;
        std::result::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_favorite_article_directive_validation() {
        let directive = super::FavoriteArticleDirective {
            slug: std::string::String::from("test-article"),
            user_id: std::string::String::from("user1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_favorite_article_directive_empty_slug() {
        let directive = super::FavoriteArticleDirective {
            slug: std::string::String::from(""),
            user_id: std::string::String::from("user1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }

    #[test]
    fn test_unfavorite_article_directive_validation() {
        let directive = super::UnfavoriteArticleDirective {
            slug: std::string::String::from("test-article"),
            user_id: std::string::String::from("user1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_ok());
    }

    #[test]
    fn test_unfavorite_article_directive_empty_slug() {
        let directive = super::UnfavoriteArticleDirective {
            slug: std::string::String::from(""),
            user_id: std::string::String::from("user1"),
        };
        std::assert!(hexser::Directive::validate(&directive).is_err());
    }
}
