use super::attraction::{
  Attraction, AttractionRating, AttractionRatingAggregate,
};
use crate::{db::database::DbConnection, model::attraction::FullAttraction};
use async_trait::async_trait;

#[async_trait]
pub trait AttractionRepository {
  async fn list(&self) -> sqlx::Result<Vec<Attraction>>;
  async fn get_attraction(&self, id: i32) -> sqlx::Result<FullAttraction>;
  async fn list_ratings(&self) -> sqlx::Result<Vec<AttractionRating>>;
  async fn list_aggregates(
    &self,
  ) -> sqlx::Result<Vec<AttractionRatingAggregate>>;
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
}
