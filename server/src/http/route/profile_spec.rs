use crate::http::{http_handler::HttpError, route::profile};
use crate::model::{organization::Organization, user::User};
use crate::setup::mock::{make_request, HttpMethod};
use actix_web::{http::StatusCode, test};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_a_user_profile() {
  let res = make_request(HttpMethod::Get, "/profile/user_bar", profile::scope()).await;
  let status = res.status();
  let body: User = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, "user_bar");
}

#[actix_rt::test]
async fn should_match_an_organization_profile() {
  let res = make_request(HttpMethod::Get, "/profile/organization_foo", profile::scope()).await;
  let status = res.status();
  let body: Organization = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, "organization_foo");
}

#[actix_rt::test]
async fn should_return_profile_not_found() {
  let res = make_request(HttpMethod::Get, "/profile/xpto", profile::scope()).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Profile not found");
}
