//! Comment repository port definitions.
//!
//! Defines the repository interface for Comment aggregate persistence operations.
//! Includes domain-owned Filter and SortKey enums for type-safe querying.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of CommentRepository port.

#[derive(hexser::HexPort, std::clone::Clone, std::fmt::Debug)]
pub enum CommentFilter {
    ById(std::string::String),
    ByArticleId(std::string::String),
    ByAuthor(std::string::String),
    All,
}

#[derive(hexser::HexPort, std::clone::Clone, std::cmp::PartialEq, std::cmp::Eq, std::fmt::Debug)]
pub enum CommentSortKey {
    Id,
    CreatedAt,
}

pub trait CommentRepository:
    hexser::ports::Repository<crate::domain::comment::Comment>
    + hexser::ports::repository::QueryRepository<
        crate::domain::comment::Comment,
        Filter = CommentFilter,
        SortKey = CommentSortKey,
    >
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_comment_filter_creation() {
        let filter = super::CommentFilter::ByArticleId(std::string::String::from("article1"));
        std::assert!(matches!(filter, super::CommentFilter::ByArticleId(_)));
    }

    #[test]
    fn test_comment_sort_key_equality() {
        std::assert_eq!(super::CommentSortKey::CreatedAt, super::CommentSortKey::CreatedAt);
        std::assert_ne!(super::CommentSortKey::Id, super::CommentSortKey::CreatedAt);
    }
}
