//! Defines the core Article entity for the domain layer.
//!
//! The Article entity represents a blog post or article in the system, containing
//! the content, metadata, author information, tags, and favorite tracking.
//! Articles are identified by a unique slug derived from the title.
//!
//! Revision History
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of Article entity.

#[derive(hexser::HexDomain, hexser::HexEntity, std::clone::Clone, std::fmt::Debug)]
pub struct Article {
    pub id: std::string::String,
    pub slug: std::string::String,
    pub title: std::string::String,
    pub description: std::string::String,
    pub body: std::string::String,
    pub author_id: std::string::String,
    pub tags: std::vec::Vec<std::string::String>,
    pub favorited_by: std::vec::Vec<std::string::String>,
    pub created_at: std::string::String,
    pub updated_at: std::string::String,
}

impl Article {
    pub fn new(
        id: std::string::String,
        title: std::string::String,
        description: std::string::String,
        body: std::string::String,
        author_id: std::string::String,
        tags: std::vec::Vec<std::string::String>,
    ) -> Self {
        let slug = Self::generate_slug(&title);
        let now = Self::current_timestamp();
        Self {
            id,
            slug,
            title,
            description,
            body,
            author_id,
            tags,
            favorited_by: std::vec::Vec::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn update(
        &mut self,
        title: std::option::Option<std::string::String>,
        description: std::option::Option<std::string::String>,
        body: std::option::Option<std::string::String>,
    ) {
        if let std::option::Option::Some(t) = title {
            self.title = t.clone();
            self.slug = Self::generate_slug(&t);
        }
        if let std::option::Option::Some(d) = description {
            self.description = d;
        }
        if let std::option::Option::Some(b) = body {
            self.body = b;
        }
        self.updated_at = Self::current_timestamp();
    }

    pub fn favorite(&mut self, user_id: std::string::String) {
        if !self.favorited_by.contains(&user_id) {
            self.favorited_by.push(user_id);
        }
    }

    pub fn unfavorite(&mut self, user_id: &str) {
        self.favorited_by.retain(|id| id != user_id);
    }

    pub fn is_favorited_by(&self, user_id: &str) -> bool {
        self.favorited_by.contains(&std::string::String::from(user_id))
    }

    pub fn favorites_count(&self) -> usize {
        self.favorited_by.len()
    }

    fn generate_slug(title: &str) -> std::string::String {
        title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
            .collect::<std::string::String>()
            .split_whitespace()
            .collect::<std::vec::Vec<&str>>()
            .join("-")
    }

    fn current_timestamp() -> std::string::String {
        std::string::String::from("2025-10-09T22:14:00Z")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_article_creation() {
        let article = super::Article::new(
            std::string::String::from("1"),
            std::string::String::from("Test Article"),
            std::string::String::from("A test description"),
            std::string::String::from("Article body content"),
            std::string::String::from("author1"),
            vec![std::string::String::from("rust"), std::string::String::from("hexagonal")],
        );
        std::assert_eq!(article.id, "1");
        std::assert_eq!(article.slug, "test-article");
        std::assert_eq!(article.title, "Test Article");
    }

    #[test]
    fn test_article_slug_generation() {
        let article = super::Article::new(
            std::string::String::from("1"),
            std::string::String::from("How to Build! Great APIs?"),
            std::string::String::from("desc"),
            std::string::String::from("body"),
            std::string::String::from("author1"),
            vec![],
        );
        std::assert_eq!(article.slug, "how-to-build-great-apis");
    }

    #[test]
    fn test_article_favorite() {
        let mut article = super::Article::new(
            std::string::String::from("1"),
            std::string::String::from("Test"),
            std::string::String::from("desc"),
            std::string::String::from("body"),
            std::string::String::from("author1"),
            vec![],
        );
        article.favorite(std::string::String::from("user1"));
        std::assert!(article.is_favorited_by("user1"));
        std::assert_eq!(article.favorites_count(), 1);
    }

    #[test]
    fn test_article_unfavorite() {
        let mut article = super::Article::new(
            std::string::String::from("1"),
            std::string::String::from("Test"),
            std::string::String::from("desc"),
            std::string::String::from("body"),
            std::string::String::from("author1"),
            vec![],
        );
        article.favorite(std::string::String::from("user1"));
        std::assert!(article.is_favorited_by("user1"));
        article.unfavorite("user1");
        std::assert!(!article.is_favorited_by("user1"));
        std::assert_eq!(article.favorites_count(), 0);
    }

    #[test]
    fn test_article_update() {
        let mut article = super::Article::new(
            std::string::String::from("1"),
            std::string::String::from("Original Title"),
            std::string::String::from("desc"),
            std::string::String::from("body"),
            std::string::String::from("author1"),
            vec![],
        );
        let original_slug = article.slug.clone();
        article.update(
            std::option::Option::Some(std::string::String::from("Updated Title")),
            std::option::Option::None,
            std::option::Option::None,
        );
        std::assert_eq!(article.title, "Updated Title");
        std::assert_ne!(article.slug, original_slug);
        std::assert_eq!(article.slug, "updated-title");
    }
}
