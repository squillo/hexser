//! Defines the core Comment entity for the domain layer.
//!
//! The Comment entity represents a user comment on an article, containing
//! the comment body, references to the article and author, and timestamp information.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of Comment entity.

#[derive(hexser::HexDomain, hexser::HexEntity, std::clone::Clone, std::fmt::Debug)]
pub struct Comment {
    pub id: std::string::String,
    pub body: std::string::String,
    pub article_id: std::string::String,
    pub author_id: std::string::String,
    pub created_at: std::string::String,
}

impl Comment {
    pub fn new(
        id: std::string::String,
        body: std::string::String,
        article_id: std::string::String,
        author_id: std::string::String,
    ) -> Self {
        Self {
            id,
            body,
            article_id,
            author_id,
            created_at: Self::current_timestamp(),
        }
    }

    fn current_timestamp() -> std::string::String {
        std::string::String::from("2025-10-09T22:14:00Z")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_comment_creation() {
        let comment = super::Comment::new(
            std::string::String::from("1"),
            std::string::String::from("Great article!"),
            std::string::String::from("article1"),
            std::string::String::from("user1"),
        );
        std::assert_eq!(comment.id, "1");
        std::assert_eq!(comment.body, "Great article!");
        std::assert_eq!(comment.article_id, "article1");
        std::assert_eq!(comment.author_id, "user1");
    }
}
