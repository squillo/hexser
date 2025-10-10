//! Get comments query and handler.
//!
//! Implements the fetch comments use case as a Query (read operation).
//! Returns all comments for a specific article.
//!
//! Revision History
//! - 2025-10-09T23:49:00Z @AI: Initial implementation of get comments query.

#[derive(hexser::HexQuery, std::clone::Clone, std::fmt::Debug)]
pub struct GetCommentsQuery {
    pub article_id: std::string::String,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CommentResponse {
    pub id: std::string::String,
    pub body: std::string::String,
    pub author_id: std::string::String,
    pub created_at: std::string::String,
}

impl CommentResponse {
    pub fn from_comment(comment: crate::domain::comment::Comment) -> Self {
        Self {
            id: comment.id,
            body: comment.body,
            author_id: comment.author_id,
            created_at: comment.created_at,
        }
    }
}

pub struct GetCommentsHandler<R>
where
    R: crate::ports::comment_repository::CommentRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::QueryHandler<GetCommentsQuery, std::vec::Vec<CommentResponse>> for GetCommentsHandler<R>
where
    R: crate::ports::comment_repository::CommentRepository,
{
    fn handle(&self, query: GetCommentsQuery) -> hexser::HexResult<std::vec::Vec<CommentResponse>> {
        let repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::comment_repository::CommentFilter::ByArticleId(query.article_id);
        let comments = hexser::ports::repository::QueryRepository::find(
            &*repo,
            &filter,
            hexser::ports::repository::FindOptions::default()
        )?;

        std::result::Result::Ok(
            comments.into_iter()
                .map(CommentResponse::from_comment)
                .collect()
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_comments_query_creation() {
        let query = super::GetCommentsQuery {
            article_id: std::string::String::from("article1"),
        };
        std::assert_eq!(query.article_id, "article1");
    }

    #[test]
    fn test_comment_response_from_comment() {
        let comment = crate::domain::comment::Comment::new(
            std::string::String::from("1"),
            std::string::String::from("Great!"),
            std::string::String::from("article1"),
            std::string::String::from("user1"),
        );
        let response = super::CommentResponse::from_comment(comment);
        std::assert_eq!(response.id, "1");
        std::assert_eq!(response.body, "Great!");
    }
}
