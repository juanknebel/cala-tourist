use std::sync::Arc;

use crate::{
  model::{
    attraction::{Attraction, AttractionType, City},
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
  pub city: String,
  pub attraction_type: String,
}

impl AttractionDto {
  pub fn new(
    (attr, attr_type, city): &(Attraction, AttractionType, City),
  ) -> Self {
    AttractionDto {
      id: attr.get_id(),
      description: attr.get_description(),
      city: city.get_description(),
      attraction_type: attr_type.get_description(),
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
  let attractions = attraction_controller.list().unwrap_or_default();
  let dtos = attractions
    .iter()
    .map(AttractionDto::new)
    .collect::<Vec<AttractionDto>>();
  Ok(Json(dtos))
}

async fn get_attraction(
  Path(id): Path<i32>,
  State(attraction_controller): State<Arc<dyn AttractionController>>,
) -> Result<Json<AttractionDto>> {
  match attraction_controller.get_attraction(id) {
    Some(an_attraction) => Ok(Json(AttractionDto::new(&an_attraction))),
    None => Err(Error::AttractionNotFound {
      id,
    }),
  }
}
