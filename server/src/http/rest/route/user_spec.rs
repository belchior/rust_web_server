use super::user;
use crate::{db::db_client_connection, http::rest::AppState, mock, model::user::User};
use actix_web::{http::StatusCode, test, App};
use mongodb::bson;

#[cfg(test)]
mod describe_route_users {
  use super::*;
  use pretty_assertions::assert_eq;

  #[actix_rt::test]
  async fn should_match_a_specified_user() {
    // TODO find a way to reuse this block of code without start the type dependencies nightmare
    let db = db_client_connection().await.unwrap();
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(user::scope())).await;
    let users = mock::insert_users(&db).await.unwrap();

    let req = test::TestRequest::get().uri("/user/foo").to_request();
    let res = test::call_service(&mut app, req).await;

    let status = res.status();
    let user: User = test::read_body_json(res).await;
    let is_inserted_user = users
      .inserted_ids
      .values()
      .any(|id| id == &bson::Bson::ObjectId(user._id));

    assert_eq!(status, StatusCode::OK);
    assert!(is_inserted_user);

    mock::delete_users(&db, users).await;
  }
}
