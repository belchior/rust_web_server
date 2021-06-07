use mongodb::{Client, Database};
use std::env;

pub async fn get_connection() -> Result<Database, mongodb::error::Error> {
  let database_uri = env::var("DATABASE_URI").unwrap();
  let client = Client::with_uri_str(&database_uri).await?;
  let db = client.database("learning-graphql");
  Ok(db)
}
