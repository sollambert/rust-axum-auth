use axum::{
    http::StatusCode,
    routing::post,
    Json,Router
};

use crate::{models::user::{CreateUser, ResponseUser}, strategies::users};

// route function to nest endpoints in router
pub fn routes() -> Router {
    // create routes
    Router::new()
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
    match users::insert_db_user(payload).await {
        Ok(id) => {
            // query to select user by given ID return by insert_user function
            // then return populated ResponseUser with data from table
            let result = users::get_db_user_by_id(id).await;
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
