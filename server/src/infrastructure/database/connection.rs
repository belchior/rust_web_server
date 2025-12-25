use mongodb::{Client, Database};
use std::env;
use tokio::sync::OnceCell;
use tracing;

pub async fn get_connection() -> Database {
  static DATABASE: OnceCell<Database> = OnceCell::const_new();

  DATABASE
    .get_or_init(|| async {
      let database_uri = env::var("DATABASE_URI").unwrap();
      let database_name = env::var("DATABASE_NAME").unwrap();
      let client = Client::with_uri_str(&database_uri).await.unwrap();
      let db = client.database(database_name.as_str());

      tracing::info!("DB connection stablished with success");

      db
    })
    .await
    .clone()
}
