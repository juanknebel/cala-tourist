use crate::{
  model::{
    attraction::AttractionRatingAggregate,
    similarity_controller::SimilarityController,
  },
  Error, Result,
};
use axum::{
  extract::{Query, State},
  routing::{get, post},
  Json, Router,
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, Default)]
pub struct RatingAggregateDto {
  pub attraction_id: i32,
  pub at: NaiveDateTime,
  pub average: BigDecimal,
  pub percentile_95: BigDecimal,
  pub percentile_99: BigDecimal,
}

impl RatingAggregateDto {
  fn new(a_rating_aggregate: &AttractionRatingAggregate) -> Self {
    RatingAggregateDto {
      attraction_id: a_rating_aggregate.get_attraction_id(),
      at: a_rating_aggregate.get_at(),
      average: a_rating_aggregate.get_average(),
      percentile_95: a_rating_aggregate.get_95_percentile(),
      percentile_99: a_rating_aggregate.get_99_percentile(),
    }
  }
}

#[derive(Deserialize)]
struct AttractionParam {
  attraction_id: i32,
}

/// Defines the endpoints that handles the interaction with the similarity and
/// the attractions.
pub fn routes(similarity_controller: Arc<dyn SimilarityController>) -> Router {
  Router::new()
    .route(
      "/similarity/aggregate",
      get(list_ratings_aggregate),
    )
    .route("/similarity/calculate", post(calculate))
    .with_state(similarity_controller)
}

/// List all the aggregates ratings from all the attractions.
///
/// # Arguments:
/// * attraction_param: the attraction query param necessary to retrieve the
/// aggregates.
/// * similarity_controller: the controller responsible of the actions.
///
/// # Return:
/// * Ok with a vector of rating aggregates.
/// * Err with the error.
async fn list_ratings_aggregate(
  Query(attraction_param): Query<AttractionParam>,
  State(similarity_controller): State<Arc<dyn SimilarityController>>,
) -> Result<Json<Vec<RatingAggregateDto>>> {
  println!(
    "->> AGGREGATES for attraction: {}\n",
    attraction_param.attraction_id
  );
  let all_aggregate_ratings = similarity_controller
    .list_rating_aggregate(attraction_param.attraction_id)
    .await
    .unwrap_or_default();
  let dtos = all_aggregate_ratings
    .iter()
    .map(RatingAggregateDto::new)
    .collect::<Vec<RatingAggregateDto>>();
  Ok(Json(dtos))
}

/// Calculate the similarity between all the attractions.
///
/// # Arguments:
/// * similarity_controller: the controller responsible of the actions.
///
/// # Return:
/// * Err with 500 status code.
async fn calculate(
  State(similarity_controller): State<Arc<dyn SimilarityController>>,
) -> Result<()> {
  println!("->> CALCULATE AGGREGATE\n");
  match similarity_controller
    .calculate_similarity_between_attractions()
    .await
  {
    Ok(_) => Ok(()),
    Err(e) => {
      println!("xx->> {}", e);
      Err(Error::GenerateSimilarityFail)
    },
  }
}
