use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use log;
use std::env;
use tokio_postgres;

pub async fn db_connection_poll() -> Result<Pool, tokio_postgres::Error> {
  let mut pg_config = tokio_postgres::Config::new();
  pg_config.user(env::var("POSTGRES_USER").unwrap().as_str());
  pg_config.password(env::var("POSTGRES_PASSWORD").unwrap().as_str());
  pg_config.host(env::var("POSTGRES_HOST").unwrap().as_str());
  pg_config.port(env::var("POSTGRES_PORT").unwrap().parse().unwrap());
  pg_config.dbname(env::var("POSTGRES_DB").unwrap().as_str());

  let conn_number: usize = env::var("POSTGRES_CONNECTIONS_NUMBER").unwrap().parse().unwrap();

  let mgr_config = ManagerConfig {
    recycling_method: RecyclingMethod::Fast,
  };
  let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
  let pool = Pool::builder(mgr).max_size(conn_number).build().unwrap();

  log::info!(
    "DB connection with `{}` stablished with success",
    env::var("POSTGRES_DB").unwrap()
  );
  Ok(pool)
}
