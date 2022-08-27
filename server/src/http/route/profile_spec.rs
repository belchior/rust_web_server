use crate::http::{http_handler::HttpError, route::profile, AppState};
use crate::model::{organization::Organization, user::User};
use crate::setup::mock;
use actix_web::{http::StatusCode, test, web, App};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_a_user_profile() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(profile::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/profile/user_bar").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: User = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, "user_bar");
}

#[actix_rt::test]
async fn should_match_an_organization_profile() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(profile::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/profile/organization_foo").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: Organization = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, "organization_foo");
}

#[actix_rt::test]
async fn should_return_profile_not_found() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(profile::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/profile/xpto").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "Profile not found");
}
