use super::organization;
use crate::{
  http::{http_handler::HttpError, rest::AppState},
  lib::cursor_connection::CursorConnection,
  mock,
  model::{organization::Organization, repository::Repository, user::User},
};
use actix_web::{http::StatusCode, test, web, App};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_an_organization() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(organization::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/organization/organization_foo")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: Organization = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, "organization_foo");
}

#[actix_rt::test]
async fn should_find_people_of_the_organization() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(organization::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/organization/organization_foo/people")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.login, "user_foo");
}

#[actix_rt::test]
async fn should_not_find_people_of_the_organization() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(organization::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/organization/empty_org/people")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_people_of_a_unknown_organization() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(organization::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/organization/organization_xxx/people")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Organization not found");
}

#[actix_rt::test]
async fn should_find_repositories_of_the_organization() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(organization::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/organization/organization_acme/repositories")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.name, "repository_tux");
}

#[actix_rt::test]
async fn should_not_find_repositories_of_the_organization() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(organization::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/organization/empty_org/repositories")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_repositories_of_a_unknown_organization() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(organization::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/organization/organization_xxx/repositories")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Organization not found");
}
