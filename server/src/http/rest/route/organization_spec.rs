use super::organization;
use crate::{
  http::rest::AppState,
  lib::cursor_connection::CursorConnection,
  mock,
  model::{organization::Organization, repository::Repository, user::User},
};
use actix_web::{http::StatusCode, test, App};

#[cfg(test)]
mod describe_route_organization {
  use super::*;
  use pretty_assertions::assert_eq;

  #[actix_rt::test]
  async fn should_match_an_organization() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(
      App::new()
        .data(AppState { db: db.clone() })
        .service(organization::scope()),
    )
    .await;
    let req = test::TestRequest::get()
      .uri("/organization/organization_foo")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: Organization = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.login, "organization_foo");
  }

  #[actix_rt::test]
  async fn should_find_people_of_the_organization() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(
      App::new()
        .data(AppState { db: db.clone() })
        .service(organization::scope()),
    )
    .await;
    let req = test::TestRequest::get()
      .uri("/organization/organization_foo/people")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: CursorConnection<User> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.login, "user_foo");
  }

  #[actix_rt::test]
  async fn should_find_repositories_of_the_organization() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(
      App::new()
        .data(AppState { db: db.clone() })
        .service(organization::scope()),
    )
    .await;
    let req = test::TestRequest::get()
      .uri("/organization/organization_foo/repositories")
      .to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: CursorConnection<Repository> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.name, "repository_foo");
  }
}
