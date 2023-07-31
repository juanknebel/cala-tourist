use bigdecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::attraction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
//#[diesel(belongs_to(AttractionType))]
pub struct Attraction {
  pub id: i32,
  description: String,
  city_id: i32,
  latitude: Option<String>,
  longitude: Option<String>,
  attraction_type_id: i32,
}

impl Attraction {
  pub fn get_id(&self) -> i32 {
    self.id
  }

  pub fn get_description(&self) -> String {
    self.description.to_string()
  }

  pub fn get_city_id(&self) -> i32 {
    self.city_id
  }

  pub fn get_attraction_type_id(&self) -> i32 {
    self.attraction_type_id
  }

  pub fn get_latitude(&self) -> Option<String> {
    self.latitude.clone()
  }

  pub fn get_longitude(&self) -> Option<String> {
    self.longitude.clone()
  }
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::attraction_type)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AttractionType {
  id: i32,
  code: String,
  description: String,
}

impl AttractionType {
  pub fn get_description(&self) -> String {
    self.description.to_string()
  }
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::city)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct City {
  id: i32,
  description: String,
  country_id: i32,
}

impl City {
  pub fn get_description(&self) -> String {
    self.description.to_string()
  }
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::attraction_rating)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AttractionRating {
  id: i32,
  at: NaiveDateTime,
  attraction_id: i32,
  rate: bigdecimal::BigDecimal,
}

impl AttractionRating {
  pub fn get_rate(&self) -> bigdecimal::BigDecimal {
    self.rate.clone()
  }

  pub fn get_at(&self) -> NaiveDateTime {
    self.at
  }

  pub fn get_id(&self) -> i32 {
    self.id
  }
}

#[derive(Identifiable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::attraction_rating_aggregate)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AttractionRatingAggregate {
  id: i32,
  attraction_id: i32,
  at: NaiveDateTime,
  average: bigdecimal::BigDecimal,
  ninety_five_percentile: bigdecimal::BigDecimal,
  ninety_nine_percentile: bigdecimal::BigDecimal,
}

impl AttractionRatingAggregate {
  pub fn get_attraction_id(&self) -> i32 {
    self.attraction_id
  }
}
