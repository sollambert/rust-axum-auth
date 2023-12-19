use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json,Router
};
use std::env;
use bcrypt::verify;

use crate::{models::user::{ResponseUser, LoginUser}, strategies::users};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
        .route("/login", post(login_user))
}

async fn login_user(
    Json(payload): Json<LoginUser>,
) -> (StatusCode, Json<ResponseUser>) {
    // create empty response user to be used if not verified
    let response_user = ResponseUser {
        uuid: String::new(),
        username: String::new(),
        email: String::new()
    };
    // get user by username from database
    let result = users::get_db_user_by_username(payload.username).await;
    // if can't get user by username, return 400
    if let Err(e) = result {
        println!("{:?}", e);
        return (StatusCode::BAD_REQUEST, Json(response_user));
    }
    // print user data
    println!("{:?}", result);
    // unwrap result from DB as user object
    let user = result.unwrap();
    // verify supplied password is validated
    if verify(payload.pass, &user.pass).unwrap() {
        let response_user = ResponseUser {
            uuid: user.uuid,
            username: user.username,
            email: user.email
        };
        //todo!("IMPLEMENT AUTH COOKIE");
        // send 201 response with JSON response
        (StatusCode::CREATED, Json(response_user))
    } else {
        // send 400 response with JSON response
        (StatusCode::BAD_REQUEST, Json(response_user))
    }
}