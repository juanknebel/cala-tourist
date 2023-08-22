use std::sync::Arc;

use crate::{
  model::{
    attraction::{Attraction, FullAttraction},
    attraction_controller::AttractionController,
  },
  Error, Result,
};
use axum::{
  extract::{Path, State},
  routing::get,
  Json, Router,
};
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

pub fn routes(attraction_controller: Arc<dyn AttractionController>) -> Router {
  Router::new()
    .route("/attraction/all", get(list_attractions))
    .route("/attraction/:id", get(get_attraction))
    .with_state(attraction_controller)
}

async fn list_attractions(
  State(attraction_controller): State<Arc<dyn AttractionController>>,
) -> Result<Json<Vec<AttractionDto>>> {
  let attractions = attraction_controller.list().await.unwrap_or_default();
  let dtos = attractions
    .iter()
    .map(AttractionDto::from_entity)
    .collect::<Vec<AttractionDto>>();
  Ok(Json(dtos))
}

async fn get_attraction(
  Path(id): Path<i32>,
  State(attraction_controller): State<Arc<dyn AttractionController>>,
) -> Result<Json<AttractionDto>> {
  match attraction_controller.get_attraction(id).await {
    Some(an_attraction) => Ok(Json(AttractionDto::from_full(&an_attraction))),
    None => Err(Error::AttractionNotFound {
      id,
    }),
  }
}
