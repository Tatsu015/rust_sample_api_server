use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(get_user));

    let addr = SocketAddr::from((([127, 0, 0, 1]), 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    return "hello";
}

async fn get_user() -> (StatusCode, Json<User>) {
    let user = User {
        id: 1234,
        name: String::from("TestUser"),
    };

    (StatusCode::ACCEPTED, Json(user))
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}
