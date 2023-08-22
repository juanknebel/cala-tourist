use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use geoutils::{self, Distance, Location};
use std::ops::Sub;

/// The information that the implementation of the trait needs to accomplish
/// the task of calculate the similarity.
pub struct AttractionInfo {
  pub attraction_id: i32,
  pub attraction_type_id: i32,
  pub avg_rating: BigDecimal,
  pub latitude: Option<String>,
  pub longitude: Option<String>,
}

impl AttractionInfo {
  fn has_unknown_location(&self) -> bool {
    self.longitude.is_none() || self.longitude.is_none()
  }

  fn location(&self) -> Option<Location> {
    if self.has_unknown_location() {
      return None;
    }

    let Ok(lat) = self.latitude.as_ref().unwrap().parse::<f64>() else {
      return None;
    };

    let Ok(long) = self.longitude.as_ref().unwrap().parse::<f64>() else {
      return None;
    };

    Some(Location::new(lat, long))
  }

  fn distance_from(
    &self,
    other_attraction: &AttractionInfo,
  ) -> Option<Distance> {
    let location = self.location();
    let other_location = other_attraction.location();
    if location.is_none() || other_location.is_none() {
      return None;
    }
    location.unwrap().distance_to(&other_location.unwrap()).ok()
  }
}

/// An trait to define the similarity between every attraction.
pub trait Similarity {
  /// Calculate the similarity between two attractions
  /// The definition of similarity between attractions is delegated in the
  /// concrete implementations of the trait. The arguments AttractionInfo could
  /// be change to adapt every implementation necessities.
  ///
  /// # Arguments:
  /// * one_attraction: all the information necessary about an attraction.
  /// * another_attraction: all the the information necessary about another
  /// attraction.
  /// # Return:
  /// * The similarity between the two attractions.
  fn similarity_between(
    &self,
    one_attraction: &AttractionInfo,
    another_attraction: &AttractionInfo,
  ) -> BigDecimal;
}

/// The first naive Similarity Calculator.
/// In the future with more data this must be replaced, but for this POC is
/// good enough.
/// The similarity is going to be calculated based on the type of the
/// attraction, the distance between two attractions and the difference
/// between their average_ratings.
#[derive(Default, Clone)]
pub struct SimilarityCalculator;

impl Similarity for SimilarityCalculator {
  fn similarity_between(
    &self,
    one_attraction: &AttractionInfo,
    another_attraction: &AttractionInfo,
  ) -> BigDecimal {
    let mut similarity: f64 = 0.0;

    // distance component
    let distance_result = one_attraction.distance_from(&another_attraction);
    match distance_result {
      None => {},
      Some(distance) => match distance.meters().to_i128().unwrap() * 1000 {
        0..5 => similarity += 10.0,
        5..10 => similarity += 5.0,
        10..20 => similarity += 2.0,
        _ => {},
      },
    }

    // attraction type component
    if one_attraction.attraction_type_id
      == another_attraction.attraction_type_id
    {
      similarity += 10.0;
    }

    // average ratings component
    let difference = one_attraction
      .avg_rating
      .clone()
      .sub(&another_attraction.avg_rating);
    if difference.abs().to_f64().unwrap() < 0.01 {
      similarity += 10.0;
    }

    similarity /= 30.0;

    BigDecimal::from_f64(similarity).unwrap()
  }
}
