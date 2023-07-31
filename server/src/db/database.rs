use diesel::{
  r2d2,
  r2d2::{ConnectionManager, PooledConnection},
  PgConnection,
};
use std::env::VarError;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct DbConnection {
  pool: DbPool,
}

impl DbConnection {
  pub fn new(db_var: Result<String, VarError>) -> Self {
    let db_url = db_var.expect("Database url not defined");
    let manager = ConnectionManager::<PgConnection>::new(db_url.as_str());
    let the_pool = r2d2::Pool::builder()
      .build(manager)
      .expect("Cannot create pool connection");
    DbConnection {
      pool: the_pool,
    }
  }

  pub fn get(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
    self.pool.get().expect("Cannot access the pool")
  }
}
