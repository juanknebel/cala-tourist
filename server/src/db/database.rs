use sqlx::{postgres::Postgres, Pool};
use std::env::VarError;

type DbPool = Pool<Postgres>;

#[derive(Clone)]
pub struct DbConnection {
  pool: DbPool,
}

impl DbConnection {
  pub async fn new(db_var: Result<String, VarError>) -> Self {
    let db_url = db_var.expect("Database url not defined");
    let pool = Pool::<Postgres>::connect(db_url.as_str())
      .await
      .expect("Cannot create the database pool.");
    DbConnection {
      pool,
    }
  }

  pub fn get(&self) -> &DbPool {
    &self.pool
  }
}
