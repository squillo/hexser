//! In-memory comment repository adapter implementation.
//!
//! Provides an in-memory implementation of the CommentRepository port for
//! storing and querying article comments.
//!
//! Revision History
//! - 2025-10-10T10:47:00Z @AI: Add HexAdapter derive macro for automatic registration and graph introspection.
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of in-memory comment adapter.

#[derive(hexser::HexAdapter, std::default::Default)]
pub struct InMemoryCommentRepository {
    comments: std::vec::Vec<crate::domain::comment::Comment>,
}

impl InMemoryCommentRepository {
    pub fn new() -> Self {
        Self {
            comments: std::vec::Vec::new(),
        }
    }

    fn matches_filter(comment: &crate::domain::comment::Comment, filter: &crate::ports::comment_repository::CommentFilter) -> bool {
        match filter {
            crate::ports::comment_repository::CommentFilter::ById(id) => &comment.id == id,
            crate::ports::comment_repository::CommentFilter::ByArticleId(article_id) => &comment.article_id == article_id,
            crate::ports::comment_repository::CommentFilter::ByAuthor(author_id) => &comment.author_id == author_id,
            crate::ports::comment_repository::CommentFilter::All => true,
        }
    }
}

impl hexser::ports::Repository<crate::domain::comment::Comment> for InMemoryCommentRepository {
    fn save(&mut self, comment: crate::domain::comment::Comment) -> hexser::HexResult<()> {
        if let std::option::Option::Some(pos) = self.comments.iter().position(|c| c.id == comment.id) {
            self.comments[pos] = comment;
        } else {
            self.comments.push(comment);
        }
        std::result::Result::Ok(())
    }
}

impl hexser::ports::repository::QueryRepository<crate::domain::comment::Comment> for InMemoryCommentRepository {
    type Filter = crate::ports::comment_repository::CommentFilter;
    type SortKey = crate::ports::comment_repository::CommentSortKey;

    fn find_one(&self, filter: &Self::Filter) -> hexser::HexResult<std::option::Option<crate::domain::comment::Comment>> {
        std::result::Result::Ok(
            self.comments
                .iter()
                .find(|c| Self::matches_filter(c, filter))
                .cloned()
        )
    }

    fn find(
        &self,
        filter: &Self::Filter,
        options: hexser::ports::repository::FindOptions<Self::SortKey>,
    ) -> hexser::HexResult<std::vec::Vec<crate::domain::comment::Comment>> {
        let mut results: std::vec::Vec<_> = self.comments
            .iter()
            .filter(|c| Self::matches_filter(c, filter))
            .cloned()
            .collect();

        if let std::option::Option::Some(sorts) = options.sort {
            for sort in sorts.into_iter().rev() {
                match (sort.key, sort.direction) {
                    (crate::ports::comment_repository::CommentSortKey::Id, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.id.cmp(&b.id));
                    }
                    (crate::ports::comment_repository::CommentSortKey::Id, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.id.cmp(&a.id));
                    }
                    (crate::ports::comment_repository::CommentSortKey::CreatedAt, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.created_at.cmp(&b.created_at));
                    }
                    (crate::ports::comment_repository::CommentSortKey::CreatedAt, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                    }
                }
            }
        }

        let offset = options.offset.unwrap_or(0) as usize;
        let limit = options.limit.map(|l| l as usize).unwrap_or(results.len());

        std::result::Result::Ok(
            results
                .into_iter()
                .skip(offset)
                .take(limit)
                .collect()
        )
    }

    fn delete_where(&mut self, filter: &Self::Filter) -> hexser::HexResult<u64> {
        let before = self.comments.len();
        self.comments.retain(|c| !Self::matches_filter(c, filter));
        std::result::Result::Ok((before - self.comments.len()) as u64)
    }
}

impl crate::ports::comment_repository::CommentRepository for InMemoryCommentRepository {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_save_and_find_comment() {
        let mut repo = super::InMemoryCommentRepository::new();
        let comment = crate::domain::comment::Comment::new(
            std::string::String::from("1"),
            std::string::String::from("Great article!"),
            std::string::String::from("article1"),
            std::string::String::from("user1"),
        );

        hexser::ports::Repository::save(&mut repo, comment.clone()).unwrap();

        let filter = crate::ports::comment_repository::CommentFilter::ById(std::string::String::from("1"));
        let found = hexser::ports::repository::QueryRepository::find_one(&repo, &filter).unwrap();
        std::assert!(found.is_some());
        std::assert_eq!(found.unwrap().body, "Great article!");
    }
}
