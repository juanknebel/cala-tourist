use super::attraction_repository::AttractionRepository;
use crate::model::attraction::{Attraction, FullAttraction};
use async_trait::async_trait;

#[async_trait]
pub trait AttractionController: Send + Sync + 'static {
  async fn list(&self) -> Option<Vec<Attraction>>;
  async fn get_attraction(&self, id: i32) -> Option<FullAttraction>;
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

#[async_trait]
impl<AttractionRepo> AttractionController
  for AttractionControllerImpl<AttractionRepo>
where
  AttractionRepo: AttractionRepository + Send + Sync + 'static,
{
  async fn list(&self) -> Option<Vec<Attraction>> {
    match self.attraction_repository.list().await {
      Ok(attractions) => Some(attractions),
      Err(_) => None,
    }
  }

  async fn get_attraction(&self, id: i32) -> Option<FullAttraction> {
    match self.attraction_repository.get_attraction(id).await {
      Ok(an_attraction) => Some(an_attraction),
      Err(_) => None,
    }
  }
}
