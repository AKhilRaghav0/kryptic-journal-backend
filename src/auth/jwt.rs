use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = OffsetDateTime::now_utc();
        let exp = now + Duration::hours(24); // Token expires in 24 hours

        Self {
            sub: user_id.to_string(),
            exp: exp.unix_timestamp(),
            iat: now.unix_timestamp(),
        }
    }
}

pub fn create_jwt(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(user_id);
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = EncodingKey::from_secret(secret.as_ref());
    
    encode(&Header::default(), &claims, &key)
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();
    
    decode::<Claims>(token, &key, &validation).map(|data| data.claims)
}

pub async fn auth_middleware(
    State(_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth_header.trim_start_matches("Bearer ");
    
    match verify_jwt(token) {
        Ok(claims) => {
            // Add user ID to request extensions for use in handlers
            request.extensions_mut().insert(claims.sub);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
} 