use crate::{
  db,
  model::{
    attraction_controller::{AttractionController, AttractionControllerImpl},
    attraction_repository::{DummyAttractionRepo, PgAttractionRepository},
    similarity_controller::{SimilarityController, SimilarityControllerImpl},
    similarity_repository::{DummySimilarityRepo, PgSimilarityRepository},
  },
};
use dotenv::dotenv;
use std::sync::Arc;

pub struct Application {
  pub attraction: Arc<dyn AttractionController>,
  pub similarity: Arc<dyn SimilarityController>,
}

impl Application {
  async fn new() -> Self {
    // ---- Database initialization ---- //
    let db_uri = std::env::var("DATABASE_URL");
    let db = db::database::DbConnection::new(db_uri).await;

    // ---- Repositories initialization ---- //
    let attraction_repo = PgAttractionRepository::new(db.clone());
    let similarity_repo = PgSimilarityRepository::new(db.clone());

    // ---- Controllers initialization ---- //
    let attraction_controller =
      AttractionControllerImpl::new(attraction_repo.clone());

    let similarity_controller = SimilarityControllerImpl::new(
      attraction_repo.clone(),
      similarity_repo.clone(),
    );

    Application {
      attraction: Arc::new(attraction_controller),
      similarity: Arc::new(similarity_controller),
    }
  }

  async fn new_test_app() -> Self {
    // ---- Database initialization ---- //
    let db_uri = std::env::var("DATABASE_URL");
    let _ = db::database::DbConnection::new(db_uri).await;

    // ---- Repositories initialization ---- //
    let attraction_repo = DummyAttractionRepo::default();
    let similarity_repo = DummySimilarityRepo::default();

    // ---- Controllers initialization ---- //
    let attraction_controller =
      AttractionControllerImpl::new(attraction_repo.clone());

    let similarity_controller = SimilarityControllerImpl::new(
      attraction_repo.clone(),
      similarity_repo.clone(),
    );

    Application {
      attraction: Arc::new(attraction_controller),
      similarity: Arc::new(similarity_controller),
    }
  }

  async fn new_dummy_app() -> Self {
    // ---- Repositories initialization ---- //
    let attraction_repo = DummyAttractionRepo::default();
    let similarity_repo = DummySimilarityRepo::default();

    // ---- Controllers initialization ---- //
    let attraction_controller =
      AttractionControllerImpl::new(attraction_repo.clone());

    let similarity_controller = SimilarityControllerImpl::new(
      attraction_repo.clone(),
      similarity_repo.clone(),
    );

    Application {
      attraction: Arc::new(attraction_controller),
      similarity: Arc::new(similarity_controller),
    }
  }
}

pub async fn start_application() -> Application {
  dotenv().ok();
  let scope = std::env::var("SCOPE").unwrap_or(String::from("TEST"));
  return match scope.as_str() {
    "DEV" | "PROD" => Application::new().await,
    "TEST" => Application::new_test_app().await,
    _ => {
      println!("xx->> UNKNOWN SCOPE: {scope}, FALLBACK INTO DUMMY\n");
      Application::new_dummy_app().await
    },
  };
}
