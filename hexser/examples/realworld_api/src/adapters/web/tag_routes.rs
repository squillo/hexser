//! Tag HTTP routes.
//!
//! Implements REST API endpoints for retrieving all tags used in articles
//! according to the RealWorld API specification.
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Initial implementation of tag routes.

#[derive(serde::Serialize)]
pub struct TagsResponse {
    pub tags: std::vec::Vec<std::string::String>,
}

pub async fn get_tags(
    axum::extract::State(tag_repo): axum::extract::State<crate::adapters::in_memory_db::tag_adapter::InMemoryTagRepository>,
) -> std::result::Result<axum::Json<TagsResponse>, axum::http::StatusCode> {
    let query = crate::application::tag::get_all::GetAllTagsQuery {};

    let handler = crate::application::tag::get_all::GetAllTagsHandler {
        repository: tag_repo,
    };

    let tags = hexser::QueryHandler::handle(&handler, query)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    std::result::Result::Ok(axum::Json(TagsResponse { tags }))
}

pub fn routes(
    tag_repo: crate::adapters::in_memory_db::tag_adapter::InMemoryTagRepository,
) -> axum::Router {
    axum::Router::new()
        .route("/api/tags", axum::routing::get(get_tags))
        .with_state(tag_repo)
}
