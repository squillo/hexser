//! Get all tags query and handler.
//!
//! Implements the fetch all tags use case as a Query (read operation).
//! Returns all unique tags from articles in the system.
//!
//! Revision History
//! - 2025-10-09T23:49:00Z @AI: Initial implementation of get all tags query.

#[derive(hexser::HexQuery, std::clone::Clone, std::fmt::Debug)]
pub struct GetAllTagsQuery {}

pub struct GetAllTagsHandler<R>
where
    R: crate::ports::tag_repository::TagRepository,
{
    pub repository: R,
}

impl<R> hexser::QueryHandler<GetAllTagsQuery, std::vec::Vec<std::string::String>> for GetAllTagsHandler<R>
where
    R: crate::ports::tag_repository::TagRepository,
{
    fn handle(&self, _query: GetAllTagsQuery) -> hexser::HexResult<std::vec::Vec<std::string::String>> {
        self.repository.find_all_tags()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_all_tags_query_creation() {
        let query = super::GetAllTagsQuery {};
        std::assert!(true);
    }
}
