use crate::http::{http_handler::HttpError, route::organization};
use crate::lib::cursor_connection::CursorConnection;
use crate::model::{organization::Organization, repository::Repository, user::User};
use crate::setup::mock::{make_request, HttpMethod};
use actix_web::{http::StatusCode, test};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_an_organization() {
  let res = make_request(HttpMethod::Get, "/organization/organization_foo", organization::scope()).await;
  let status = res.status();
  let body: Organization = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, "organization_foo");
}

#[actix_rt::test]
async fn should_find_people_of_the_organization() {
  let res = make_request(
    HttpMethod::Get,
    "/organization/organization_foo/people",
    organization::scope(),
  )
  .await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.login, "user_foo");
}

#[actix_rt::test]
async fn should_not_find_people_of_the_organization() {
  let res = make_request(HttpMethod::Get, "/organization/empty_org/people", organization::scope()).await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_people_of_a_unknown_organization() {
  let res = make_request(
    HttpMethod::Get,
    "/organization/organization_xxx/people",
    organization::scope(),
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Organization not found");
}

#[actix_rt::test]
async fn should_find_repositories_of_the_organization() {
  let res = make_request(
    HttpMethod::Get,
    "/organization/organization_acme/repositories",
    organization::scope(),
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.name, "repository_tux");
}

#[actix_rt::test]
async fn should_not_find_repositories_of_the_organization() {
  let res = make_request(
    HttpMethod::Get,
    "/organization/empty_org/repositories",
    organization::scope(),
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_repositories_of_a_unknown_organization() {
  let res = make_request(
    HttpMethod::Get,
    "/organization/organization_xxx/repositories",
    organization::scope(),
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Organization not found");
}
