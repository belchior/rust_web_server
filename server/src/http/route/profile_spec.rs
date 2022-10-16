use crate::http::{http_handler::HttpError, route::profile};
use crate::model::{organization::Organization, user::User};
use crate::setup::mock;
use actix_web::{http::StatusCode, test};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_a_user_profile() {
  let sufix = mock::random_sufix();
  let login = format!("user_bar_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/profile/{login}"),
    profile::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: User = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, format!("user_bar_{sufix}"));
}

#[actix_rt::test]
async fn should_match_an_organization_profile() {
  let sufix = mock::random_sufix();
  let login = format!("organization_foo_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/profile/{login}"),
    profile::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: Organization = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, format!("organization_foo_{sufix}"));
}

#[actix_rt::test]
async fn should_return_profile_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("xpto_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/profile/{login}"),
    profile::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Profile not found");
}
