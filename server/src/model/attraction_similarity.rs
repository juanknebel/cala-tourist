use crate::model::{
  attraction::AttractionRatingAggregate,
  attraction_repository::{AttractionRepository, EntityId},
  similarity_generator::Similarity,
};
use bigdecimal::{BigDecimal, Zero};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use sqlx::FromRow;
use std::{
  collections::HashSet,
  ops::{AddAssign, Div},
  sync::Arc,
};

#[derive(FromRow, Hash, Eq, PartialEq, Clone, Debug)]
pub struct AttractionByDate {
  pub attraction_id: i32,
  pub at: NaiveDateTime,
}

#[derive(FromRow, Debug)]
pub struct SimilarityBetweenAttraction {
  pub id: i32,
  pub attraction_id: i32,
  pub to_attraction_id: i32,
  pub similarity: BigDecimal,
  pub at: NaiveDateTime,
}

#[derive(Clone)]
pub struct AttractionSimilarity<AttractionRepo> {
  attraction_repo: AttractionRepo,
}

impl<AttractionRepo> AttractionSimilarity<AttractionRepo>
where
  AttractionRepo: AttractionRepository,
{
  pub fn new(attraction_repo: AttractionRepo) -> Self {
    AttractionSimilarity {
      attraction_repo,
    }
  }

  pub async fn aggregate_for(&self, attraction_id: i32) -> Result<(), String> {
    let ratings_by_date = self
      .attraction_repo
      .group_ratings_by_date(attraction_id)
      .await
      .map_err(|e| e.to_string())?;
    let aggregate_by_date = self
      .attraction_repo
      .group_aggregate_by_date(attraction_id)
      .await
      .map_err(|e| e.to_string())?;
    let aggregate_by_date: HashSet<AttractionByDate> =
      HashSet::from_iter(aggregate_by_date);

    let missing_aggregates = ratings_by_date
      .into_iter()
      .filter(|item| !aggregate_by_date.contains(item))
      .collect::<Vec<AttractionByDate>>();

    for an_attraction_date in missing_aggregates {
      match self
        .aggregate_for_at(
          an_attraction_date.attraction_id,
          an_attraction_date.at.date(),
        )
        .await
      {
        Ok(_) => {},
        Err(e) => {
          println!(
            "Error generating the aggregation for attraction: {} \n {}",
            an_attraction_date.attraction_id, e
          );
        },
      }
    }

    Ok(())
  }

  async fn aggregate_for_at(
    &self,
    attraction_id: i32,
    at: NaiveDate,
  ) -> Result<(), String> {
    let ratings = self
      .attraction_repo
      .sorted_ratings_for(attraction_id, at)
      .await
      .map_err(|e| e.to_string())?;

    let total = ratings.len() as i32;
    let index: usize = (0.95 * (total as f32)) as usize;
    let percentile_95 = ratings.get(index).unwrap();
    let percentile_95 = percentile_95.get_rate();
    let index: usize = (0.99 * (total as f32)) as usize;
    let percentile_99 = ratings.get(index).unwrap().get_rate();

    let mut average = BigDecimal::zero();

    for a_rating in ratings {
      average.add_assign(a_rating.rate);
    }
    average = average.div(BigDecimal::from(total));

    let attraction_aggregate = AttractionRatingAggregate {
      id: 0,
      attraction_id,
      at: at.and_time(NaiveTime::default()),
      average,
      ninety_five_percentile: percentile_95,
      ninety_nine_percentile: percentile_99,
    };

    self
      .attraction_repo
      .save_attraction_rating_aggregate(attraction_aggregate)
      .await
      .map_err(|e| e.to_string())?;
    Ok(())
  }

  /// Generate similarity between all the attractions.
  ///
  /// In this first approach I decided to iterate over every attraction and
  /// fetch the information in every loop, justo to keep it simple. But it is
  /// going to be optimized because it doesn't scale well if the number of
  /// attractions grows.
  /// # Return:
  /// * Nothing if everything is ok.
  /// * Err a string that represents the error.
  pub async fn generate_similarity(
    &self,
    similarity_calculator: impl Similarity,
  ) -> Result<(), String> {
    let attractions: Arc<[EntityId]> = self
      .attraction_repo
      .all_attractions_ids()
      .await
      .map_err(|e| e.to_string())?
      .into();
    for an_attraction in attractions.clone().iter() {
      let one_attraction_info = self
        .attraction_repo
        .get_info(an_attraction.id)
        .await
        .map_err(|e| e.to_string())?;
      for other_attraction in attractions.clone().iter() {
        let other_attraction_info = self
          .attraction_repo
          .get_info(other_attraction.id)
          .await
          .map_err(|e| e.to_string())?;
        let similarity = similarity_calculator
          .similarity_between(&one_attraction_info, &other_attraction_info);
        let similarity_between_attraction = SimilarityBetweenAttraction {
          id: 0,
          attraction_id: an_attraction.id,
          to_attraction_id: other_attraction.id,
          similarity: similarity.clone(),
          at: Utc::now().naive_utc(),
        };
        self
          .attraction_repo
          .save_similarity(similarity_between_attraction)
          .await
          .map_err(|e| e.to_string())?;
      }
    }
    Ok(())
  }
}
