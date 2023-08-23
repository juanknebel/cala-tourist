use super::{
  attraction::AttractionRatingAggregate,
  attraction_repository::AttractionRepository,
};
use crate::model::{
  attraction_similarity::AttractionSimilarity,
  similarity_generator::SimilarityCalculator,
  similarity_repository::SimilarityRepository,
};
use async_trait::async_trait;

#[async_trait]
pub trait SimilarityController: Send + Sync + 'static {
  async fn list_rating_aggregate(
    &self,
  ) -> Option<Vec<AttractionRatingAggregate>>;
  async fn calculate_similarity_between_attractions(
    &self,
  ) -> Result<(), String>;
}

#[derive(Clone)]
pub struct SimilarityControllerImpl<AttractionRepo, SimilarityRepo> {
  attraction_repo: AttractionRepo,
  similarity_repo: SimilarityRepo,
  attraction_similarity: AttractionSimilarity<AttractionRepo, SimilarityRepo>,
}

impl<AttractionRepo, SimilarityRepo>
  SimilarityControllerImpl<AttractionRepo, SimilarityRepo>
where
  AttractionRepo: AttractionRepository + Clone,
  SimilarityRepo: SimilarityRepository + Clone,
{
  pub fn new(
    attraction_repo: AttractionRepo,
    similarity_repo: SimilarityRepo,
  ) -> Self {
    let att_repo_clone = attraction_repo.clone();
    let sim_repo_clone = similarity_repo.clone();
    SimilarityControllerImpl {
      attraction_repo: att_repo_clone,
      similarity_repo: sim_repo_clone,
      attraction_similarity: AttractionSimilarity::new(
        attraction_repo,
        similarity_repo,
      ),
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
          println!("Cannot aggregate for attraction {}\n{}", id, e)
        },
      }
    }
    Ok(())
  }
}

#[async_trait]
impl<AttractionRepo, SimilarityRepo> SimilarityController
  for SimilarityControllerImpl<AttractionRepo, SimilarityRepo>
where
  AttractionRepo: AttractionRepository + Send + Sync + 'static + Clone,
  SimilarityRepo: SimilarityRepository + Send + Sync + 'static + Clone,
{
  async fn list_rating_aggregate(
    &self,
  ) -> Option<Vec<AttractionRatingAggregate>> {
    match self.similarity_repo.list_aggregates().await {
      Ok(aggregates) => Some(aggregates),
      Err(_) => None,
    }
  }

  async fn calculate_similarity_between_attractions(
    &self,
  ) -> Result<(), String> {
    self.aggregate().await?;
    let similarity_calculator = SimilarityCalculator::default();
    self
      .attraction_similarity
      .generate_similarity(similarity_calculator)
      .await?;
    Ok(())
  }
}
