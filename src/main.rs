use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use std::collections::HashMap;
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

// async fn create_user(Json(user): Json<CreateUser>) -> (StatusCode, Json<User>) {
//     let id = 1;
//     let name = user.name;

//     println!("{:?}", id);
//     println!("{:?}", name);
// }

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

struct CreateUser {
    name: String,
}
