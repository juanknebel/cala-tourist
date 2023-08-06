use super::{
  attraction::AttractionRatingAggregate,
  attraction_repository::AttractionRepository,
};
use crate::model::attraction::AttractionRating;
use async_trait::async_trait;

#[async_trait]
pub trait SimilarityController: Send + Sync + 'static {
  async fn list_rating_aggregate(
    &self,
  ) -> Option<Vec<AttractionRatingAggregate>>;
  async fn list_ratings(&self) -> Option<Vec<AttractionRating>>;
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

#[async_trait]
impl<AttractionRepo> SimilarityController
  for SimilarityControllerImpl<AttractionRepo>
where
  AttractionRepo: AttractionRepository + Send + Sync + 'static,
{
  async fn list_rating_aggregate(
    &self,
  ) -> Option<Vec<AttractionRatingAggregate>> {
    match self.attraction_repo.list_aggregates().await {
      Ok(aggregates) => Some(aggregates),
      Err(_) => None,
    }
  }

  async fn list_ratings(&self) -> Option<Vec<AttractionRating>> {
    match self.attraction_repo.list_ratings().await {
      Ok(ratings) => Some(ratings),
      Err(_) => None,
    }
  }
}
