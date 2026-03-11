use axum::{
     Router, routing::get
};

mod controllers;
mod services;
use controllers::image_controller;

#[tokio::main]
async fn main() {
    let v1_static_stickers_routes = Router::new()
    .route("/static", get(image_controller::random_path))
    .route("/static/{id}", get(image_controller::input_path));

    let app = Router::new().nest("/api.naughty.animals/v1/sticker", v1_static_stickers_routes);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    clearscreen::clear().unwrap();
    println!("Server running in localhost:4000");
    axum::serve(listener, app).await.unwrap();
}