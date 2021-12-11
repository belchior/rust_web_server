use super::profile;
use crate::{
  http::{http_handler::HttpError, rest::AppState},
  mock,
  model::{organization::Organization, user::User},
};
use actix_web::{http::StatusCode, test, App};

#[cfg(test)]
mod describe_profile {
  use super::*;
  use pretty_assertions::assert_eq;

  #[actix_rt::test]
  async fn should_match_a_user_profile() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(profile::scope())).await;
    let req = test::TestRequest::get().uri("/profile/user_bar").to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: User = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.login, "user_bar");
  }

  #[actix_rt::test]
  async fn should_match_an_organization_profile() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(profile::scope())).await;
    let req = test::TestRequest::get().uri("/profile/organization_foo").to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: Organization = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.login, "organization_foo");
  }

  #[actix_rt::test]
  async fn should_return_profile_not_found() {
    let (db, _) = mock::setup().await;
    let mut app = test::init_service(App::new().data(AppState { db: db.clone() }).service(profile::scope())).await;
    let req = test::TestRequest::get().uri("/profile/xpto").to_request();
    let res = test::call_service(&mut app, req).await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "Profile not found");
  }
}
