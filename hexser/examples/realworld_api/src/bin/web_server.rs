//! Web server entry point for RealWorld API.
//!
//! Starts an HTTP server using axum that serves the complete RealWorld API
//! specification with JWT authentication and all CRUD operations.
//!
//! Run with: `cargo run --bin web_server`
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Initial implementation of web server entry point.

#[tokio::main]
async fn main() -> std::result::Result<(), std::boxed::Box<dyn std::error::Error>> {
    println!("=== RealWorld API - Web Server ===\n");

    let user_repo = std::sync::Arc::new(std::sync::Mutex::new(
        realworld_api::adapters::in_memory_db::user_adapter::InMemoryUserRepository::new()
    ));

    let article_repo = std::sync::Arc::new(std::sync::Mutex::new(
        realworld_api::adapters::in_memory_db::article_adapter::InMemoryArticleRepository::with_user_repo(
            std::sync::Arc::clone(&user_repo)
        )
    ));

    let comment_repo = std::sync::Arc::new(std::sync::Mutex::new(
        realworld_api::adapters::in_memory_db::comment_adapter::InMemoryCommentRepository::new()
    ));

    let tag_repo = realworld_api::adapters::in_memory_db::tag_adapter::InMemoryTagRepository::new(
        std::sync::Arc::clone(&article_repo)
    );

    println!("✓ Initialized in-memory repositories");

    let app = realworld_api::adapters::web::routes::app_router(
        user_repo,
        article_repo,
        comment_repo,
        tag_repo,
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("✓ Server listening on http://0.0.0.0:3000");
    println!("\nAvailable endpoints:");
    println!("  POST   /api/users               - Register user");
    println!("  POST   /api/users/login         - Login");
    println!("  GET    /api/user                - Get current user (auth)");
    println!("  PUT    /api/user                - Update user (auth)");
    println!("  GET    /api/profiles/:username  - Get profile");
    println!("  POST   /api/profiles/:username/follow   - Follow user (auth)");
    println!("  DELETE /api/profiles/:username/follow   - Unfollow user (auth)");
    println!("  GET    /api/articles            - List articles");
    println!("  POST   /api/articles            - Create article (auth)");
    println!("  GET    /api/articles/feed       - Get feed (auth)");
    println!("  GET    /api/articles/:slug      - Get article");
    println!("  PUT    /api/articles/:slug      - Update article (auth)");
    println!("  DELETE /api/articles/:slug      - Delete article (auth)");
    println!("  POST   /api/articles/:slug/favorite   - Favorite article (auth)");
    println!("  DELETE /api/articles/:slug/favorite   - Unfavorite article (auth)");
    println!("  GET    /api/articles/:slug/comments   - Get comments");
    println!("  POST   /api/articles/:slug/comments   - Add comment (auth)");
    println!("  DELETE /api/articles/:slug/comments/:id - Delete comment (auth)");
    println!("  GET    /api/tags                - Get all tags");
    println!("\n🚀 Server started successfully!\n");

    axum::serve(listener, app).await?;

    std::result::Result::Ok(())
}
