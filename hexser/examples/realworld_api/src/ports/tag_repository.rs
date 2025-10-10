//! Tag repository port definitions.
//!
//! Defines a simple repository interface for retrieving tags. Since tags are
//! value objects extracted from articles, this repository primarily supports
//! querying all unique tags in the system.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of TagRepository port.

pub trait TagRepository {
    fn find_all_tags(&self) -> hexser::HexResult<std::vec::Vec<std::string::String>>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tag_repository_trait_exists() {
        fn assert_tag_repo<T: super::TagRepository>() {}
    }
}
