use axum::{
    routing::{get, post},
    http::StatusCode,
    Json,Router
};
use std::env;
use bcrypt::{DEFAULT_COST, hash_with_salt};
use uuid::Uuid;

use crate::{models::user::{self, CreateUser, ResponseUser}, pool};

// route function to nest endpoints in router
pub fn user_routes() -> Router {
    // create routes
    Router::new()
        .route("/login", post(login_user))
        .route("/create", post(create_user))
}

// handler for creating a new user
async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<ResponseUser>) {
    // insert user into table
    // if successful return a valid ResponseUser and 201 CREATED
    // if unsuccessful return an empty ResponseUser object and a 400 BAD REQUEST
    match insert_user(payload).await {
        Ok(id) => {
            // query to select user by given ID return by insert_user function
            // then return populated ResponseUser with data from table
            let results: Vec<ResponseUser> =
                sqlx::query_as::<_, ResponseUser>(
                    "SELECT * FROM \"users\" WHERE id = $1")
                .bind(id)
                .fetch_all(&pool::get_pool()).await.unwrap();
            return (StatusCode::CREATED, Json(results[0].to_owned()))
        },
        Err(e) => {
            // print error to console and send 400 BAD REQUEST with empty ResponseUser
            println!("{}", e);
            return (StatusCode::BAD_REQUEST, Json(
                ResponseUser {
                    uuid: String::new(),
                    username: String::new(),
                    email: String::new()
                }
            ))
        }
    }
}

async fn insert_user(create_user: CreateUser) -> Result<i64, sqlx::Error> {
    let id = Uuid::new_v4();
    let mut salt: [u8; 16] = [0;16];
    salt.copy_from_slice(&env::var("PASSWORD_SALT").unwrap().as_bytes()[0..16]);
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO \"users\" (uuid, username, pass, email)
        VALUES ($1, $2, $3, $4)
        RETURNING id;")
        .bind(id.to_string())
        .bind(create_user.username)
        .bind(hash_with_salt(
            create_user.password,
            DEFAULT_COST,
            salt
        ).unwrap().to_string())
        .bind(create_user.email)
        .fetch_one(&pool::get_pool()).await?;
    Ok(row.0)
}

async fn login_user(
    Json(payload): Json<user::LoginUser>,
) -> (StatusCode, Json<user::ResponseUser>) {
    let id = Uuid::new_v4();
    let user = user::ResponseUser {
        uuid: id.to_string(),
        email: "no email".to_string(),
        username: payload.username
    };
    // send 200 response with JSON response
    (StatusCode::OK, Json(user))
}