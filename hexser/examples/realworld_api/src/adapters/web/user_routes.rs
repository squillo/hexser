//! User and authentication HTTP routes.
//!
//! Implements REST API endpoints for user registration, authentication,
//! and profile management according to the RealWorld API specification.
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Initial implementation of user routes.

#[derive(serde::Deserialize)]
pub struct RegisterRequest {
    pub user: RegisterUserData,
}

#[derive(serde::Deserialize)]
pub struct RegisterUserData {
    pub email: std::string::String,
    pub username: std::string::String,
    pub password: std::string::String,
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub user: LoginUserData,
}

#[derive(serde::Deserialize)]
pub struct LoginUserData {
    pub email: std::string::String,
    pub password: std::string::String,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserRequest {
    pub user: UpdateUserData,
}

#[derive(serde::Deserialize)]
pub struct UpdateUserData {
    pub email: std::option::Option<std::string::String>,
    pub username: std::option::Option<std::string::String>,
    pub password: std::option::Option<std::string::String>,
    pub bio: std::option::Option<std::string::String>,
    pub image: std::option::Option<std::string::String>,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub user: UserData,
}

#[derive(serde::Serialize)]
pub struct UserData {
    pub email: std::string::String,
    pub token: std::string::String,
    pub username: std::string::String,
    pub bio: std::option::Option<std::string::String>,
    pub image: std::option::Option<std::string::String>,
}

pub async fn register(
    axum::extract::State(user_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
    axum::Json(payload): axum::Json<RegisterRequest>,
) -> std::result::Result<axum::Json<UserResponse>, axum::http::StatusCode> {
    let email = payload.user.email.clone();

    let directive = crate::application::user::register::RegisterUserDirective {
        email: email.clone(),
        username: payload.user.username,
        password: payload.user.password,
    };

    let handler = crate::application::user::register::RegisterUserHandler {
        repository: user_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let repo = user_repo.lock().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let filter = crate::ports::user_repository::UserFilter::ByEmail(email);
    let user = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = crate::adapters::web::auth::generate_token(&user.id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    std::result::Result::Ok(axum::Json(UserResponse {
        user: UserData {
            email: user.email,
            token,
            username: user.username,
            bio: user.bio,
            image: user.image,
        },
    }))
}

pub async fn login(
    axum::extract::State(user_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
    axum::Json(payload): axum::Json<LoginRequest>,
) -> std::result::Result<axum::Json<UserResponse>, axum::http::StatusCode> {
    let query = crate::application::user::login::LoginUserQuery {
        email: payload.user.email,
        password: payload.user.password,
    };

    let handler = crate::application::user::login::LoginUserHandler {
        repository: user_repo,
    };

    let response = hexser::QueryHandler::handle(&handler, query)
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

    let token = crate::adapters::web::auth::generate_token(&response.user_id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    std::result::Result::Ok(axum::Json(UserResponse {
        user: UserData {
            email: response.email,
            token,
            username: response.username,
            bio: response.bio,
            image: response.image,
        },
    }))
}

pub async fn get_current_user(
    axum::extract::State(user_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
) -> std::result::Result<axum::Json<UserResponse>, axum::http::StatusCode> {
    let query = crate::application::user::get_current::GetCurrentUserQuery {
        user_id: claims.sub,
    };

    let handler = crate::application::user::get_current::GetCurrentUserHandler {
        repository: user_repo,
    };

    let response = hexser::QueryHandler::handle(&handler, query)
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    std::result::Result::Ok(axum::Json(UserResponse {
        user: UserData {
            email: response.email,
            token: response.token,
            username: response.username,
            bio: response.bio,
            image: response.image,
        },
    }))
}

pub async fn update_user(
    axum::extract::State(user_repo): axum::extract::State<std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>>,
    axum::Extension(claims): axum::Extension<crate::adapters::web::auth::Claims>,
    axum::Json(payload): axum::Json<UpdateUserRequest>,
) -> std::result::Result<axum::Json<UserResponse>, axum::http::StatusCode> {
    let directive = crate::application::user::update::UpdateUserDirective {
        user_id: claims.sub.clone(),
        email: payload.user.email,
        username: payload.user.username,
        password: payload.user.password,
        bio: payload.user.bio,
        image: payload.user.image,
    };

    let handler = crate::application::user::update::UpdateUserHandler {
        repository: user_repo.clone(),
    };

    hexser::DirectiveHandler::handle(&handler, directive)
        .map_err(|_| axum::http::StatusCode::UNPROCESSABLE_ENTITY)?;

    let repo = user_repo.lock().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    let filter = crate::ports::user_repository::UserFilter::ById(claims.sub.clone());
    let user = hexser::ports::repository::QueryRepository::find_one(&*repo, &filter)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let token = crate::adapters::web::auth::generate_token(&user.id)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    std::result::Result::Ok(axum::Json(UserResponse {
        user: UserData {
            email: user.email,
            token,
            username: user.username,
            bio: user.bio,
            image: user.image,
        },
    }))
}

pub fn routes(
    user_repo: std::sync::Arc<std::sync::Mutex<crate::adapters::in_memory_db::user_adapter::InMemoryUserRepository>>,
) -> axum::Router {
    axum::Router::new()
        .route("/api/users", axum::routing::post(register))
        .route("/api/users/login", axum::routing::post(login))
        .route(
            "/api/user",
            axum::routing::get(get_current_user)
                .put(update_user)
                .route_layer(axum::middleware::from_fn(crate::adapters::web::auth::auth_middleware)),
        )
        .with_state(user_repo)
}
