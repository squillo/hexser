//! Comment HTTP routes.
//!
//! Implements REST API endpoints for adding, retrieving, and deleting
//! comments on articles according to the RealWorld API specification.
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Initial implementation of comment routes.

#[derive(serde::Deserialize)]
pub struct AddCommentRequest {
    pub comment: AddCommentData,
}

#[derive(serde::Deserialize)]
pub struct AddCommentData {
    pub body: std::string::String,
}

#[derive(serde::Serialize)]
pub struct CommentResponse {
    pub comment: CommentData,
}

#[derive(serde::Serialize)]
pub struct CommentsResponse {
    pub comments: std::vec::Vec<CommentData>,
}

#[derive(serde::Serialize)]
pub struct CommentData {
    pub id: std::string::String,
    pub body: std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: std::string::String,
    #[serde(rename = "authorId")]
    pub author_id: std::string::String,
}

pub async fn add_comment(
    axum::extract::State(comment_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::comment_adapter::InMemoryCommentRepository>>>,
    axum::extract::Path(slug): axum::extract::Path<std::string::String>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
    axum::Json(payload): axum::Json<AddCommentRequest>,
) -> std::result::Result<axum::Json<CommentResponse>, axum::http::StatusCode> {
    let directive = crate::application::comment::add::AddCommentDirective {
        body: payload.comment.body,
        article_id: slug,
        author_id: claims.sub.clone(),
    };

    let handler = crate::application::comment::add::AddCommentHandler {
        repository: comment_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let repo = comment_repo.lock().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let filter = crate::ports::comment_repository::CommentFilter::ByAuthor(claims.sub);
    let comments = hexser::ports::repository::QueryRepository::find(&*repo, &filter, hexser::ports::repository::FindOptions::default())
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let comment = comments.last().ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    std::result::Result::Ok(axum::Json(CommentResponse {
        comment: CommentData {
            id: comment.id.clone(),
            body: comment.body.clone(),
            created_at: comment.created_at.clone(),
            author_id: comment.author_id.clone(),
        },
    }))
}

pub async fn get_comments(
    axum::extract::State(comment_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::comment_adapter::InMemoryCommentRepository>>>,
    axum::extract::Path(slug): axum::extract::Path<std::string::String>,
) -> std::result::Result<axum::Json<CommentsResponse>, axum::http::StatusCode> {
    let query = crate::application::comment::get::GetCommentsQuery {
        article_id: slug,
    };

    let handler = crate::application::comment::get::GetCommentsHandler {
        repository: comment_repo,
    };

    let comments = hexser::QueryHandler::handle(&handler, query)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let comment_data: std::vec::Vec<CommentData> = comments.into_iter().map(|c| CommentData {
        id: c.id,
        body: c.body,
        created_at: c.created_at,
        author_id: c.author_id,
    }).collect();

    std::result::Result::Ok(axum::Json(CommentsResponse {
        comments: comment_data,
    }))
}

pub async fn delete_comment(
    axum::extract::State(comment_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::comment_adapter::InMemoryCommentRepository>>>,
    axum::extract::Path((_slug, comment_id)): axum::extract::Path<(std::string::String, std::string::String)>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
) -> std::result::Result<axum::http::StatusCode, axum::http::StatusCode> {
    let directive = crate::application::comment::delete::DeleteCommentDirective {
        comment_id,
        author_id: claims.sub,
    };

    let handler = crate::application::comment::delete::DeleteCommentHandler {
        repository: comment_repo,
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::FORBIDDEN)?;

    std::result::Result::Ok(axum::http::StatusCode::NO_CONTENT)
}

pub fn routes(
    comment_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::comment_adapter::InMemoryCommentRepository>>,
) -> axum::Router {
    axum::Router::new()
        .route("/api/articles/:slug/comments", axum::routing::get(get_comments))
        .route(
            "/api/articles/:slug/comments",
            axum::routing::post(add_comment)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::auth_middleware)),
        )
        .route(
            "/api/articles/:slug/comments/:id",
            axum::routing::delete(delete_comment)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::auth_middleware)),
        )
        .with_state(comment_repo)
}
