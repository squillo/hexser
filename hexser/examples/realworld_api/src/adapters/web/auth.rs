//! JWT authentication middleware for web adapter.
//!
//! Provides JWT token generation, validation, and authentication middleware
//! for protecting API endpoints. Uses jsonwebtoken crate for token handling.
//!
//! Revision History
//! - 2025-10-10T08:28:00Z @AI: Initial implementation of JWT authentication middleware.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

const JWT_SECRET: &[u8] = b"your-secret-key-change-in-production";

#[derive(std::clone::Clone, std::fmt::Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: std::string::String,
    pub exp: usize,
}

impl Claims {
    pub fn new(user_id: std::string::String) -> Self {
        let exp = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 86400 * 7) as usize;

        Self { sub: user_id, exp }
    }
}

pub fn generate_token(user_id: &str) -> std::result::Result<std::string::String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(std::string::String::from(user_id));
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
}

pub fn validate_token(token: &str) -> std::result::Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    std::result::Result::Ok(token_data.claims)
}

pub async fn auth_middleware(
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> std::result::Result<axum::response::Response, axum::http::StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let std::option::Option::Some(auth_value) = auth_header {
        if auth_value.starts_with("Token ") {
            let token = &auth_value[6..];
            match validate_token(token) {
                std::result::Result::Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    return std::result::Result::Ok(next.run(req).await);
                }
                std::result::Result::Err(_) => {
                    return std::result::Result::Err(axum::http::StatusCode::UNAUTHORIZED);
                }
            }
        }
    }

    std::result::Result::Err(axum::http::StatusCode::UNAUTHORIZED)
}

pub async fn optional_auth_middleware(
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let std::option::Option::Some(auth_value) = auth_header {
        if auth_value.starts_with("Token ") {
            let token = &auth_value[6..];
            if let std::result::Result::Ok(claims) = validate_token(token) {
                req.extensions_mut().insert(claims);
            }
        }
    }

    next.run(req).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_claims_creation() {
        let claims = super::Claims::new(std::string::String::from("user123"));
        std::assert_eq!(claims.sub, "user123");
        std::assert!(claims.exp > 0);
    }

    #[test]
    fn test_token_generation_and_validation() {
        let user_id = "test_user";
        let token = super::generate_token(user_id).unwrap();
        std::assert!(!token.is_empty());

        let claims = super::validate_token(&token).unwrap();
        std::assert_eq!(claims.sub, user_id);
    }

    #[test]
    fn test_invalid_token() {
        let result = super::validate_token("invalid.token.here");
        std::assert!(result.is_err());
    }
}
