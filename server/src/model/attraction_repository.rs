use diesel::{prelude::*, QueryDsl, QueryResult, RunQueryDsl};

use crate::db::database::DbConnection;

use super::attraction::{
  Attraction, AttractionRating, AttractionRatingAggregate, AttractionType, City,
};
use crate::schema::{
  attraction::dsl::{attraction, *},
  attraction_rating::dsl::attraction_rating,
  attraction_rating_aggregate::dsl::attraction_rating_aggregate,
  attraction_type::dsl::*,
  city::dsl::*,
};

pub trait AttractionRepository {
  fn list(&self) -> QueryResult<Vec<(Attraction, AttractionType, City)>>;
  fn get_attraction(
    &self,
    id: i32,
  ) -> QueryResult<(Attraction, AttractionType, City)>;
  fn list_ratings(&self) -> QueryResult<Vec<(AttractionRating, Attraction)>>;
  fn list_aggregates(
    &self,
  ) -> QueryResult<Vec<(AttractionRatingAggregate, Attraction)>>;
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

// TODO: this should be optimized to use some kind of cache and avoid the
// joins. But for now this is the best solution.
impl AttractionRepository for PgAttractionRepository {
  fn list(&self) -> QueryResult<Vec<(Attraction, AttractionType, City)>> {
    let mut conn = self.connection.get();
    attraction
      .inner_join(attraction_type)
      .inner_join(city)
      .limit(20)
      .load::<(Attraction, AttractionType, City)>(&mut conn)
  }

  fn get_attraction(
    &self,
    the_attraction_id: i32,
  ) -> QueryResult<(Attraction, AttractionType, City)> {
    todo!()
  }

  fn list_ratings(&self) -> QueryResult<Vec<(AttractionRating, Attraction)>> {
    let mut conn = self.connection.get();
    attraction_rating
      .inner_join(attraction)
      .limit(20)
      .load::<(AttractionRating, Attraction)>(&mut conn)
  }

  fn list_aggregates(
    &self,
  ) -> QueryResult<Vec<(AttractionRatingAggregate, Attraction)>> {
    let mut conn = self.connection.get();
    attraction_rating_aggregate
      .inner_join(attraction)
      .limit(20)
      .load::<(AttractionRatingAggregate, Attraction)>(&mut conn)
  }
}
