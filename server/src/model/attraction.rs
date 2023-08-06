use bigdecimal;
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

impl AttractionType {
  pub fn get_description(&self) -> String {
    self.description.to_string()
  }
}

#[derive(FromRow)]
pub struct City {
  pub id: i32,
  pub description: String,
  pub country_id: i32,
}

impl City {
  pub fn get_description(&self) -> String {
    self.description.to_string()
  }
}

#[derive(FromRow)]
pub struct AttractionRating {
  pub id: i32,
  pub at: NaiveDateTime,
  pub attraction_id: i32,
  pub rate: bigdecimal::BigDecimal,
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

  pub fn get_attraction_id(&self) -> i32 {
    self.attraction_id
  }
}

#[derive(FromRow)]
pub struct AttractionRatingAggregate {
  pub id: i32,
  pub attraction_id: i32,
  pub at: NaiveDateTime,
  pub average: bigdecimal::BigDecimal,
  pub ninety_five_percentile: bigdecimal::BigDecimal,
  pub ninety_nine_percentile: bigdecimal::BigDecimal,
}

impl AttractionRatingAggregate {
  pub fn get_attraction_id(&self) -> i32 {
    self.attraction_id
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
