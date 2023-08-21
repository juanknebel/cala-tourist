use super::attraction::{
  Attraction, AttractionRating, AttractionRatingAggregate,
};
use crate::{
  db::database::DbConnection,
  model::{
    attraction::FullAttraction, attraction_similarity::AttractionByDate,
  },
};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use sqlx::{postgres::PgRow, Row};
use std::{fmt, fmt::Formatter};

#[derive(Debug)]
pub struct EntityId {
  pub id: i32,
}

impl fmt::Display for EntityId {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.id)
  }
}

#[async_trait]
pub trait AttractionRepository {
  async fn list(&self) -> sqlx::Result<Vec<Attraction>>;
  async fn get_attraction(&self, id: i32) -> sqlx::Result<FullAttraction>;
  async fn list_ratings(&self) -> sqlx::Result<Vec<AttractionRating>>;
  async fn list_aggregates(
    &self,
  ) -> sqlx::Result<Vec<AttractionRatingAggregate>>;
  async fn all_attractions_ids(&self) -> sqlx::Result<Vec<EntityId>>;
  async fn sorted_ratings_for(
    &self,
    attraction_id: i32,
    from: NaiveDate,
  ) -> sqlx::Result<Vec<AttractionRating>>;
  async fn group_ratings_by_date(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionByDate>>;
  async fn group_aggregate_by_date(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionByDate>>;
  async fn save_attraction_rating_aggregate(
    &self,
    att_rating_aggregate: AttractionRatingAggregate,
  ) -> sqlx::Result<EntityId>;
}

#[derive(Clone)]
pub struct PgAttractionRepository {
  connection: DbConnection,
}

impl PgAttractionRepository {
  pub fn new(connection: DbConnection) -> Self {
    PgAttractionRepository {
      connection,
    }
  }
}

#[async_trait]
impl AttractionRepository for PgAttractionRepository {
  async fn list(&self) -> sqlx::Result<Vec<Attraction>> {
    let conn = self.connection.get();
    sqlx::query_as!(
      Attraction,
      r#"
      SELECT * FROM attraction
      LIMIT 20
      "#
    )
    .fetch_all(conn)
    .await
  }

  async fn get_attraction(
    &self,
    the_attraction_id: i32,
  ) -> sqlx::Result<FullAttraction> {
    let conn = self.connection.get();
    sqlx::query_as!(
      FullAttraction,
      r#"
      SELECT a.id as attraction_id, a.description, c.description as city,
      at.description as attraction_type
      FROM attraction a
      INNER JOIN attraction_type at ON a.attraction_type_id = at.id
      INNER JOIN city c ON a.city_id = c.id
      WHERE a.id = $1
      "#,
      the_attraction_id
    )
    .fetch_one(conn)
    .await
  }

  async fn list_ratings(&self) -> sqlx::Result<Vec<AttractionRating>> {
    let conn = self.connection.get();
    sqlx::query_as!(
      AttractionRating,
      r#"
      SELECT * FROM attraction_rating
      LIMIT 20
      "#
    )
    .fetch_all(conn)
    .await
  }

  async fn list_aggregates(
    &self,
  ) -> sqlx::Result<Vec<AttractionRatingAggregate>> {
    let conn = self.connection.get();
    sqlx::query_as!(
      AttractionRatingAggregate,
      r#"
      SELECT * FROM attraction_rating_aggregate
      LIMIT 20
      "#
    )
    .fetch_all(conn)
    .await
  }

  async fn all_attractions_ids(&self) -> sqlx::Result<Vec<EntityId>> {
    let conn = self.connection.get();
    sqlx::query_as!(
      EntityId,
      r#"
      SELECT id FROM attraction
      "#
    )
    .fetch_all(conn)
    .await
  }

  /// Returns the elements sorted by the rate.
  async fn sorted_ratings_for(
    &self,
    attraction_id: i32,
    from: NaiveDate,
  ) -> sqlx::Result<Vec<AttractionRating>> {
    let conn = self.connection.get();
    sqlx::query_as!(
      AttractionRating,
      r#"
      SELECT * FROM attraction_rating
      WHERE attraction_id = $1
      AND DATE_TRUNC('day', at) = DATE_TRUNC('day', $2::timestamp)
      ORDER BY rate asc
      "#,
      attraction_id,
      NaiveDateTime::new(from, NaiveTime::default())
    )
    .fetch_all(conn)
    .await
  }

  async fn group_ratings_by_date(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionByDate>> {
    let conn = self.connection.get();
    let rows = sqlx::query(
      r#"
      SELECT attraction_id, DATE_TRUNC('day', at) as at
      FROM attraction_rating
      WHERE attraction_id = $1
      GROUP BY attraction_id, DATE_TRUNC('day', at)
      "#,
    )
    .bind(attraction_id)
    .fetch_all(conn)
    .await?;
    let res = AttractionByDate::from_database(rows);
    Ok(res)
  }

  async fn group_aggregate_by_date(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionByDate>> {
    let conn = self.connection.get();
    let rows = sqlx::query(
      r#"
      SELECT attraction_id, DATE_TRUNC('day', at) as at
      FROM attraction_rating_aggregate 
      WHERE attraction_id = $1
      GROUP BY attraction_id, DATE_TRUNC('day', at)
      "#,
    )
    .bind(attraction_id)
    .fetch_all(conn)
    .await?;
    let res = AttractionByDate::from_database(rows);
    Ok(res)
  }

  async fn save_attraction_rating_aggregate(
    &self,
    att_rating_aggregate: AttractionRatingAggregate,
  ) -> sqlx::Result<EntityId> {
    let conn = self.connection.get();
    sqlx::query_as!(
      EntityId,
      r#"
      INSERT INTO attraction_rating_aggregate
      (attraction_id, at, average, ninety_five_percentile, ninety_nine_percentile)
      VALUES ($1, $2, $3, $4, $5) returning id
      "#,
      att_rating_aggregate.get_attraction_id(),
      att_rating_aggregate.get_at(),
      att_rating_aggregate.get_average(),
      att_rating_aggregate.get_95_percentile(),
      att_rating_aggregate.get_99_percentile(),
    ).fetch_one(conn).await
  }
}
impl AttractionByDate {
  fn from_database(rows: Vec<PgRow>) -> Vec<AttractionByDate> {
    let res = rows
      .iter()
      .map(|row| AttractionByDate {
        attraction_id: row.get::<i32, _>("attraction_id"),
        at: row.get::<NaiveDateTime, _>("at"),
      })
      .collect::<Vec<AttractionByDate>>();
    res
  }
}
