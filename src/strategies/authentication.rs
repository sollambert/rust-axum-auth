use std::{env, time::{SystemTime, UNIX_EPOCH}};

use axum::{
    Json, RequestPartsExt, http::{
        StatusCode, request::Parts
    }, response::{
        IntoResponse, Response
    }, extract::FromRequestParts, async_trait, body::Body
};
use axum_extra::{headers::{Authorization, authorization::Bearer}, TypedHeader};
use jsonwebtoken::{EncodingKey, DecodingKey, Validation, decode, encode, Header};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::user::ResponseUser;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be configured.");
    Keys::new(secret.as_bytes())
});

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret)
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        // Check if token is expired
        if SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() > u128::from_str_radix(env::var("JWT_EXPIRE").unwrap().as_str(), 0-9).unwrap() {
            return Err(AuthError::TokenExpired)
        }
        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::TokenExpired => (StatusCode::BAD_REQUEST, "Token expired"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

pub fn generate_token_response(response_user: ResponseUser) -> AuthTokenResponse {
    let claims = Claims {
        sub: env::var("JWT_SUB").unwrap(),
        com: env::var("JWT_COMPANY").unwrap(),
        iat: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    };
    return AuthTokenResponse {
        auth_body: AuthBody::new(encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation).unwrap()),
        response_user
    }
}

#[derive(Debug, Serialize)]
pub struct AuthTokenResponse {
    auth_body: AuthBody,
    response_user: ResponseUser
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    com: String,
    iat: u128
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    TokenExpired,
    InvalidToken
}