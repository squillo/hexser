//! Article HTTP routes.
//!
//! Implements REST API endpoints for article CRUD operations, feed,
//! and favorite/unfavorite actions according to the RealWorld API specification.
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Initial implementation of article routes.

#[derive(serde::Deserialize)]
pub struct CreateArticleRequest {
    pub article: CreateArticleData,
}

#[derive(serde::Deserialize)]
pub struct CreateArticleData {
    pub title: std::string::String,
    pub description: std::string::String,
    pub body: std::string::String,
    #[serde(default)]
    pub tag_list: std::vec::Vec<std::string::String>,
}

#[derive(serde::Deserialize)]
pub struct UpdateArticleRequest {
    pub article: UpdateArticleData,
}

#[derive(serde::Deserialize)]
pub struct UpdateArticleData {
    pub title: std::option::Option<std::string::String>,
    pub description: std::option::Option<std::string::String>,
    pub body: std::option::Option<std::string::String>,
}

#[derive(serde::Serialize)]
pub struct ArticleResponse {
    pub article: ArticleData,
}

#[derive(serde::Serialize)]
pub struct ArticleListResponse {
    pub articles: std::vec::Vec<ArticleData>,
    pub articles_count: usize,
}

#[derive(serde::Serialize)]
pub struct ArticleData {
    pub slug: std::string::String,
    pub title: std::string::String,
    pub description: std::string::String,
    pub body: std::string::String,
    #[serde(rename = "tagList")]
    pub tag_list: std::vec::Vec<std::string::String>,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: usize,
    #[serde(rename = "createdAt")]
    pub created_at: std::string::String,
    #[serde(rename = "updatedAt")]
    pub updated_at: std::string::String,
}

