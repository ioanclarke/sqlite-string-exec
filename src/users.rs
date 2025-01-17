use std::fs;
use crate::db;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) username: String,
}

#[derive(Deserialize, Eq, PartialEq)]
pub struct CreateUser {
    pub(crate) username: String,
}

pub async fn get_user(Path(user_id): Path<String>) -> Response {
    let user = db::get_user(user_id);

    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    
    match user {
        Some(user) => Json(user).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn get_all_users() -> (StatusCode, Json<Vec<User>>) {
    let users = db::get_all_users();
    (StatusCode::OK, Json(users))
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = db::insert_user(payload);
    (StatusCode::CREATED, Json(user))
}
