use axum::{
    routing::{get, post},
    http::StatusCode,
    Json,Router
};

use crate::models::user;

pub fn user_routes() -> Router {
    Router::new()
        .route("/login", post(login_user))
        .route("/create", post(create_user))
}

async fn create_user(
    Json(payload): Json<user::CreateUser>,
) -> (StatusCode, Json<user::ResponseUser>) {
    let user = user::ResponseUser {
        uuid: "1234567890".to_string(),
        email: "test@testing.com".to_string(),
        username: "Testing".to_string()
    };
    // send 201 response with JSON response
    (StatusCode::CREATED, Json(user))
}

async fn login_user(
    Json(payload): Json<user::LoginUser>,
) -> (StatusCode, Json<user::ResponseUser>) {
    let user = user::ResponseUser {
        uuid: "1234567890".to_string(),
        email: "test@testing.com".to_string(),
        username: "Testing".to_string()
    };
    // send 201 response with JSON response
    (StatusCode::OK, Json(user))
}