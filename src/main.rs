use axum::routing::patch;
use axum::{
    routing::{get, post},
    Router,
};
use rust_crud::db::init_db;
use rust_crud::users::{create_user, get_all_users, get_user, patch_user};

#[tokio::main]
async fn main() {
    init_db();

    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_all_users))
        .route("/users/{user_id}", get(get_user))
        .route("/users/{user_id}", patch(patch_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
