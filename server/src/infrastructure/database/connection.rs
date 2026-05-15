use chrono::{DateTime, Utc};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::{env, time::Duration};
use tokio::sync::OnceCell;
use tokio_postgres;
use tracing;

pub async fn get_connection() -> Pool {
  static DATABASE: OnceCell<Pool> = OnceCell::const_new();

  DATABASE
    .get_or_init(|| async {
      let mut pg_config = tokio_postgres::Config::new();
      pg_config.dbname(env::var("DATABASE_DB").unwrap().as_str());
      pg_config.host(env::var("DATABASE_HOST").unwrap().as_str());
      pg_config.password(env::var("DATABASE_PASSWORD").unwrap().as_str());
      pg_config.port(env::var("DATABASE_PORT").unwrap().parse().unwrap());
      pg_config.user(env::var("DATABASE_USER").unwrap().as_str());
      pg_config.connect_timeout(Duration::from_millis(
        env::var("DATABASE_TIMEOUT").unwrap().parse().unwrap(),
      ));
      pg_config.options("-c statement_timeout=30s");

      let conn_number: usize = env::var("DATABASE_CONNECTIONS_NUMBER").unwrap().parse().unwrap();

      let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
      };
      let mgr = Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
      let pool = Pool::builder(mgr).max_size(conn_number).build().unwrap();

      let result = pool.get().await.unwrap().query("select now()", &[]).await;
      tracing::info!(
        "DB connection stablished with success at {}",
        result.unwrap()[0].get::<_, DateTime<Utc>>("now")
      );

      pool
    })
    .await
    .clone()
}
