//! In-memory tag repository adapter implementation.
//!
//! Provides an in-memory implementation of the TagRepository port.
//! Extracts unique tags from articles in the system.
//!
//! Revision History
//! - 2025-10-10T10:47:00Z @AI: Add HexAdapter derive macro for automatic registration and graph introspection.
//! - 2025-10-10T09:17:00Z @AI: Add Clone derive for axum state management compatibility.
//! - 2025-10-09T23:49:00Z @AI: Initial implementation of in-memory tag adapter.

#[derive(hexser::HexAdapter, std::clone::Clone)]
pub struct InMemoryTagRepository {
    article_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>,
}

impl InMemoryTagRepository {
    pub fn new(article_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>) -> Self {
        Self { article_repo }
    }
}

impl crate::ports::tag_repository::TagRepository for InMemoryTagRepository {
    fn find_all_tags(&self) -> hexser::HexResult<std::vec::Vec<std::string::String>> {
        let repo = self.article_repo.lock().map_err(|e| {
            hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
        })?;

        let filter = crate::ports::article_repository::ArticleFilter::All;
        let articles = hexser::ports::repository::QueryRepository::find(
            &*repo,
            &filter,
            hexser::ports::repository::FindOptions::default()
        )?;

        let mut tags = std::collections::HashSet::new();
        for article in articles {
            for tag in article.tags {
                tags.insert(tag);
            }
        }

        let mut tag_list: std::vec::Vec<_> = tags.into_iter().collect();
        tag_list.sort();
        std::result::Result::Ok(tag_list)
    }
}
