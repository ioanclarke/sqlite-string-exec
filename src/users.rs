use crate::db;
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;
use rust_patch::Patch;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub(crate) id: String,
    pub(crate) username: String,
}

impl User {
    pub(crate) fn new(username: String) -> User {
        User {
            id: String::from(Uuid::new_v4()),
            username,
        }
    }
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
    let maybe_user = db::get_user(user_id);
    get_or_404(maybe_user)
}

pub async fn patch_user(Path(user_id): Path<String>, Json(patch): Json<UserPatch>) -> Response {
    let maybe_user = db::update_user(user_id, patch);
    get_or_404(maybe_user)
}

fn get_or_404<T: Serialize>(maybe_entity: Option<T>) -> Response {
    match maybe_entity {
        Some(entty) => Json(entty).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
