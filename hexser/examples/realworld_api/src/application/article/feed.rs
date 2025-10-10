//! Article feed query and handler.
//!
//! Implements the article feed use case as a Query, returning articles
//! from users that the requester follows. Supports pagination.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of article feed query.

#[derive(hexser::HexQuery, std::clone::Clone, std::fmt::Debug)]
pub struct GetArticleFeedQuery {
    pub user_id: std::string::String,
    pub limit: std::option::Option<u32>,
    pub offset: std::option::Option<u64>,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ArticleFeedResponse {
    pub articles: std::vec::Vec<FeedArticle>,
    pub articles_count: usize,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct FeedArticle {
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

pub struct GetArticleFeedHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::QueryHandler<GetArticleFeedQuery, ArticleFeedResponse> for GetArticleFeedHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    fn handle(&self, query: GetArticleFeedQuery) -> hexser::HexResult<ArticleFeedResponse> {
        let repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::article_repository::ArticleFilter::FeedForUser(query.user_id.clone());

        let options = hexser::ports::repository::FindOptions {
            sort: std::option::Option::Some(vec![hexser::ports::repository::Sort {
                key: crate::ports::article_repository::ArticleSortKey::CreatedAt,
                direction: hexser::ports::repository::Direction::Desc,
            }]),
            limit: query.limit,
            offset: query.offset,
        };

        let articles = hexser::ports::repository::QueryRepository::find(&*repo, &filter, options)?;

        let feed_articles: std::vec::Vec<FeedArticle> = articles
            .into_iter()
            .map(|article| {
                let favorited = article.is_favorited_by(&query.user_id);
                FeedArticle {
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
                }
            })
            .collect();

        let count = feed_articles.len();

        std::result::Result::Ok(ArticleFeedResponse {
            articles: feed_articles,
            articles_count: count,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_article_feed_query_creation() {
        let query = super::GetArticleFeedQuery {
            user_id: std::string::String::from("user1"),
            limit: std::option::Option::Some(20),
            offset: std::option::Option::Some(0),
        };
        std::assert_eq!(query.user_id, "user1");
    }

    #[test]
    fn test_feed_article_creation() {
        let article = super::FeedArticle {
            id: std::string::String::from("1"),
            slug: std::string::String::from("test-article"),
            title: std::string::String::from("Test Article"),
            description: std::string::String::from("Description"),
            body: std::string::String::from("Body"),
            author_id: std::string::String::from("author1"),
            tags: vec![std::string::String::from("rust")],
            favorites_count: 3,
            favorited: true,
            created_at: std::string::String::from("2025-10-10T00:00:00Z"),
            updated_at: std::string::String::from("2025-10-10T00:00:00Z"),
        };
        std::assert_eq!(article.favorites_count, 3);
        std::assert!(article.favorited);
    }
}
