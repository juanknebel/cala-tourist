use chrono::{NaiveDateTime, Utc};
use dotenv::dotenv;
use fake::{
  faker::{address::raw::*, name::raw::*},
  locales::*,
  Fake,
};
use rust_decimal::Decimal;
use sqlx::{postgres::Postgres, Decode, FromRow, Pool};

#[derive(Debug, FromRow)]
struct Attraction {
  id: i32,
  description: String,
  city_id: i32,
  latitude: String,
  longitude: String,
  attraction_type_id: i32,
}

impl Attraction {
  fn new(
    id: i32,
    description: impl Into<String>,
    city_id: i32,
    latitude: impl Into<String>,
    longitude: impl Into<String>,
    attraction_type_id: i32,
  ) -> Self {
    Attraction {
      id,
      description: description.into(),
      city_id,
      latitude: latitude.into(),
      longitude: longitude.into(),
      attraction_type_id,
    }
  }
}

#[derive(Debug, FromRow)]
struct AttractionWithType {
  attraction_id: i32,
  description: String,
  city_id: i32,
  latitude: String,
  longitude: String,
  attraction_type_id: i32,
  code: String,
  type_description: String,
}

#[derive(Debug, FromRow)]
struct AttractionRating {
  id: i32,
  at: NaiveDateTime,
  attraction_id: i32,
  rate: Decimal,
}

fn random_attraction() -> Attraction {
  // println!("lorem {:?}", Words(5..10).fake::<Vec<String>>());
  Attraction::new(
    0,
    Name(EN).fake::<String>(),
    1,
    Latitude(EN).fake::<String>(),
    Longitude(EN).fake::<String>(),
    (1..=5).fake::<i32>(),
  )
}

async fn get_db_pool(connection: &str) -> Result<Pool<Postgres>, sqlx::Error> {
  let pool = Pool::<Postgres>::connect(connection).await?;
  Ok(pool)
}

async fn generate_attractions(pool: &Pool<Postgres>) {
  for _n in 1..100 {
    let rnd_attraction = random_attraction();
    match sqlx::query_as::<_, Attraction>(
      "INSERT INTO attraction (description, city_id, latitude, longitude, \
       attraction_type_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(rnd_attraction.description.to_string())
    .bind(rnd_attraction.city_id)
    .bind(rnd_attraction.latitude.to_string())
    .bind(rnd_attraction.longitude.to_string())
    .bind(rnd_attraction.attraction_type_id)
    .fetch_one(pool)
    .await
    {
      Ok(inserted_attraction) => {
        // println!(
        //  "Inserted attraction with id: {}",
        //  inserted_attraction.id
        //);
        generate_rating(&inserted_attraction, &pool).await
      },
      Err(e) => println!("Error inserting an attraction: {}", e.to_string()),
    }
  }
}

fn random_ratings(an_attraction: &Attraction) -> AttractionRating {
  AttractionRating {
    id: 0,
    at: Utc::now().naive_utc(),
    attraction_id: an_attraction.id,
    rate: Decimal::new((0..=100).fake::<i64>(), 2),
  }
}

async fn generate_rating(an_attraction: &Attraction, pool: &Pool<Postgres>) {
  for _n in 1..50 {
    let rnd_rating = random_ratings(&an_attraction);
    match sqlx::query_as::<_, AttractionRating>(
      "INSERT INTO attraction_rating (at, attraction_id, rate) VALUES ($1, \
       $2, $3) RETURNING *",
    )
    .bind(rnd_rating.at)
    .bind(rnd_rating.attraction_id)
    .bind(rnd_rating.rate)
    .fetch_one(pool)
    .await
    {
      Ok(inserted_rating) => {
        // println!("Inserted rating with id: {}", inserted_rating.id)
      },
      Err(e) => println!("Error inserting a rating: {}", e.to_string()),
    }
  }
}

async fn testing_connection(pool: &Pool<Postgres>) {
  match sqlx::query_as::<_, AttractionWithType>(
    r#"
    SELECT a.id as attraction_id, a.description, a.latitude, a.city_id, a.longitude,
    a.attraction_type_id, at.code, at.description as type_description
    FROM attraction AS a INNER JOIN attraction_type AS at
    ON a.attraction_type_id = at.id
    WHERE a.id = $1
    "#).bind(20471).fetch_one(pool).await
  {
    Ok(attr_type) => {
      dbg!(attr_type);
    }
    Err(e) => {println!("Error: {}", e.to_string())}
  }
}

#[async_std::main]
async fn main() -> Result<(), ()> {
  dotenv().ok();
  let db_uri = std::env::var("DATABASE_URL").unwrap();
  let pool = get_db_pool(&db_uri)
    .await
    .expect("Error creating the database pool");
  generate_attractions(&pool).await;
  // testing_connection(&pool).await;
  Ok(())
}
