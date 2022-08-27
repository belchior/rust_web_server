use crate::setup::db::db_client_connection;
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_connect_to_database_test_learning() {
  let db = db_client_connection().await.unwrap();

  assert_eq!(db.name(), "test_database");
}
