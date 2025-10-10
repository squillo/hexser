//! User repository port definitions.
//!
//! Defines the repository interface for User aggregate persistence operations.
//! Includes domain-owned Filter and SortKey enums for type-safe querying.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of UserRepository port.

#[derive(hexser::HexPort, std::clone::Clone, std::fmt::Debug)]
pub enum UserFilter {
    ById(std::string::String),
    ByEmail(std::string::String),
    ByUsername(std::string::String),
    All,
}

#[derive(hexser::HexPort, std::clone::Clone, std::cmp::PartialEq, std::cmp::Eq, std::fmt::Debug)]
pub enum UserSortKey {
    Id,
    Email,
    Username,
    CreatedAt,
}

pub trait UserRepository:
    hexser::ports::Repository<crate::domain::user::User>
    + hexser::ports::repository::QueryRepository<
        crate::domain::user::User,
        Filter = UserFilter,
        SortKey = UserSortKey,
    >
{
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_user_filter_creation() {
        let filter = super::UserFilter::ById(std::string::String::from("1"));
        std::assert!(matches!(filter, super::UserFilter::ById(_)));
    }

    #[test]
    fn test_user_sort_key_equality() {
        std::assert_eq!(super::UserSortKey::Email, super::UserSortKey::Email);
        std::assert_ne!(super::UserSortKey::Email, super::UserSortKey::Username);
    }
}
