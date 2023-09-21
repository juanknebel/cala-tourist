use super::attraction::{Attraction, AttractionRating};
use crate::{
  db::database::DbConnection,
  model::attraction::{AttractionByDate, FullAttraction},
};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::{fmt, fmt::Formatter};

#[derive(Debug, Clone, Copy)]
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
  async fn ratings_for(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionRating>>;
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
}

#[derive(Clone, Default)]
pub struct DummyAttractionRepo;

#[async_trait]
impl AttractionRepository for DummyAttractionRepo {
  async fn list(&self) -> sqlx::Result<Vec<Attraction>> {
    todo!()
  }

  async fn get_attraction(&self, _: i32) -> sqlx::Result<FullAttraction> {
    todo!()
  }

  async fn ratings_for(&self, _: i32) -> sqlx::Result<Vec<AttractionRating>> {
    todo!()
  }

  async fn all_attractions_ids(&self) -> sqlx::Result<Vec<EntityId>> {
    todo!()
  }

  async fn sorted_ratings_for(
    &self,
    _: i32,
    _: NaiveDate,
  ) -> sqlx::Result<Vec<AttractionRating>> {
    todo!()
  }

  async fn group_ratings_by_date(
    &self,
    _: i32,
  ) -> sqlx::Result<Vec<AttractionByDate>> {
    todo!()
  }
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

  async fn ratings_for(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionRating>> {
    let conn = self.connection.get();
    sqlx::query_as!(
      AttractionRating,
      r#"
      SELECT * FROM attraction_rating
      WHERE attraction_id = $1
      ORDER BY at desc
      "#,
      attraction_id
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
      order by id asc
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
    let rows = sqlx::query_as!(
      AttractionByDate,
      r#"
      SELECT attraction_id, DATE_TRUNC('day', at) as at
      FROM attraction_rating
      WHERE attraction_id = $1
      GROUP BY attraction_id, DATE_TRUNC('day', at)
      "#,
      attraction_id
    )
    .fetch_all(conn)
    .await?;
    Ok(rows)
  }
}
