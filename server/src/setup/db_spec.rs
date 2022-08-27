use crate::setup::db::*;
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_connect_to_database() {
  let poll = db_connection_poll().await.unwrap();
  let db_client = poll.get().await.unwrap();
  let result = db_client.query_one("SELECT current_database()", &[]).await.unwrap();
  let db_name: &str = result.get(0);

  assert_eq!(db_name, "database");
}
