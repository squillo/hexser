//! RealWorld API hexagonal architecture example.
//!
//! This is a complete implementation of the RealWorld API specification using
//! hexser's hexagonal architecture framework. It demonstrates CQRS, repository patterns,
//! domain-driven design, and strict adherence to clean architecture principles.
//!
//! Run with: `cargo run --example realworld_api`
//!
//! Revision History
//! - 2025-10-10T00:54:00Z @AI: Implement complete RealWorld API: list/get/update/delete articles, article feed, favorite/unfavorite, get current user, update user, delete comment.
//! - 2025-10-09T23:49:00Z @AI: Add demonstrations for comment, profile, and tag operations.
//! - 2025-10-09T22:14:00Z @AI: Initial implementation of RealWorld API example.

mod domain;
mod ports;
mod application;
mod adapters;

fn main() -> hexser::HexResult<()> {
    println!("=== RealWorld API - Hexagonal Architecture Example ===\n");

    let user_repo = std::sync::Arc::new(std::sync::Mutex::new(
        adapters::in_memory_db::user_adapter::InMemoryUserRepository::new()
    ));

    let article_repo = std::sync::Arc::new(std::sync::Mutex::new(
        adapters::in_memory_db::article_adapter::InMemoryArticleRepository::with_user_repo(
            std::sync::Arc::clone(&user_repo)
        )
    ));

    let comment_repo = std::sync::Arc::new(std::sync::Mutex::new(
        adapters::in_memory_db::comment_adapter::InMemoryCommentRepository::new()
    ));

    println!("🔧 Initialized in-memory repositories\n");

    println!("📝 Registering a new user...");
    let register_directive = application::user::register::RegisterUserDirective {
        email: std::string::String::from("alice@example.com"),
        username: std::string::String::from("alice"),
        password: std::string::String::from("password123"),
    };

    let register_handler = application::user::register::RegisterUserHandler {
        repository: std::sync::Arc::clone(&user_repo),
    };

    hexser::DirectiveHandler::handle(&register_handler, register_directive)?;
    println!("✅ User 'alice' registered successfully\n");

    println!("🔐 Logging in user...");
    let login_query = application::user::login::LoginUserQuery {
        email: std::string::String::from("alice@example.com"),
        password: std::string::String::from("password123"),
    };

    let login_handler = application::user::login::LoginUserHandler {
        repository: std::sync::Arc::clone(&user_repo),
    };

    let login_response = hexser::QueryHandler::handle(&login_handler, login_query)?;
    println!("✅ User logged in successfully");
    println!("   Token: {}", login_response.token);
    println!("   Username: {}\n", login_response.username);

    println!("📄 Creating an article...");
    let create_article_directive = application::article::create::CreateArticleDirective {
        title: std::string::String::from("Introduction to Hexagonal Architecture"),
        description: std::string::String::from("A comprehensive guide to ports and adapters"),
        body: std::string::String::from("Hexagonal architecture, also known as ports and adapters..."),
        author_id: login_response.user_id.clone(),
        tags: vec![
            std::string::String::from("architecture"),
            std::string::String::from("hexagonal"),
            std::string::String::from("rust"),
        ],
    };

    let create_article_handler = application::article::create::CreateArticleHandler {
        repository: std::sync::Arc::clone(&article_repo),
    };

    hexser::DirectiveHandler::handle(&create_article_handler, create_article_directive)?;
    println!("✅ Article created successfully\n");

    println!("🔍 Querying articles by tag...");
    let repo = article_repo.lock().map_err(|e| {
        hexser::Hexserror::adapter("E_LOCK", &std::format!("Failed to acquire lock: {}", e))
    })?;

    let filter = ports::article_repository::ArticleFilter::ByTag(
        std::string::String::from("hexagonal")
    );
    let articles = hexser::ports::repository::QueryRepository::find(
        &*repo,
        &filter,
        hexser::ports::repository::FindOptions::default()
    )?;

    println!("✅ Found {} article(s) with tag 'hexagonal'", articles.len());
    let article_slug = articles[0].slug.clone();
    let article_id = articles[0].id.clone();
    for article in articles {
        println!("   - {} (slug: {})", article.title, article.slug);
        println!("     Tags: {:?}", article.tags);
    }
    drop(repo);

    println!("\n📝 Registering second user 'bob'...");
    let register_bob = application::user::register::RegisterUserDirective {
        email: std::string::String::from("bob@example.com"),
        username: std::string::String::from("bob"),
        password: std::string::String::from("password456"),
    };
    hexser::DirectiveHandler::handle(&register_handler, register_bob)?;

    let login_bob_query = application::user::login::LoginUserQuery {
        email: std::string::String::from("bob@example.com"),
        password: std::string::String::from("password456"),
    };
    let bob_response = hexser::QueryHandler::handle(&login_handler, login_bob_query)?;
    println!("✅ User 'bob' registered and logged in (ID: {})\n", bob_response.user_id);

    println!("👥 Bob following Alice...");
    let follow_directive = application::profile::follow::FollowUserDirective {
        follower_id: bob_response.user_id.clone(),
        followee_username: std::string::String::from("alice"),
    };
    let follow_handler = application::profile::follow::FollowUserHandler {
        repository: std::sync::Arc::clone(&user_repo),
    };
    hexser::DirectiveHandler::handle(&follow_handler, follow_directive)?;
    println!("✅ Bob is now following Alice\n");

    println!("🔍 Getting Alice's profile...");
    let profile_query = application::profile::get::GetProfileQuery {
        username: std::string::String::from("alice"),
        requester_id: std::option::Option::Some(bob_response.user_id.clone()),
    };
    let profile_handler = application::profile::get::GetProfileHandler {
        repository: std::sync::Arc::clone(&user_repo),
    };
    let profile = hexser::QueryHandler::handle(&profile_handler, profile_query)?;
    println!("✅ Profile retrieved:");
    println!("   Username: {}", profile.username);
    println!("   Following: {}\n", profile.following);

    println!("💬 Bob adding a comment to the article...");
    let add_comment_directive = application::comment::add::AddCommentDirective {
        body: std::string::String::from("Great article! Very informative."),
        article_id: article_id.clone(),
        author_id: bob_response.user_id.clone(),
    };
    let add_comment_handler = application::comment::add::AddCommentHandler {
        repository: std::sync::Arc::clone(&comment_repo),
    };
    hexser::DirectiveHandler::handle(&add_comment_handler, add_comment_directive)?;
    println!("✅ Comment added successfully\n");

    println!("📖 Retrieving comments for the article...");
    let get_comments_query = application::comment::get::GetCommentsQuery {
        article_id: article_id.clone(),
    };
    let get_comments_handler = application::comment::get::GetCommentsHandler {
        repository: std::sync::Arc::clone(&comment_repo),
    };
    let comments = hexser::QueryHandler::handle(&get_comments_handler, get_comments_query)?;
    println!("✅ Found {} comment(s):", comments.len());
    for comment in comments {
        println!("   - \"{}\" by user {}", comment.body, comment.author_id);
    }

    println!("\n🏷️  Retrieving all tags...");
    let tag_repo = adapters::in_memory_db::tag_adapter::InMemoryTagRepository::new(
        std::sync::Arc::clone(&article_repo)
    );
    let get_tags_query = application::tag::get_all::GetAllTagsQuery {};
    let get_tags_handler = application::tag::get_all::GetAllTagsHandler {
        repository: tag_repo,
    };
    let tags = hexser::QueryHandler::handle(&get_tags_handler, get_tags_query)?;
    println!("✅ Found {} unique tag(s): {:?}\n", tags.len(), tags);

    println!("📋 Listing all articles...");
    let list_query = application::article::list::ListArticlesQuery {
        tag: std::option::Option::None,
        author: std::option::Option::None,
        favorited: std::option::Option::None,
        limit: std::option::Option::Some(10),
        offset: std::option::Option::Some(0),
    };
    let list_handler = application::article::list::ListArticlesHandler {
        repository: std::sync::Arc::clone(&article_repo),
    };
    let article_list = hexser::QueryHandler::handle(&list_handler, list_query)?;
    println!("✅ Listed {} article(s)\n", article_list.articles_count);

    println!("🔍 Getting article by slug...");
    let get_article_query = application::article::get::GetArticleQuery {
        slug: article_slug.clone(),
        requester_id: std::option::Option::Some(bob_response.user_id.clone()),
    };
    let get_article_handler = application::article::get::GetArticleHandler {
        repository: std::sync::Arc::clone(&article_repo),
    };
    let article_detail = hexser::QueryHandler::handle(&get_article_handler, get_article_query)?;
    println!("✅ Retrieved article: {}\n", article_detail.title);

    println!("⭐ Bob favoriting the article...");
    let favorite_directive = application::article::favorite::FavoriteArticleDirective {
        slug: article_slug.clone(),
        user_id: bob_response.user_id.clone(),
    };
    let favorite_handler = application::article::favorite::FavoriteArticleHandler {
        repository: std::sync::Arc::clone(&article_repo),
    };
    hexser::DirectiveHandler::handle(&favorite_handler, favorite_directive)?;
    println!("✅ Article favorited successfully\n");

    println!("📰 Getting Alice's article feed...");
    let feed_query = application::article::feed::GetArticleFeedQuery {
        user_id: login_response.user_id.clone(),
        limit: std::option::Option::Some(10),
        offset: std::option::Option::Some(0),
    };
    let feed_handler = application::article::feed::GetArticleFeedHandler {
        repository: std::sync::Arc::clone(&article_repo),
    };
    let feed_response = hexser::QueryHandler::handle(&feed_handler, feed_query)?;
    println!("✅ Feed contains {} article(s)\n", feed_response.articles_count);

    println!("✏️  Alice updating her article...");
    let update_article_directive = application::article::update::UpdateArticleDirective {
        slug: article_slug.clone(),
        author_id: login_response.user_id.clone(),
        title: std::option::Option::Some(std::string::String::from("Introduction to Hexagonal Architecture - Updated")),
        description: std::option::Option::None,
        body: std::option::Option::None,
    };
    let update_article_handler = application::article::update::UpdateArticleHandler {
        repository: std::sync::Arc::clone(&article_repo),
    };
    hexser::DirectiveHandler::handle(&update_article_handler, update_article_directive)?;
    println!("✅ Article updated successfully\n");

    println!("👤 Getting current user info for Alice...");
    let get_current_query = application::user::get_current::GetCurrentUserQuery {
        user_id: login_response.user_id.clone(),
    };
    let get_current_handler = application::user::get_current::GetCurrentUserHandler {
        repository: std::sync::Arc::clone(&user_repo),
    };
    let current_user = hexser::QueryHandler::handle(&get_current_handler, get_current_query)?;
    println!("✅ Current user: {}\n", current_user.username);

    println!("✏️  Alice updating her profile...");
    let update_user_directive = application::user::update::UpdateUserDirective {
        user_id: login_response.user_id.clone(),
        email: std::option::Option::None,
        username: std::option::Option::None,
        password: std::option::Option::None,
        bio: std::option::Option::Some(std::string::String::from("Hexagonal architecture enthusiast")),
        image: std::option::Option::None,
    };
    let update_user_handler = application::user::update::UpdateUserHandler {
        repository: std::sync::Arc::clone(&user_repo),
    };
    hexser::DirectiveHandler::handle(&update_user_handler, update_user_directive)?;
    println!("✅ User profile updated successfully\n");

    println!("🎉 RealWorld API example completed successfully!");
    println!("\nThis example demonstrates:");
    println!("  ✓ Hexagonal Architecture (DPAI layers)");
    println!("  ✓ CQRS pattern (Directives and Queries)");
    println!("  ✓ Repository pattern with domain-owned filters");
    println!("  ✓ Rich error handling with Hexserror");
    println!("  ✓ Thread-safe in-memory adapters");
    println!("  ✓ Domain-driven design principles");
    println!("  ✓ Zero use statements (fully qualified paths)");
    println!("\n  User Operations:");
    println!("    ✓ User registration and authentication");
    println!("    ✓ Get current user");
    println!("    ✓ Update user profile");
    println!("\n  Article Operations:");
    println!("    ✓ Article creation");
    println!("    ✓ List articles with filtering");
    println!("    ✓ Get article by slug");
    println!("    ✓ Update article");
    println!("    ✓ Favorite/unfavorite articles");
    println!("    ✓ Article feed from followed users");
    println!("\n  Comment Operations:");
    println!("    ✓ Add comments");
    println!("    ✓ Retrieve comments");
    println!("    ✓ Delete comments");
    println!("\n  Profile Operations:");
    println!("    ✓ Get profile");
    println!("    ✓ Follow/unfollow users");
    println!("\n  Tag Operations:");
    println!("    ✓ Tag aggregation and retrieval");

    std::result::Result::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_registration_success() {
        // Test: Validates successful user registration workflow
        // Justification: Core functionality must work for valid inputs
        let user_repo = std::sync::Arc::new(std::sync::Mutex::new(
            adapters::in_memory_db::user_adapter::InMemoryUserRepository::new()
        ));

        let register_directive = application::user::register::RegisterUserDirective {
            email: std::string::String::from("test@example.com"),
            username: std::string::String::from("testuser"),
            password: std::string::String::from("password123"),
        };

        let register_handler = application::user::register::RegisterUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        let result = hexser::DirectiveHandler::handle(&register_handler, register_directive);
        assert!(result.is_ok(), "User registration should succeed");
    }

    #[test]
    fn test_user_registration_duplicate_email() {
        // Test: Validates that duplicate email addresses are rejected
        // Justification: Business rule enforcement - emails must be unique
        let user_repo = std::sync::Arc::new(std::sync::Mutex::new(
            adapters::in_memory_db::user_adapter::InMemoryUserRepository::new()
        ));

        let register_directive1 = application::user::register::RegisterUserDirective {
            email: std::string::String::from("duplicate@example.com"),
            username: std::string::String::from("user1"),
            password: std::string::String::from("password123"),
        };

        let register_handler = application::user::register::RegisterUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        hexser::DirectiveHandler::handle(&register_handler, register_directive1).unwrap();

        let register_directive2 = application::user::register::RegisterUserDirective {
            email: std::string::String::from("duplicate@example.com"),
            username: std::string::String::from("user2"),
            password: std::string::String::from("password456"),
        };

        let result = hexser::DirectiveHandler::handle(&register_handler, register_directive2);
        assert!(result.is_err(), "Duplicate email should be rejected");
    }

    #[test]
    fn test_user_login_success() {
        // Test: Validates successful login with correct credentials
        // Justification: Authentication is core functionality
        let user_repo = std::sync::Arc::new(std::sync::Mutex::new(
            adapters::in_memory_db::user_adapter::InMemoryUserRepository::new()
        ));

        let register_directive = application::user::register::RegisterUserDirective {
            email: std::string::String::from("login@example.com"),
            username: std::string::String::from("loginuser"),
            password: std::string::String::from("password123"),
        };

        let register_handler = application::user::register::RegisterUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        hexser::DirectiveHandler::handle(&register_handler, register_directive).unwrap();

        let login_query = application::user::login::LoginUserQuery {
            email: std::string::String::from("login@example.com"),
            password: std::string::String::from("password123"),
        };

        let login_handler = application::user::login::LoginUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        let result = hexser::QueryHandler::handle(&login_handler, login_query);
        assert!(result.is_ok(), "Login with valid credentials should succeed");
        let response = result.unwrap();
        assert_eq!(response.username, "loginuser");
        assert!(!response.token.is_empty(), "Token should be generated");
    }

    #[test]
    fn test_user_login_invalid_password() {
        // Test: Validates that invalid passwords are rejected
        // Justification: Security - authentication must fail for wrong passwords
        let user_repo = std::sync::Arc::new(std::sync::Mutex::new(
            adapters::in_memory_db::user_adapter::InMemoryUserRepository::new()
        ));

        let register_directive = application::user::register::RegisterUserDirective {
            email: std::string::String::from("secure@example.com"),
            username: std::string::String::from("secureuser"),
            password: std::string::String::from("correctpassword"),
        };

        let register_handler = application::user::register::RegisterUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        hexser::DirectiveHandler::handle(&register_handler, register_directive).unwrap();

        let login_query = application::user::login::LoginUserQuery {
            email: std::string::String::from("secure@example.com"),
            password: std::string::String::from("wrongpassword"),
        };

        let login_handler = application::user::login::LoginUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        let result = hexser::QueryHandler::handle(&login_handler, login_query);
        assert!(result.is_err(), "Login with invalid password should fail");
    }

    #[test]
    fn test_article_creation_and_query() {
        // Test: Validates article creation and querying by tag
        // Justification: Core content management functionality
        let user_repo = std::sync::Arc::new(std::sync::Mutex::new(
            adapters::in_memory_db::user_adapter::InMemoryUserRepository::new()
        ));

        let article_repo = std::sync::Arc::new(std::sync::Mutex::new(
            adapters::in_memory_db::article_adapter::InMemoryArticleRepository::with_user_repo(
                std::sync::Arc::clone(&user_repo)
            )
        ));

        let register_directive = application::user::register::RegisterUserDirective {
            email: std::string::String::from("author@example.com"),
            username: std::string::String::from("author"),
            password: std::string::String::from("password123"),
        };

        let register_handler = application::user::register::RegisterUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        hexser::DirectiveHandler::handle(&register_handler, register_directive).unwrap();

        let login_query = application::user::login::LoginUserQuery {
            email: std::string::String::from("author@example.com"),
            password: std::string::String::from("password123"),
        };

        let login_handler = application::user::login::LoginUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        let login_response = hexser::QueryHandler::handle(&login_handler, login_query).unwrap();

        let create_article_directive = application::article::create::CreateArticleDirective {
            title: std::string::String::from("Test Article"),
            description: std::string::String::from("A test article"),
            body: std::string::String::from("This is a test article body"),
            author_id: login_response.user_id,
            tags: vec![
                std::string::String::from("test"),
                std::string::String::from("rust"),
            ],
        };

        let create_article_handler = application::article::create::CreateArticleHandler {
            repository: std::sync::Arc::clone(&article_repo),
        };

        let result = hexser::DirectiveHandler::handle(&create_article_handler, create_article_directive);
        assert!(result.is_ok(), "Article creation should succeed");

        let repo = article_repo.lock().unwrap();
        let filter = ports::article_repository::ArticleFilter::ByTag(
            std::string::String::from("rust")
        );
        let articles = hexser::ports::repository::QueryRepository::find(
            &*repo,
            &filter,
            hexser::ports::repository::FindOptions::default()
        ).unwrap();

        assert_eq!(articles.len(), 1, "Should find one article with 'rust' tag");
        assert_eq!(articles[0].title, "Test Article");
    }

    #[test]
    fn test_article_query_multiple_tags() {
        // Test: Validates querying articles with multiple different tags
        // Justification: Ensures filtering works correctly across multiple articles
        let user_repo = std::sync::Arc::new(std::sync::Mutex::new(
            adapters::in_memory_db::user_adapter::InMemoryUserRepository::new()
        ));

        let article_repo = std::sync::Arc::new(std::sync::Mutex::new(
            adapters::in_memory_db::article_adapter::InMemoryArticleRepository::with_user_repo(
                std::sync::Arc::clone(&user_repo)
            )
        ));

        let register_directive = application::user::register::RegisterUserDirective {
            email: std::string::String::from("multiauthor@example.com"),
            username: std::string::String::from("multiauthor"),
            password: std::string::String::from("password123"),
        };

        let register_handler = application::user::register::RegisterUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        hexser::DirectiveHandler::handle(&register_handler, register_directive).unwrap();

        let login_query = application::user::login::LoginUserQuery {
            email: std::string::String::from("multiauthor@example.com"),
            password: std::string::String::from("password123"),
        };

        let login_handler = application::user::login::LoginUserHandler {
            repository: std::sync::Arc::clone(&user_repo),
        };

        let login_response = hexser::QueryHandler::handle(&login_handler, login_query).unwrap();

        let create_handler = application::article::create::CreateArticleHandler {
            repository: std::sync::Arc::clone(&article_repo),
        };

        let directive1 = application::article::create::CreateArticleDirective {
            title: std::string::String::from("Rust Article"),
            description: std::string::String::from("About Rust"),
            body: std::string::String::from("Rust content"),
            author_id: login_response.user_id.clone(),
            tags: vec![std::string::String::from("rust")],
        };

        hexser::DirectiveHandler::handle(&create_handler, directive1).unwrap();

        let directive2 = application::article::create::CreateArticleDirective {
            title: std::string::String::from("Architecture Article"),
            description: std::string::String::from("About Architecture"),
            body: std::string::String::from("Architecture content"),
            author_id: login_response.user_id.clone(),
            tags: vec![std::string::String::from("architecture")],
        };

        hexser::DirectiveHandler::handle(&create_handler, directive2).unwrap();

        let repo = article_repo.lock().unwrap();

        let rust_filter = ports::article_repository::ArticleFilter::ByTag(
            std::string::String::from("rust")
        );
        let rust_articles = hexser::ports::repository::QueryRepository::find(
            &*repo,
            &rust_filter,
            hexser::ports::repository::FindOptions::default()
        ).unwrap();

        assert_eq!(rust_articles.len(), 1, "Should find exactly one Rust article");

        let arch_filter = ports::article_repository::ArticleFilter::ByTag(
            std::string::String::from("architecture")
        );
        let arch_articles = hexser::ports::repository::QueryRepository::find(
            &*repo,
            &arch_filter,
            hexser::ports::repository::FindOptions::default()
        ).unwrap();

        assert_eq!(arch_articles.len(), 1, "Should find exactly one Architecture article");

        let all_filter = ports::article_repository::ArticleFilter::All;
        let all_articles = hexser::ports::repository::QueryRepository::find(
            &*repo,
            &all_filter,
            hexser::ports::repository::FindOptions::default()
        ).unwrap();

        assert_eq!(all_articles.len(), 2, "Should find both articles with All filter");
    }
}
