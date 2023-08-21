use crate::{
  model::{
    attraction::{AttractionRating, AttractionRatingAggregate},
    similarity_controller::SimilarityController,
  },
  Error, Result,
};
use axum::{
  extract::State,
  routing::{get, post},
  Json, Router,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Default)]
pub struct RatingAggregateDto {
  pub attraction_id: i32,
}

impl RatingAggregateDto {
  fn new(a_rating_aggregate: &AttractionRatingAggregate) -> Self {
    RatingAggregateDto {
      attraction_id: a_rating_aggregate.get_attraction_id(),
    }
  }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct RatingDto {
  pub id: i32,
  pub attraction_id: i32,
  pub at: NaiveDateTime,
  pub rate: bigdecimal::BigDecimal,
}

impl RatingDto {
  fn from_entity(a_rating: &AttractionRating) -> Self {
    RatingDto {
      id: a_rating.get_id(),
      attraction_id: a_rating.get_attraction_id(),
      at: a_rating.get_at(),
      rate: a_rating.get_rate(),
    }
  }
}

pub fn routes(similarity_controller: Arc<dyn SimilarityController>) -> Router {
  Router::new()
    .route(
      "/similarity/aggregate",
      get(list_ratings_aggregate),
    )
    .route("/similarity/rating", get(list_rating))
    .route("/similarity/calculate", post(calculate))
    .with_state(similarity_controller)
}

async fn list_rating(
  State(similarity_controller): State<Arc<dyn SimilarityController>>,
) -> Result<Json<Vec<RatingDto>>> {
  println!("->> RATINGS\n");
  let all_ratings = similarity_controller
    .list_ratings()
    .await
    .unwrap_or_default();
  let dtos = all_ratings
    .iter()
    .map(RatingDto::from_entity)
    .collect::<Vec<RatingDto>>();
  Ok(Json(dtos))
}

async fn list_ratings_aggregate(
  State(similarity_controller): State<Arc<dyn SimilarityController>>,
) -> Result<Json<Vec<RatingAggregateDto>>> {
  println!("->> AGGREGATES\n");
  let all_aggregate_ratings = similarity_controller
    .list_rating_aggregate()
    .await
    .unwrap_or_default();
  let dtos = all_aggregate_ratings
    .iter()
    .map(RatingAggregateDto::new)
    .collect::<Vec<RatingAggregateDto>>();
  Ok(Json(dtos))
}

async fn calculate(
  State(similarity_controller): State<Arc<dyn SimilarityController>>,
) -> Result<()> {
  println!("->> CALCULATE AGGREGATE\n");
  match similarity_controller
    .calculate_similarity_between_attractions()
    .await
  {
    Ok(_) => Ok(()),
    Err(e) => Err(Error::GenerateSimilarityFail),
  }
}
