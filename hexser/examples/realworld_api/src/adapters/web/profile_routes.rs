//! Profile HTTP routes.
//!
//! Implements REST API endpoints for viewing user profiles and
//! follow/unfollow operations according to the RealWorld API specification.
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Initial implementation of profile routes.

#[derive(serde::Serialize)]
pub struct ProfileResponse {
    pub profile: ProfileData,
}

#[derive(serde::Serialize)]
pub struct ProfileData {
    pub username: std::string::String,
    pub bio: std::option::Option<std::string::String>,
    pub image: std::option::Option<std::string::String>,
    pub following: bool,
}

pub async fn get_profile(
    axum::extract::State(user_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
    axum::extract::Path(username): axum::extract::Path<std::string::String>,
    claims: std::option::Option<axum::Extension<crate::adapters::web::auth::Claims>>,
) -> std::result::Result<axum::Json<ProfileResponse>, axum::http::StatusCode> {
    let requester_id = claims.map(|c| c.sub.clone());

    let query = crate::application::profile::get::GetProfileQuery {
        username,
        requester_id,
    };

    let handler = crate::application::profile::get::GetProfileHandler {
        repository: user_repo,
    };

    let response = hexser::QueryHandler::handle(&handler, query)
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    std::result::Result::Ok(axum::Json(ProfileResponse {
        profile: ProfileData {
            username: response.username,
            bio: response.bio,
            image: response.image,
            following: response.following,
        },
    }))
}

pub async fn follow_profile(
    axum::extract::State(user_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
    axum::extract::Path(username): axum::extract::Path<std::string::String>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
) -> std::result::Result<axum::Json<ProfileResponse>, axum::http::StatusCode> {
    let directive = crate::application::profile::follow::FollowUserDirective {
        follower_id: claims.sub.clone(),
        followee_username: username.clone(),
    };

    let handler = crate::application::profile::follow::FollowUserHandler {
        repository: user_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let query = crate::application::profile::get::GetProfileQuery {
        username,
        requester_id: std::option::Option::Some(claims.sub),
    };

    let get_handler = crate::application::profile::get::GetProfileHandler {
        repository: user_repo,
    };

    let response = hexser::QueryHandler::handle(&get_handler, query)
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    std::result::Result::Ok(axum::Json(ProfileResponse {
        profile: ProfileData {
            username: response.username,
            bio: response.bio,
            image: response.image,
            following: response.following,
        },
    }))
}

pub async fn unfollow_profile(
    axum::extract::State(user_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
    axum::extract::Path(username): axum::extract::Path<std::string::String>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
) -> std::result::Result<axum::Json<ProfileResponse>, axum::http::StatusCode> {
    let directive = crate::application::profile::unfollow::UnfollowUserDirective {
        follower_id: claims.sub.clone(),
        followee_username: username.clone(),
    };

    let handler = crate::application::profile::unfollow::UnfollowUserHandler {
        repository: user_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let query = crate::application::profile::get::GetProfileQuery {
        username,
        requester_id: std::option::Option::Some(claims.sub),
    };

    let get_handler = crate::application::profile::get::GetProfileHandler {
        repository: user_repo,
    };

    let response = hexser::QueryHandler::handle(&get_handler, query)
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    std::result::Result::Ok(axum::Json(ProfileResponse {
        profile: ProfileData {
            username: response.username,
            bio: response.bio,
            image: response.image,
            following: response.following,
        },
    }))
}

pub fn routes(
    user_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>,
) -> axum::Router {
    axum::Router::new()
        .route(
            "/api/profiles/:username",
            axum::routing::get(get_profile)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::optional_auth_middleware)),
        )
        .route(
            "/api/profiles/:username/follow",
            axum::routing::post(follow_profile)
                .delete(unfollow_profile)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::auth_middleware)),
        )
        .with_state(user_repo)
}
