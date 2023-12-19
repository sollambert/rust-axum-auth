use serde::{Deserialize, Serialize};

use crate::models::user::ResponseUser;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    created_at: usize
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    user: ResponseUser,
    client_secret: String
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken
}