pub async fn create_article(
    axum::extract::State(article_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
    axum::Json(payload): axum::Json<CreateArticleRequest>,
) -> std::result::Result<axum::Json<ArticleResponse>, axum::http::StatusCode> {
    let directive = crate::application::article::create::CreateArticleDirective {
        title: payload.article.title,
        description: payload.article.description,
        body: payload.article.body,
        author_id: claims.sub.clone(),
        tags: payload.article.tag_list,
    };

    let handler = crate::application::article::create::CreateArticleHandler {
        repository: article_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let repo = article_repo.lock().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let filter = crate::ports::article_repository::ArticleFilter::ByAuthor(claims.sub.clone());
    let articles = hexser::ports::repository::QueryRepository::find(&*repo, &filter, hexser::ports::repository::FindOptions::default())
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let article = articles.last().ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    std::result::Result::Ok(axum::Json(ArticleResponse {
        article: ArticleData {
            slug: article.slug.clone(),
            title: article.title.clone(),
            description: article.description.clone(),
            body: article.body.clone(),
            tag_list: article.tags.clone(),
            favorited: article.is_favorited_by(&claims.sub),
            favorites_count: article.favorites_count(),
            created_at: article.created_at.clone(),
            updated_at: article.updated_at.clone(),
        },
    }))
}

pub async fn list_articles(
    axum::extract::State(article_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<std::string::String, std::string::String>>,
) -> std::result::Result<axum::Json<ArticleListResponse>, axum::http::StatusCode> {
    let tag = params.get("tag").cloned();
    let author = params.get("author").cloned();
    let favorited = params.get("favorited").cloned();
    let limit = params.get("limit").and_then(|s| s.parse::<u32>().ok());
    let offset = params.get("offset").and_then(|s| s.parse::<u64>().ok());

    let query = crate::application::article::list::ListArticlesQuery {
        tag,
        author,
        favorited,
        limit,
        offset,
    };

    let handler = crate::application::article::list::ListArticlesHandler {
        repository: article_repo,
    };

    let response = hexser::QueryHandler::handle(&handler, query)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let articles: std::vec::Vec<ArticleData> = response.articles.into_iter().map(|a| ArticleData {
        slug: a.slug,
        title: a.title,
        description: a.description,
        body: a.body,
        tag_list: a.tags,
        favorited: a.favorited,
        favorites_count: a.favorites_count,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }).collect();

    std::result::Result::Ok(axum::Json(ArticleListResponse {
        articles,
        articles_count: response.articles_count,
    }))
}

pub async fn get_article(
    axum::extract::State(article_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>>,
    axum::extract::Path(slug): axum::extract::Path<std::string::String>,
    claims: std::option::Option<axum::Extension<crate::adapters::web::auth::Claims>>,
) -> std::result::Result<axum::Json<ArticleResponse>, axum::http::StatusCode> {
    let requester_id = claims.map(|c| c.sub.clone());

    let query = crate::application::article::get::GetArticleQuery {
        slug,
        requester_id,
    };

    let handler = crate::application::article::get::GetArticleHandler {
        repository: article_repo,
    };

    let response = hexser::QueryHandler::handle(&handler, query)
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    std::result::Result::Ok(axum::Json(ArticleResponse {
        article: ArticleData {
            slug: response.slug,
            title: response.title,
            description: response.description,
            body: response.body,
            tag_list: response.tags,
            favorited: response.favorited,
            favorites_count: response.favorites_count,
            created_at: response.created_at,
            updated_at: response.updated_at,
        },
    }))
}

pub async fn update_article(
    axum::extract::State(article_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>>,
    axum::extract::Path(slug): axum::extract::Path<std::string::String>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
    axum::Json(payload): axum::Json<UpdateArticleRequest>,
) -> std::result::Result<axum::Json<ArticleResponse>, axum::http::StatusCode> {
    let directive = crate::application::article::update::UpdateArticleDirective {
        slug: slug.clone(),
        author_id: claims.sub.clone(),
        title: payload.article.title,
        description: payload.article.description,
        body: payload.article.body,
    };

    let handler = crate::application::article::update::UpdateArticleHandler {
        repository: article_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let repo = article_repo.lock().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let filter = crate::ports::article_repository::ArticleFilter::BySlug(slug);
    let article = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    std::result::Result::Ok(axum::Json(ArticleResponse {
        article: ArticleData {
            slug: article.slug.clone(),
            title: article.title.clone(),
            description: article.description.clone(),
            body: article.body.clone(),
            tag_list: article.tags.clone(),
            favorited: article.is_favorited_by(&claims.sub),
            favorites_count: article.favorites_count(),
            created_at: article.created_at.clone(),
            updated_at: article.updated_at.clone(),
        },
    }))
}

pub async fn delete_article(
    axum::extract::State(article_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>>,
    axum::extract::Path(slug): axum::extract::Path<std::string::String>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
) -> std::result::Result<axum::http::StatusCode, axum::http::StatusCode> {
    let directive = crate::application::article::delete::DeleteArticleDirective {
        slug,
        author_id: claims.sub,
    };

    let handler = crate::application::article::delete::DeleteArticleHandler {
        repository: article_repo,
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::FORBIDDEN)?;

    std::result::Result::Ok(axum::http::StatusCode::NO_CONTENT)
}

pub async fn get_feed(
    axum::extract::State(article_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<std::string::String, std::string::String>>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
) -> std::result::Result<axum::Json<ArticleListResponse>, axum::http::StatusCode> {
    let limit = params.get("limit").and_then(|s| s.parse::<u32>().ok());
    let offset = params.get("offset").and_then(|s| s.parse::<u64>().ok());

    let query = crate::application::article::feed::GetArticleFeedQuery {
        user_id: claims.sub.clone(),
        limit,
        offset,
    };

    let handler = crate::application::article::feed::GetArticleFeedHandler {
        repository: article_repo,
    };

    let response = hexser::QueryHandler::handle(&handler, query)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let articles: std::vec::Vec<ArticleData> = response.articles.into_iter().map(|a| ArticleData {
        slug: a.slug,
        title: a.title,
        description: a.description,
        body: a.body,
        tag_list: a.tags,
        favorited: a.favorited,
        favorites_count: a.favorites_count,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }).collect();

    std::result::Result::Ok(axum::Json(ArticleListResponse {
        articles,
        articles_count: response.articles_count,
    }))
}

pub async fn favorite_article(
    axum::extract::State(article_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>>,
    axum::extract::Path(slug): axum::extract::Path<std::string::String>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
) -> std::result::Result<axum::Json<ArticleResponse>, axum::http::StatusCode> {
    let directive = crate::application::article::favorite::FavoriteArticleDirective {
        slug: slug.clone(),
        user_id: claims.sub.clone(),
    };

    let handler = crate::application::article::favorite::FavoriteArticleHandler {
        repository: article_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let repo = article_repo.lock().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let filter = crate::ports::article_repository::ArticleFilter::BySlug(slug);
    let article = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    std::result::Result::Ok(axum::Json(ArticleResponse {
        article: ArticleData {
            slug: article.slug.clone(),
            title: article.title.clone(),
            description: article.description.clone(),
            body: article.body.clone(),
            tag_list: article.tags.clone(),
            favorited: article.is_favorited_by(&claims.sub),
            favorites_count: article.favorites_count(),
            created_at: article.created_at.clone(),
            updated_at: article.updated_at.clone(),
        },
    }))
}

pub async fn unfavorite_article(
    axum::extract::State(article_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>>,
    axum::extract::Path(slug): axum::extract::Path<std::string::String>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
) -> std::result::Result<axum::Json<ArticleResponse>, axum::http::StatusCode> {
    let directive = crate::application::article::favorite::UnfavoriteArticleDirective {
        slug: slug.clone(),
        user_id: claims.sub.clone(),
    };

    let handler = crate::application::article::favorite::UnfavoriteArticleHandler {
        repository: article_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let repo = article_repo.lock().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let filter = crate::ports::article_repository::ArticleFilter::BySlug(slug);
    let article = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    std::result::Result::Ok(axum::Json(ArticleResponse {
        article: ArticleData {
            slug: article.slug.clone(),
            title: article.title.clone(),
            description: article.description.clone(),
            body: article.body.clone(),
            tag_list: article.tags.clone(),
            favorited: article.is_favorited_by(&claims.sub),
            favorites_count: article.favorites_count(),
            created_at: article.created_at.clone(),
            updated_at: article.updated_at.clone(),
        },
    }))
}

pub fn routes(
    article_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::article_adapter::InMemoryArticleRepository>>,
) -> axum::Router {
    axum::Router::new()
        .route("/api/articles", axum::routing::get(list_articles))
        .route(
            "/api/articles",
            axum::routing::post(create_article)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::auth_middleware)),
        )
        .route(
            "/api/articles/feed",
            axum::routing::get(get_feed)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::auth_middleware)),
        )
        .route(
            "/api/articles/:slug",
            axum::routing::get(get_article)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::optional_auth_middleware)),
        )
        .route(
            "/api/articles/:slug",
            axum::routing::put(update_article)
                .delete(delete_article)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::auth_middleware)),
        )
        .route(
            "/api/articles/:slug/favorite",
            axum::routing::post(favorite_article)
                .delete(unfavorite_article)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::auth_middleware)),
        )
        .with_state(article_repo)
}
