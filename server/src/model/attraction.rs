use bigdecimal::{self, BigDecimal};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Attraction {
  pub id: i32,
  pub description: String,
  pub city_id: i32,
  pub latitude: Option<String>,
  pub longitude: Option<String>,
  pub attraction_type_id: i32,
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

#[derive(FromRow)]
pub struct AttractionType {
  pub id: i32,
  pub code: String,
  pub description: String,
}

#[derive(FromRow)]
pub struct City {
  pub id: i32,
  pub description: String,
  pub country_id: i32,
}

#[derive(FromRow, Debug, Clone)]
pub struct AttractionRating {
  pub id: i32,
  pub at: NaiveDateTime,
  pub attraction_id: i32,
  pub rate: BigDecimal,
}

impl AttractionRating {
  pub fn get_rate(&self) -> BigDecimal {
    self.rate.clone()
  }

  pub fn get_at(&self) -> NaiveDateTime {
    self.at
  }

  pub fn get_id(&self) -> i32 {
    self.id
  }

  pub fn get_attraction_id(&self) -> i32 {
    self.attraction_id
  }
}

#[derive(FromRow)]
pub struct AttractionRatingAggregate {
  pub id: i32,
  pub attraction_id: i32,
  pub at: NaiveDateTime,
  pub average: BigDecimal,
  pub ninety_five_percentile: BigDecimal,
  pub ninety_nine_percentile: BigDecimal,
}

impl AttractionRatingAggregate {
  pub fn get_attraction_id(&self) -> i32 {
    self.attraction_id
  }

  pub fn get_at(&self) -> NaiveDateTime {
    self.at
  }

  pub fn get_average(&self) -> BigDecimal {
    self.average.clone()
  }

  pub fn get_95_percentile(&self) -> BigDecimal {
    self.ninety_five_percentile.clone()
  }

  pub fn get_99_percentile(&self) -> BigDecimal {
    self.ninety_nine_percentile.clone()
  }
}

#[derive(FromRow)]
pub struct FullAttraction {
  pub attraction_id: i32,
  pub description: String,
  pub city: String,
  pub attraction_type: String,
}

impl FullAttraction {
  pub fn get_attraction_id(&self) -> i32 {
    self.attraction_id
  }

  pub fn get_description(&self) -> String {
    self.description.to_string()
  }

  pub fn get_city(&self) -> String {
    self.city.to_string()
  }

  pub fn get_attraction_type(&self) -> String {
    self.attraction_type.to_string()
  }
}

#[derive(FromRow, Hash, Eq, PartialEq, Clone, Debug)]
pub struct AttractionByDate {
  pub attraction_id: i32,
  pub at: Option<NaiveDateTime>,
}
