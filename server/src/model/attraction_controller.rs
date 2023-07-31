use super::{
  attraction::Attraction, attraction_repository::AttractionRepository,
};
use crate::model::attraction::{AttractionType, City};

pub trait AttractionController: Send + Sync + 'static {
  fn list(&self) -> Option<Vec<(Attraction, AttractionType, City)>>;
  fn get_attraction(
    &self,
    id: i32,
  ) -> Option<(Attraction, AttractionType, City)>;
}

#[derive(Clone)]
pub struct AttractionControllerImpl<AttractionRepo> {
  attraction_repository: AttractionRepo,
}

impl<AttractionRepo> AttractionControllerImpl<AttractionRepo>
where
  AttractionRepo: AttractionRepository,
{
  pub fn new(attraction_repository: AttractionRepo) -> Self {
    AttractionControllerImpl {
      attraction_repository,
    }
  }
}

impl<AttractionRepo> AttractionController
  for AttractionControllerImpl<AttractionRepo>
where
  AttractionRepo: AttractionRepository + Send + Sync + 'static,
{
  fn list(&self) -> Option<Vec<(Attraction, AttractionType, City)>> {
    match self.attraction_repository.list() {
      Ok(attractions) => Some(attractions),
      Err(_) => None,
    }
  }

  fn get_attraction(
    &self,
    id: i32,
  ) -> Option<(Attraction, AttractionType, City)> {
    match self.attraction_repository.get_attraction(id) {
      Ok(an_attraction) => Some(an_attraction),
      Err(_) => None,
    }
  }
}
