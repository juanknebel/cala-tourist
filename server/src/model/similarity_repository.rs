use crate::{
  db::database::DbConnection,
  model::{
    attraction::{AttractionByDate, AttractionRatingAggregate},
    attraction_repository::EntityId,
    attraction_similarity::SimilarityBetweenAttraction,
    similarity_generator::AttractionInfo,
  },
};
use async_trait::async_trait;

#[async_trait]
pub trait SimilarityRepository {
  async fn list_aggregates(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionRatingAggregate>>;
  async fn group_aggregate_by_date(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionByDate>>;
  async fn save_attraction_rating_aggregate(
    &self,
    att_rating_aggregate: AttractionRatingAggregate,
  ) -> sqlx::Result<EntityId>;
  async fn get_info(&self, attraction_id: i32) -> sqlx::Result<AttractionInfo>;
  async fn save_similarity(
    &self,
    similarity: SimilarityBetweenAttraction,
  ) -> sqlx::Result<EntityId>;
}

#[derive(Clone, Default)]
pub struct DummySimilarityRepo;

#[async_trait]
impl SimilarityRepository for DummySimilarityRepo {
  async fn list_aggregates(
    &self,
    _: i32,
  ) -> sqlx::Result<Vec<AttractionRatingAggregate>> {
    todo!()
  }

  async fn group_aggregate_by_date(
    &self,
    _: i32,
  ) -> sqlx::Result<Vec<AttractionByDate>> {
    todo!()
  }

  async fn save_attraction_rating_aggregate(
    &self,
    _: AttractionRatingAggregate,
  ) -> sqlx::Result<EntityId> {
    todo!()
  }

  async fn get_info(&self, _: i32) -> sqlx::Result<AttractionInfo> {
    todo!()
  }

  async fn save_similarity(
    &self,
    _: SimilarityBetweenAttraction,
  ) -> sqlx::Result<EntityId> {
    todo!()
  }
}

#[derive(Clone)]
pub struct PgSimilarityRepository {
  connection: DbConnection,
}

impl PgSimilarityRepository {
  pub fn new(connection: DbConnection) -> Self {
    PgSimilarityRepository {
      connection,
    }
  }
}

#[async_trait]
impl SimilarityRepository for PgSimilarityRepository {
  async fn list_aggregates(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionRatingAggregate>> {
    let conn = self.connection.get();
    sqlx::query_as!(
      AttractionRatingAggregate,
      r#"
      SELECT * FROM attraction_rating_aggregate
      WHERE attraction_id = $1
      "#,
      attraction_id
    )
    .fetch_all(conn)
    .await
  }

  async fn group_aggregate_by_date(
    &self,
    attraction_id: i32,
  ) -> sqlx::Result<Vec<AttractionByDate>> {
    let conn = self.connection.get();
    let rows = sqlx::query_as!(
      AttractionByDate,
      r#"
      SELECT attraction_id, DATE_TRUNC('day', at) as at
      FROM attraction_rating_aggregate 
      WHERE attraction_id = $1
      GROUP BY attraction_id, DATE_TRUNC('day', at)
      "#,
      attraction_id
    )
    //.bind(attraction_id)
    .fetch_all(conn)
    .await?;
    // let res = AttractionByDate::from_database(rows);
    Ok(rows)
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

  async fn get_info(&self, attraction_id: i32) -> sqlx::Result<AttractionInfo> {
    let conn = self.connection.get();
    sqlx::query_as!(
      AttractionInfo,
      r#"
      SELECT a.id as attraction_id, a.attraction_type_id as attraction_type_id,
      ara.average as avg_rating, a.latitude as latitude, a.longitude as longitude
      FROM attraction a
      INNER JOIN attraction_rating_aggregate ara ON a.id = ara.attraction_id
      WHERE a.id = $1 ORDER BY ara.at DESC
      LIMIT 1
      "#,
      attraction_id
    )
        .fetch_one(conn)
        .await
  }

  async fn save_similarity(
    &self,
    similarity: SimilarityBetweenAttraction,
  ) -> sqlx::Result<EntityId> {
    let conn = self.connection.get();
    sqlx::query_as!(
      EntityId,
      r#"
      INSERT INTO attraction_similarity 
      (attraction_id, to_attraction_id, similarity, at) VALUES ($1, $2, $3, $4)
      returning id
      "#,
      similarity.attraction_id,
      similarity.to_attraction_id,
      similarity.similarity,
      similarity.at
    )
    .fetch_one(conn)
    .await
  }
}
