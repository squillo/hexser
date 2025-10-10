//! Main route configuration for the web adapter.
//!
//! Combines all route modules (users, articles, comments, profiles, tags)
//! into a single application router with CORS and logging middleware.
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Initial implementation of main route configuration.

pub fn app_router(
    user_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>,
    article_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>,
    comment_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::comment_adapter::InMemoryCommentRepository>>,
    tag_repo: crate::adapters::in_memory_db::tag_adapter::InMemoryTagRepository,
) -> axum::Router {
    let user_routes = crate::adapters::web::user_routes::routes(user_repo.clone());
    let article_routes = crate::adapters::web::article_routes::routes(article_repo);
    let comment_routes = crate::adapters::web::comment_routes::routes(comment_repo);
    let profile_routes = crate::adapters::web::profile_routes::routes(user_repo);
    let tag_routes = crate::adapters::web::tag_routes::routes(tag_repo);

    axum::Router::new()
        .merge(user_routes)
        .merge(article_routes)
        .merge(comment_routes)
        .merge(profile_routes)
        .merge(tag_routes)
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                ])
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ])
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
