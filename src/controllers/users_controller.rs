use axum::{
    routing::{get, post},
    http::StatusCode,
    Json,Router
};
use std::env;
use bcrypt::{DEFAULT_COST, hash_with_salt};
use uuid::Uuid;

use crate::{models::user::{self, CreateUser, ResponseUser, User}, pool};

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
    // empty ResponseUser object to send if errors encountered
    let response_user = ResponseUser {
        uuid: String::new(),
        username: String::new(),
        email: String::new()
    };
    // insert user into table
    // if successful return a valid ResponseUser and 201 CREATED
    // if unsuccessful return an empty ResponseUser object and a 400 BAD REQUEST
    match insert_user(payload).await {
        Ok(id) => {
            // query to select user by given ID return by insert_user function
            // then return populated ResponseUser with data from table
            let result = get_db_user_by_id(id).await;
            // check result for error and return error code if necessary
            if let Err(e) = result {
                println!("{:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response_user));
            }
            println!("{:?}", result);
            let user = result.unwrap();
            // re-create response_user with populated fields
            let response_user = ResponseUser {
                uuid: user.uuid,
                email: user.email,
                username: user.username
            };
            return (StatusCode::CREATED, Json(response_user))
        },
        Err(e) => {
            // print error to console and send 400 BAD REQUEST with empty ResponseUser
            println!("{:?}", e);
            return (StatusCode::BAD_REQUEST, Json(response_user))
        }
    }
}

async fn get_db_user_by_id(id: i64) -> Result<User, sqlx::Error> {
    // query for getting all data from users table where user row matches given user ID
    let rows = sqlx::query_as::<_, User>(
        "SELECT * FROM \"users\" WHERE id = $1")
    .bind(id)
    .fetch_all(&pool::get_pool()).await.unwrap();
    // if row[0] exists return the User otherwise return RowNotFound error
    if rows.len() > 0 {
        Ok(rows[0].to_owned())
    } else {
        Err(sqlx::Error::RowNotFound)
    }
}

async fn insert_user(create_user: CreateUser) -> Result<i64, sqlx::Error> {
    // generate new user id
    let id = Uuid::new_v4();
    // initialize salt str slice
    let mut salt: [u8; 16] = [0;16];
    // load 16 bytes from PASSWORD_SALT env variable to salt str slice
    salt.copy_from_slice(&env::var("PASSWORD_SALT").unwrap().as_bytes()[0..16]);
    // perform query to insert new user with hashed password and bind all payload object fields
    let row: (i64,) = sqlx::query_as(
        "INSERT INTO \"users\" (uuid, username, pass, email)
        VALUES ($1, $2, $3, $4)
        RETURNING id;")
        .bind(id.to_string())
        .bind(create_user.username)
        // hash password with salt
        .bind(hash_with_salt(
            create_user.pass,
            DEFAULT_COST,
            salt
        ).unwrap().to_string())
        .bind(create_user.email)
        .fetch_one(&pool::get_pool()).await?;
    // return id that was returned by sql query
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