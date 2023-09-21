use std::net::SocketAddr;

pub use self::errors::{Error, Result};

mod application;
mod db;
mod errors;
mod model;

use crate::application::app::start_application;
use application::{attraction_api, similarity_api};
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
  let application = start_application().await;

  // ---- Routes initialization ---- //
  let attractions_api = attraction_api::routes(application.attraction.clone());

  let similarity_api = similarity_api::routes(application.similarity.clone());

  let router = Router::new()
    .route("/hello", get(hello))
    .merge(attractions_api)
    .merge(similarity_api);

  // ---- run it with hyper on localhost:8080 ---- //
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("->> LISTENING on {addr}\n");
  axum::Server::bind(&addr)
    .serve(router.into_make_service())
    .await
    .unwrap();
}

// basic handler that responds with a static string
async fn hello() -> &'static str {
  "Hello, World!"
}
