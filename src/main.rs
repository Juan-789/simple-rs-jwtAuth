mod model;
mod controller;
use axum::{Router};
use axum::routing::{post, get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", post(controller::login_handler))
        .route("/info", get(controller::get_info_handler));

    let listener =
    tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("Listening");

    axum::serve(listener, app)
        .await
        .unwrap();
}
