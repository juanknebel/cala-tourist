use std::sync::Arc;

use crate::{
  model::{
    attraction::{Attraction, AttractionRating, FullAttraction},
    attraction_controller::AttractionController,
  },
  Error, Result,
};
use axum::{
  extract::{Path, State},
  routing::get,
  Json, Router,
};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Default)]
pub struct AttractionDto {
  pub id: i32,
  pub description: String,
  pub city: Option<String>,
  pub attraction_type: Option<String>,
  pub city_id: Option<i32>,
  pub attraction_type_id: Option<i32>,
  pub latitude: Option<String>,
  pub longitude: Option<String>,
}

impl AttractionDto {
  pub fn from_full(full_attraction: &FullAttraction) -> Self {
    AttractionDto {
      id: full_attraction.get_attraction_id(),
      description: full_attraction.get_description(),
      city: Some(full_attraction.get_city()),
      attraction_type: Some(full_attraction.get_attraction_type()),
      city_id: None,
      attraction_type_id: None,
      latitude: None,
      longitude: None,
    }
  }

  pub fn from_entity(an_attraction: &Attraction) -> Self {
    AttractionDto {
      id: an_attraction.get_id(),
      description: an_attraction.get_description(),
      city: None,
      attraction_type: None,
      city_id: Some(an_attraction.get_city_id()),
      attraction_type_id: Some(an_attraction.get_attraction_type_id()),
      latitude: an_attraction.get_latitude(),
      longitude: an_attraction.get_longitude(),
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

/// Defines the endpoints that handles the interaction with the attractions.
pub fn routes(attraction_controller: Arc<dyn AttractionController>) -> Router {
  Router::new()
    .route("/attraction/all", get(list))
    .route("/attraction/:id", get(get_attraction))
    .route("/attraction/:id/rating", get(rating))
    .with_state(attraction_controller)
}

/// List all the registered attractions.
///
/// # Arguments:
/// * attraction_controller: the controller responsible of the actions.
///
/// # Return:
/// * Ok with a vector of attractions.
/// * Err with the error.
async fn list(
  State(attraction_controller): State<Arc<dyn AttractionController>>,
) -> Result<Json<Vec<AttractionDto>>> {
  println!("->> ATTRACTIONS\n");
  let attractions = attraction_controller.list().await.unwrap_or_default();
  let dtos = attractions
    .iter()
    .map(AttractionDto::from_entity)
    .collect::<Vec<AttractionDto>>();
  Ok(Json(dtos))
}

/// Retrieve a specific attraction.
///
/// # Arguments:
/// * id: the id of the attraction to be retrieved.
/// * attraction_controller: the controller responsible of the actions.
///
/// # Return:
/// * Ok with the attraction that matches the id.
/// * Err with 404 status code.
async fn get_attraction(
  Path(id): Path<i32>,
  State(attraction_controller): State<Arc<dyn AttractionController>>,
) -> Result<Json<AttractionDto>> {
  println!("->> ATTRACTION\n");
  match attraction_controller.get_attraction(id).await {
    Some(an_attraction) => Ok(Json(AttractionDto::from_full(&an_attraction))),
    None => Err(Error::AttractionNotFound {
      id,
    }),
  }
}

/// List the ratings from particular attraction.
///
/// # Arguments:
/// * id: the id of the attraction looking for the ratings.
/// * attraction_controller: the controller responsible of the actions.
///
/// # Return:
/// * Ok with a vector of ratings from the attraction.
/// * Err with the error.
async fn rating(
  Path(id): Path<i32>,
  State(attraction_controller): State<Arc<dyn AttractionController>>,
) -> Result<Json<Vec<RatingDto>>> {
  println!("->> RATINGS\n");
  let Some(ratings_for) = attraction_controller.ratings_for(id).await else {
    return Err(Error::AttractionNotFound {
      id,
    });
  };
  let dtos = ratings_for
    .iter()
    .map(RatingDto::from_entity)
    .collect::<Vec<RatingDto>>();
  Ok(Json(dtos))
}
