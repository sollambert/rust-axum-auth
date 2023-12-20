use axum::{
    http::StatusCode,
    routing::post,
    Json,Router
};
use bcrypt::verify;
use serde::Serialize;

use crate::{models::user::{ResponseUser, LoginUser}, strategies::{users, authentication::{AuthError, generate_new_token, AuthBody, Claims}}};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .route("/login", post(login_user))
        .route("/protected", post(protected))
}

// response struct for login route
#[derive(Serialize)]
struct LoginResponse {
    auth: AuthBody,
    user: ResponseUser
}

// example route for authentication protection, will be replaced with middleware
// right now, authentication is only required for routes that extract the Claims object from a requests decoded Bearer token
async fn protected(_claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)",
    ))
}

// route for logging in user with provided LoginUser json
async fn login_user(
    Json(payload): Json<LoginUser>,
) -> Result<(StatusCode, Json<LoginResponse>), AuthError> {
    // check if supplied credentials are not empty
    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(AuthError::MissingCredentials)
    }
    // get user by username from database
    let result = users::get_db_user_by_username(payload.username).await;
    // if can't get user by username, return 400
    if let Err(e) = result {
        println!("{:?}", e);
        // return (StatusCode::BAD_REQUEST, Json(response_user));
        return Err(AuthError::WrongCredentials)
    }
    // print user data
    println!("{:?}", result);
    // unwrap result from DB as user object
    let user = result.unwrap();
    // verify supplied password is validated
    if verify(payload.pass, &user.pass).unwrap() {
        // build response user
        let response_user = ResponseUser {
            uuid: user.uuid,
            username: user.username,
            email: user.email
        };
        // send 201 response with JWT token response
        Ok((StatusCode::CREATED, Json(LoginResponse {
            auth: generate_new_token(),
            user: response_user
        })))
    } else {
        // send 400 response with JSON response
        Err(AuthError::WrongCredentials)
    }
}