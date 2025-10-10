//! In-memory article repository adapter implementation.
//!
//! Provides an in-memory implementation of the ArticleRepository port with
//! support for complex filtering including tags, authors, favorites, and feed generation.
//!
//! Revision History
//! - 2025-10-10T10:47:00Z @AI: Add HexAdapter derive macro for automatic registration and graph introspection.
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of in-memory article adapter.

#[derive(hexser::HexAdapter, std::default::Default)]
pub struct InMemoryArticleRepository {
    articles: std::vec::Vec<crate::domain::article::Article>,
    user_repo: std::option::Option<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
}

impl InMemoryArticleRepository {
    pub fn new() -> Self {
        Self {
            articles: std::vec::Vec::new(),
            user_repo: std::option::Option::None,
        }
    }

    pub fn with_user_repo(
        user_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>
    ) -> Self {
        Self {
            articles: std::vec::Vec::new(),
            user_repo: std::option::Option::Some(user_repo),
        }
    }

    fn matches_filter(
        article: &crate::domain::article::Article,
        filter: &crate::ports::article_repository::ArticleFilter,
        user_repo: &std::option::Option<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
    ) -> bool {
        match filter {
            crate::ports::article_repository::ArticleFilter::ById(id) => &article.id == id,
            crate::ports::article_repository::ArticleFilter::BySlug(slug) => &article.slug == slug,
            crate::ports::article_repository::ArticleFilter::ByTag(tag) => article.tags.contains(tag),
            crate::ports::article_repository::ArticleFilter::ByAuthor(author_id) => &article.author_id == author_id,
            crate::ports::article_repository::ArticleFilter::ByFavoritedBy(user_id) => article.favorited_by.contains(user_id),
            crate::ports::article_repository::ArticleFilter::FeedForUser(user_id) => {
                if let std::option::Option::Some(repo_arc) = user_repo {
                    if let std::result::Result::Ok(repo) = repo_arc.lock() {
                        let user_filter = crate::ports::user_repository::UserFilter::ById(user_id.clone());
                        if let std::result::Result::Ok(std::option::Option::Some(user)) = hexser::ports::repository::QueryRepository::find_one(&*repo, &user_filter) {
                            return user.followed_users.contains(&article.author_id);
                        }
                    }
                }
                false
            }
            crate::ports::article_repository::ArticleFilter::And(filters) => {
                filters.iter().all(|f| Self::matches_filter(article, f, user_repo))
            }
            crate::ports::article_repository::ArticleFilter::All => true,
        }
    }
}

impl hexser::ports::Repository<crate::domain::article::Article> for InMemoryArticleRepository {
    fn save(&mut self, article: crate::domain::article::Article) -> hexser::HexResult<()> {
        if let std::option::Option::Some(pos) = self.articles.iter().position(|a| a.id == article.id) {
            self.articles[pos] = article;
        } else {
            self.articles.push(article);
        }
        std::result::Result::Ok(())
    }
}

impl hexser::ports::repository::QueryRepository<crate::domain::article::Article> for InMemoryArticleRepository {
    type Filter = crate::ports::article_repository::ArticleFilter;
    type SortKey = crate::ports::article_repository::ArticleSortKey;

    fn find_one(&self, filter: &Self::Filter) -> hexser::HexResult<std::option::Option<crate::domain::article::Article>> {
        std::result::Result::Ok(
            self.articles
                .iter()
                .find(|a| Self::matches_filter(a, filter, &self.user_repo))
                .cloned()
        )
    }

    fn find(
        &self,
        filter: &Self::Filter,
        options: hexser::ports::repository::FindOptions<Self::SortKey>,
    ) -> hexser::HexResult<std::vec::Vec<crate::domain::article::Article>> {
        let mut results: std::vec::Vec<_> = self.articles
            .iter()
            .filter(|a| Self::matches_filter(a, filter, &self.user_repo))
            .cloned()
            .collect();

        if let std::option::Option::Some(sorts) = options.sort {
            for sort in sorts.into_iter().rev() {
                match (sort.key, sort.direction) {
                    (crate::ports::article_repository::ArticleSortKey::Id, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.id.cmp(&b.id));
                    }
                    (crate::ports::article_repository::ArticleSortKey::Id, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.id.cmp(&a.id));
                    }
                    (crate::ports::article_repository::ArticleSortKey::Title, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.title.cmp(&b.title));
                    }
                    (crate::ports::article_repository::ArticleSortKey::Title, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.title.cmp(&a.title));
                    }
                    (crate::ports::article_repository::ArticleSortKey::CreatedAt, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.created_at.cmp(&b.created_at));
                    }
                    (crate::ports::article_repository::ArticleSortKey::CreatedAt, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                    }
                    (crate::ports::article_repository::ArticleSortKey::UpdatedAt, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
                    }
                    (crate::ports::article_repository::ArticleSortKey::UpdatedAt, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
                    }
                    (crate::ports::article_repository::ArticleSortKey::FavoritesCount, hexser::ports::repository::Direction::Asc) => {
                        results.sort_by_key(|a| a.favorited_by.len());
                    }
                    (crate::ports::article_repository::ArticleSortKey::FavoritesCount, hexser::ports::repository::Direction::Desc) => {
                        results.sort_by_key(|a| std::cmp::Reverse(a.favorited_by.len()));
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
        let before = self.articles.len();
        let user_repo = &self.user_repo;
        self.articles.retain(|a| !Self::matches_filter(a, filter, user_repo));
        std::result::Result::Ok((before - self.articles.len()) as u64)
    }
}

impl crate::ports::article_repository::ArticleRepository for InMemoryArticleRepository {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_save_and_find_article() {
        let mut repo = super::InMemoryArticleRepository::new();
        let article = crate::domain::article::Article::new(
            std::string::String::from("1"),
            std::string::String::from("Test Article"),
            std::string::String::from("Description"),
            std::string::String::from("Body"),
            std::string::String::from("author1"),
            vec![std::string::String::from("rust")],
        );

        hexser::ports::Repository::save(&mut repo, article.clone()).unwrap();

        let filter = crate::ports::article_repository::ArticleFilter::ById(std::string::String::from("1"));
        let found = hexser::ports::repository::QueryRepository::find_one(&repo, &filter).unwrap();
        std::assert!(found.is_some());
        std::assert_eq!(found.unwrap().title, "Test Article");
    }

    #[test]
    fn test_find_by_tag() {
        let mut repo = super::InMemoryArticleRepository::new();
        let article = crate::domain::article::Article::new(
            std::string::String::from("1"),
            std::string::String::from("Test"),
            std::string::String::from("Desc"),
            std::string::String::from("Body"),
            std::string::String::from("author1"),
            vec![std::string::String::from("rust"), std::string::String::from("hexagonal")],
        );

        hexser::ports::Repository::save(&mut repo, article).unwrap();

        let filter = crate::ports::article_repository::ArticleFilter::ByTag(std::string::String::from("rust"));
        let found = hexser::ports::repository::QueryRepository::find_one(&repo, &filter).unwrap();
        std::assert!(found.is_some());
    }
}
