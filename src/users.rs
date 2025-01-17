use crate::db;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use rust_patch::Patch;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) username: String,
}

#[derive(Deserialize, Eq, PartialEq)]
pub struct UserCreate {
    pub(crate) username: String,
}

#[derive(Deserialize, Patch)]
#[patch = "User"]
pub struct UserPatch {
    pub(crate) username: String,
}

pub async fn create_user(Json(payload): Json<UserCreate>) -> (StatusCode, Json<User>) {
    let user = db::insert_user(payload);
    (StatusCode::CREATED, Json(user))
}

pub async fn get_all_users() -> (StatusCode, Json<Vec<User>>) {
    let users = db::get_all_users();
    (StatusCode::OK, Json(users))
}

pub async fn get_user(Path(user_id): Path<String>) -> Response {
    let user = db::get_user(user_id);

    match user {
        Some(user) => Json(user).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn patch_user(Path(user_id): Path<String>, Json(patch): Json<UserPatch>) -> Json<User> {
    let user = db::update_user(user_id, patch);
    Json(user)
}
