use crate::http::{http_handler::HttpError, route::organization};
use crate::lib::cursor_connection::CursorConnection;
use crate::model::{organization::Organization, repository::Repository, user::User};
use crate::setup::mock;
use actix_web::{http::StatusCode, test};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_an_organization() {
  let sufix = mock::random_sufix();
  let login = format!("organization_foo_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/organization/{login}"),
    organization::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: Organization = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, login);
}

#[actix_rt::test]
async fn should_find_people_of_the_organization() {
  let sufix = mock::random_sufix();
  let login = format!("organization_foo_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/organization/{login}/people"),
    organization::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.login, format!("user_foo_{sufix}"));
}

#[actix_rt::test]
async fn should_not_find_people_of_the_organization() {
  let sufix = mock::random_sufix();
  let login = format!("empty_org_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/organization/{login}/people"),
    organization::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_people_of_a_unknown_organization() {
  let sufix = mock::random_sufix();
  let login = format!("organization_xxx_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/organization/{login}/people"),
    organization::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Organization not found");
}

#[actix_rt::test]
async fn should_find_repositories_of_the_organization() {
  let sufix = mock::random_sufix();
  let login = format!("organization_acme_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/organization/{login}/repositories"),
    organization::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.name, format!("repository_tux_{sufix}"));
}

#[actix_rt::test]
async fn should_not_find_repositories_of_the_organization() {
  let sufix = mock::random_sufix();
  let login = format!("empty_org_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/organization/{login}/repositories"),
    organization::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_repositories_of_a_unknown_organization() {
  let sufix = mock::random_sufix();
  let login = format!("organization_xxx_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/organization/{login}/repositories"),
    organization::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Organization not found");
}
