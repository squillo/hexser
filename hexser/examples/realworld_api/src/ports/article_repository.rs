//! Article repository port definitions.
//!
//! Defines the repository interface for Article aggregate persistence operations.
//! Includes domain-owned Filter and SortKey enums for type-safe querying with
//! support for filtering by tag, author, favorited users, and feed generation.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of ArticleRepository port.

#[derive(hexser::HexPort, std::clone::Clone, std::fmt::Debug)]
pub enum ArticleFilter {
    ById(std::string::String),
    BySlug(std::string::String),
    ByTag(std::string::String),
    ByAuthor(std::string::String),
    ByFavoritedBy(std::string::String),
    FeedForUser(std::string::String),
    And(std::vec::Vec<ArticleFilter>),
    All,
}

#[derive(hexser::HexPort, std::clone::Clone, std::cmp::PartialEq, std::cmp::Eq, std::fmt::Debug)]
pub enum ArticleSortKey {
    Id,
    Title,
    CreatedAt,
    UpdatedAt,
    FavoritesCount,
}

pub trait ArticleRepository:
    hexser::ports::Repository<crate::domain::article::Article>
    + hexser::ports::repository::QueryRepository<
        crate::domain::article::Article,
        Filter = ArticleFilter,
        SortKey = ArticleSortKey,
    >
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_article_filter_creation() {
        let filter = super::ArticleFilter::BySlug(std::string::String::from("test-article"));
        std::assert!(matches!(filter, super::ArticleFilter::BySlug(_)));
    }

    #[test]
    fn test_article_filter_and() {
        let filter = super::ArticleFilter::And(vec![
            super::ArticleFilter::ByTag(std::string::String::from("rust")),
            super::ArticleFilter::ByAuthor(std::string::String::from("author1")),
        ]);
        std::assert!(matches!(filter, super::ArticleFilter::And(_)));
    }

    #[test]
    fn test_article_sort_key_equality() {
        std::assert_eq!(super::ArticleSortKey::CreatedAt, super::ArticleSortKey::CreatedAt);
        std::assert_ne!(super::ArticleSortKey::CreatedAt, super::ArticleSortKey::UpdatedAt);
    }
}
