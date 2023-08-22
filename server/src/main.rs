#![feature(exclusive_range_pattern)]

use std::{net::SocketAddr, sync::Arc};

pub use self::errors::{Error, Result};

mod application;
mod db;
mod errors;
mod model;

use application::{attraction_api, similarity_api};
use axum::{routing::get, Router};
use dotenv::dotenv;
use model::{
  attraction_controller::AttractionControllerImpl,
  attraction_repository::PgAttractionRepository,
  similarity_controller::SimilarityControllerImpl,
};

#[tokio::main]
async fn main() {
  dotenv().ok();
  // ---- Database initialization ---- //
  let db_uri = std::env::var("DATABASE_URL");
  let db = db::database::DbConnection::new(db_uri).await;

  // ---- Repositories initialization ---- //
  let attraction_repo = PgAttractionRepository::new(db.clone());

  // ---- Controllers initialization ---- //
  let attraction_controller =
    AttractionControllerImpl::new(attraction_repo.clone());

  let similarity_controller =
    SimilarityControllerImpl::new(attraction_repo.clone());

  // ---- Routes initialization ---- //
  let attractions_api =
    attraction_api::routes(Arc::new(attraction_controller.clone()));

  let similarity_api = similarity_api::routes(Arc::new(similarity_controller));

  let app = Router::new()
    .route("/hello", get(hello))
    .merge(attractions_api)
    .merge(similarity_api);

  // ---- run it with hyper on localhost:8080 ---- //
  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("->> LISTENING on {addr}\n");
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

// basic handler that responds with a static string
async fn hello() -> &'static str {
  "Hello, World!"
}
