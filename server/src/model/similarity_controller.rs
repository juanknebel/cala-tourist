use super::{
  attraction::AttractionRatingAggregate,
  attraction_repository::AttractionRepository,
};
use crate::model::attraction::{Attraction, AttractionRating};

pub trait SimilarityController: Send + Sync + 'static {
  fn list_rating_aggregate(
    &self,
  ) -> Option<Vec<(AttractionRatingAggregate, Attraction)>>;
  fn list_ratings(&self) -> Option<Vec<(AttractionRating, Attraction)>>;
}

#[derive(Clone)]
pub struct SimilarityControllerImpl<AttractionRepo> {
  attraction_repo: AttractionRepo,
}

impl<AttractionRepo> SimilarityControllerImpl<AttractionRepo>
where
  AttractionRepo: AttractionRepository,
{
  pub fn new(attraction_repo: AttractionRepo) -> Self {
    SimilarityControllerImpl {
      attraction_repo,
    }
  }
}

impl<AttractionRepo> SimilarityController
  for SimilarityControllerImpl<AttractionRepo>
where
  AttractionRepo: AttractionRepository + Send + Sync + 'static,
{
  fn list_rating_aggregate(
    &self,
  ) -> Option<Vec<(AttractionRatingAggregate, Attraction)>> {
    match self.attraction_repo.list_aggregates() {
      Ok(aggregates) => Some(aggregates),
      Err(_) => None,
    }
  }

  fn list_ratings(&self) -> Option<Vec<(AttractionRating, Attraction)>> {
    match self.attraction_repo.list_ratings() {
      Ok(ratings) => Some(ratings),
      Err(_) => None,
    }
  }
}
