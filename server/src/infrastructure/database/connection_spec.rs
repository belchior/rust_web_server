use crate::infrastructure::database;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn should_connect_to_database() {
  let pool = database::db_connection_poll().await.unwrap();
  let db = pool.get().await.unwrap();
  let result = db.query_one("SELECT current_database()", &[]).await.unwrap();
  let db_name: &str = result.get(0);

  assert_eq!(db_name, "database");
}
