//! Get single article query and handler.
//!
//! Implements retrieving a single article by slug as a Query.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of get article query.

#[derive(hexser::HexQuery, std::clone::Clone, std::fmt::Debug)]
pub struct GetArticleQuery {
    pub slug: std::string::String,
    pub requester_id: std::option::Option<std::string::String>,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetArticleResponse {
    pub id: std::string::String,
    pub slug: std::string::String,
    pub title: std::string::String,
    pub description: std::string::String,
    pub body: std::string::String,
    pub author_id: std::string::String,
    pub tags: std::vec::Vec<std::string::String>,
    pub favorites_count: usize,
    pub favorited: bool,
    pub created_at: std::string::String,
    pub updated_at: std::string::String,
}

pub struct GetArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::QueryHandler<GetArticleQuery, GetArticleResponse> for GetArticleHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    fn handle(&self, query: GetArticleQuery) -> hexser::HexResult<GetArticleResponse> {
        let repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::article_repository::ArticleFilter::BySlug(query.slug.clone());
        let article = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)?
            .ok_or_else(|| {
                hexser::Hexserror::not_found("Article", &query.slug)
                    .with_next_step("Check the article slug and try again")
            })?;

        let favorited = if let std::option::Option::Some(user_id) = query.requester_id {
            article.is_favorited_by(&user_id)
        } else {
            false
        };

        std::result::Result::Ok(GetArticleResponse {
            id: article.id,
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            author_id: article.author_id,
            tags: article.tags,
            favorites_count: article.favorited_by.len(),
            favorited,
            created_at: article.created_at,
            updated_at: article.updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_article_query_creation() {
        let query = super::GetArticleQuery {
            slug: std::string::String::from("test-article"),
            requester_id: std::option::Option::None,
        };
        std::assert_eq!(query.slug, "test-article");
    }

    #[test]
    fn test_get_article_response_creation() {
        let response = super::GetArticleResponse {
            id: std::string::String::from("1"),
            slug: std::string::String::from("test-article"),
            title: std::string::String::from("Test Article"),
            description: std::string::String::from("Description"),
            body: std::string::String::from("Body"),
            author_id: std::string::String::from("author1"),
            tags: vec![std::string::String::from("rust")],
            favorites_count: 5,
            favorited: true,
            created_at: std::string::String::from("2025-10-10T00:00:00Z"),
            updated_at: std::string::String::from("2025-10-10T00:00:00Z"),
        };
        std::assert_eq!(response.favorites_count, 5);
        std::assert!(response.favorited);
    }
}
