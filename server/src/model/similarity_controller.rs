use super::{
  attraction::AttractionRatingAggregate,
  attraction_repository::AttractionRepository,
};
use crate::model::{
  attraction::AttractionRating, attraction_similarity::AttractionSimilarity,
};
use async_trait::async_trait;
use axum::body::HttpBody;

#[async_trait]
pub trait SimilarityController: Send + Sync + 'static {
  async fn list_rating_aggregate(
    &self,
  ) -> Option<Vec<AttractionRatingAggregate>>;
  async fn list_ratings(&self) -> Option<Vec<AttractionRating>>;
  async fn calculate_similarity_between_attractions(
    &self,
  ) -> Result<(), String>;
}

#[derive(Clone)]
pub struct SimilarityControllerImpl<AttractionRepo> {
  attraction_repo: AttractionRepo,
  attraction_similarity: AttractionSimilarity<AttractionRepo>,
}

impl<AttractionRepo> SimilarityControllerImpl<AttractionRepo>
where
  AttractionRepo: AttractionRepository + Clone,
{
  pub fn new(attraction_repo: AttractionRepo) -> Self {
    let repo_clone = attraction_repo.clone();
    SimilarityControllerImpl {
      attraction_repo: repo_clone,
      attraction_similarity: AttractionSimilarity::new(attraction_repo),
    }
  }

  async fn aggregate(&self) -> Result<(), String> {
    let attraction_ids = self
      .attraction_repo
      .all_attractions_ids()
      .await
      .map_err(|e| e.to_string())?;

    for id in attraction_ids {
      match self.attraction_similarity.aggregate_for(id.id).await {
        Ok(_) => {},
        Err(e) => {
          println!("Cannot aggregate for attraction {}", id)
        },
      }
    }
    Ok(())
  }

  async fn similarity(&self) -> Result<(), String> {
    self.attraction_similarity.generate_similarity().await
  }
}

#[async_trait]
impl<AttractionRepo> SimilarityController
  for SimilarityControllerImpl<AttractionRepo>
where
  AttractionRepo: AttractionRepository + Send + Sync + 'static + Clone,
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

  async fn calculate_similarity_between_attractions(
    &self,
  ) -> Result<(), String> {
    self.aggregate().await?;
    self.similarity().await?;
    Ok(())
  }
}
