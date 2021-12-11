use super::user;
use crate::{
  http::rest::AppState,
  lib::cursor_connection::CursorConnection,
  mock,
  model::{organization::Organization, repository::Repository, user::User},
};
use actix_web::{http::StatusCode, test, App};
use mongodb::bson;

#[cfg(test)]
mod describe_route_users {
  use super::*;
  use pretty_assertions::assert_eq;

  #[actix_rt::test]
  async fn should_match_a_specified_user() {
    // TODO find a way to reuse this block of code without start the type dependencies nightmare
    let (db, users) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(user::scope())).await;
    let req = test::TestRequest::get().uri("/user/user_foo").to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: User = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(users.inserted_id, bson::Bson::ObjectId(body._id));
  }

  #[actix_rt::test]
  async fn should_find_organizations_of_the_user() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(user::scope())).await;
    let req = test::TestRequest::get()
      .uri("/user/user_foo/organizations")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: CursorConnection<Organization> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.login, "organization_foo");
  }

  #[actix_rt::test]
  async fn should_find_repositories_of_the_user() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(user::scope())).await;
    let req = test::TestRequest::get().uri("/user/user_bar/repositories").to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: CursorConnection<Repository> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.name, "repository_bar");
  }

  #[actix_rt::test]
  async fn should_find_starred_repositories_of_the_user() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(user::scope())).await;
    let req = test::TestRequest::get()
      .uri("/user/user_bar/starred-repositories")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: CursorConnection<Repository> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.name, "repository_bar");
  }

  #[actix_rt::test]
  async fn should_find_followers_of_the_user() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(user::scope())).await;
    let req = test::TestRequest::get().uri("/user/user_bar/followers").to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: CursorConnection<User> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.login, "user_dee");
  }

  #[actix_rt::test]
  async fn should_find_following_of_the_user() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(user::scope())).await;
    let req = test::TestRequest::get().uri("/user/user_bar/following").to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: CursorConnection<User> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.login, "user_dee");
  }
}
