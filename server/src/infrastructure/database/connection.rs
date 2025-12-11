use mongodb::{Client, Database};
use std::env;
use tracing;

pub async fn db_client_connection() -> Result<Database, mongodb::error::Error> {
  let database_uri = env::var("DATABASE_URI").unwrap();
  let database_name = env::var("DATABASE_NAME").unwrap();
  let client = Client::with_uri_str(&database_uri).await;

  match client {
    Ok(client) => {
      tracing::info!("DB connection stablished with success");
      let db = client.database(database_name.as_str());
      Ok(db)
    }
    Err(err) => {
      tracing::error!("DB connection fails with the error\n{}", err);
      Err(err)
    }
  }
}
