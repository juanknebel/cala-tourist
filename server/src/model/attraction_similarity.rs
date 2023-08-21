use crate::model::{
  attraction::AttractionRatingAggregate,
  attraction_repository::AttractionRepository,
};
use bigdecimal::{BigDecimal, ToPrimitive, Zero};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};
use std::{
  collections::HashSet,
  ops::{AddAssign, Div},
};

#[derive(FromRow, Hash, Eq, PartialEq, Clone, Debug)]
pub struct AttractionByDate {
  pub attraction_id: i32,
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

  pub async fn generate_similarity(&self) -> Result<(), String> {
    Ok(())
  }
}
