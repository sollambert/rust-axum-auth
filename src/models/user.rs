use serde::{Deserialize, Serialize};
pub struct User {
    pub id: i32,
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String
}

#[derive(Clone, Serialize, sqlx::FromRow)]
pub struct ResponseUser {
    pub uuid: String,
    pub username: String,
    pub email: String
}