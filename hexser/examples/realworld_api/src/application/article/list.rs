//! Article listing query and handler.
//!
//! Implements the article listing use case as a Query with filtering by tag,
//! author, and favorited user. Supports pagination and sorting.
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Initial implementation of article listing query.

#[derive(hexser::HexQuery, std::clone::Clone, std::fmt::Debug)]
pub struct ListArticlesQuery {
    pub tag: std::option::Option<std::string::String>,
    pub author: std::option::Option<std::string::String>,
    pub favorited: std::option::Option<std::string::String>,
    pub limit: std::option::Option<u32>,
    pub offset: std::option::Option<u64>,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ArticleListResponse {
    pub articles: std::vec::Vec<ArticleResponse>,
    pub articles_count: usize,
}

#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ArticleResponse {
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

pub struct ListArticlesHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    pub repository: std::sync::Arc<std::sync::Mutex<R>>,
}

impl<R> hexser::QueryHandler<ListArticlesQuery, ArticleListResponse> for ListArticlesHandler<R>
where
    R: crate::ports::article_repository::ArticleRepository,
{
    fn handle(&self, query: ListArticlesQuery) -> hexser::HexResult<ArticleListResponse> {
        let repo = self.repository.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let mut filters = std::vec::Vec::new();

        if let std::option::Option::Some(tag) = query.tag {
            filters.push(crate::ports::article_repository::ArticleFilter::ByTag(tag));
        }

        if let std::option::Option::Some(author) = query.author {
            filters.push(crate::ports::article_repository::ArticleFilter::ByAuthor(author));
        }

        if let std::option::Option::Some(favorited) = query.favorited {
            filters.push(crate::ports::article_repository::ArticleFilter::ByFavoritedBy(favorited));
        }

        let filter = if filters.is_empty() {
            crate::ports::article_repository::ArticleFilter::All
        } else if filters.len() == 1 {
            filters.into_iter().next().unwrap()
        } else {
            crate::ports::article_repository::ArticleFilter::And(filters)
        };

        let options = hexser::ports::repository::FindOptions {
            sort: std::option::Option::Some(vec![hexser::ports::repository::Sort {
                key: crate::ports::article_repository::ArticleSortKey::CreatedAt,
                direction: hexser::ports::repository::Direction::Desc,
            }]),
            limit: query.limit,
            offset: query.offset,
        };

        let articles = hexser::ports::repository::QueryRepository::find(&*repo, &filter, options)?;

        let article_responses: std::vec::Vec<ArticleResponse> = articles
            .into_iter()
            .map(|article| ArticleResponse {
                id: article.id,
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                author_id: article.author_id,
                tags: article.tags,
                favorites_count: article.favorited_by.len(),
                favorited: false,
                created_at: article.created_at,
                updated_at: article.updated_at,
            })
            .collect();

        let count = article_responses.len();

        std::result::Result::Ok(ArticleListResponse {
            articles: article_responses,
            articles_count: count,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_list_articles_query_creation() {
        let query = super::ListArticlesQuery {
            tag: std::option::Option::Some(std::string::String::from("rust")),
            author: std::option::Option::None,
            favorited: std::option::Option::None,
            limit: std::option::Option::Some(10),
            offset: std::option::Option::Some(0),
        };
        std::assert!(query.tag.is_some());
    }

    #[test]
    fn test_article_response_creation() {
        let response = super::ArticleResponse {
            id: std::string::String::from("1"),
            slug: std::string::String::from("test-article"),
            title: std::string::String::from("Test Article"),
            description: std::string::String::from("Test description"),
            body: std::string::String::from("Test body"),
            author_id: std::string::String::from("author1"),
            tags: vec![std::string::String::from("rust")],
            favorites_count: 0,
            favorited: false,
            created_at: std::string::String::from("2025-10-10T00:00:00Z"),
            updated_at: std::string::String::from("2025-10-10T00:00:00Z"),
        };
        std::assert_eq!(response.title, "Test Article");
    }
}
