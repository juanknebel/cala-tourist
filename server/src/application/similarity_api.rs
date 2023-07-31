use crate::{
  model::{
    attraction::{Attraction, AttractionRating, AttractionRatingAggregate},
    similarity_controller::SimilarityController,
  },
  Result,
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
  fn new(
    (a_rating_aggregate, _an_attraction): &(
      AttractionRatingAggregate,
      Attraction,
    ),
  ) -> Self {
    RatingAggregateDto {
      attraction_id: a_rating_aggregate.get_attraction_id(),
    }
  }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct RatingDto {
  pub id: i32,
  pub attraction: String,
  pub at: NaiveDateTime,
  pub rate: bigdecimal::BigDecimal,
}

impl RatingDto {
  fn new((a_rating, an_attraction): &(AttractionRating, Attraction)) -> Self {
    RatingDto {
      id: a_rating.get_id(),
      attraction: an_attraction.get_description(),
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
  let all_ratings = similarity_controller.list_ratings().unwrap_or_default();
  let dtos = all_ratings
    .iter()
    .map(RatingDto::new)
    .collect::<Vec<RatingDto>>();
  Ok(Json(dtos))
}

async fn list_ratings_aggregate(
  State(similarity_controller): State<Arc<dyn SimilarityController>>,
) -> Result<Json<Vec<RatingAggregateDto>>> {
  let all_aggregate_ratings = similarity_controller
    .list_rating_aggregate()
    .unwrap_or_default();
  let dtos = all_aggregate_ratings
    .iter()
    .map(RatingAggregateDto::new)
    .collect::<Vec<RatingAggregateDto>>();
  Ok(Json(dtos))
}

async fn calculate(
  State(similarity_controller): State<Arc<dyn SimilarityController>>,
) {
}
