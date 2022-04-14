use crate::http::rest::middleware;
use actix_web::{http::StatusCode, test, web, App, HttpResponse};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn validation_should_not_fail_when_querystring_be_empty() {
  let querystring = "/?";
  let app = test::init_service(
    App::new()
      .wrap(middleware::ValidatePaginationArguments)
      .route("/", web::get().to(|| HttpResponse::Ok())),
  )
  .await;
  let req = test::TestRequest::get().uri(querystring).to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();

  assert_eq!(status, StatusCode::OK);
}

#[actix_rt::test]
async fn validation_should_not_fail_when_querystring_has_non_pagination_arguments() {
  let querystring = "/?foo=bar";
  let app = test::init_service(
    App::new()
      .wrap(middleware::ValidatePaginationArguments)
      .route("/", web::get().to(|| HttpResponse::Ok())),
  )
  .await;
  let req = test::TestRequest::get().uri(querystring).to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();

  assert_eq!(status, StatusCode::OK);
}

#[actix_rt::test]
async fn validation_should_fail_when_querystring_has_both_first_and_last() {
  let querystring = "/?first=1&last=2";
  let app = test::init_service(
    App::new()
      .wrap(middleware::ValidatePaginationArguments)
      .route("/", web::get().to(|| HttpResponse::Ok())),
  )
  .await;
  let req = test::TestRequest::get().uri(querystring).to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();

  assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn validation_should_fail_with_bad_request_when_querystring_has_wrong_type_for_first_or_last() {
  let querystring = "/?first=abc";
  let app = test::init_service(
    App::new()
      .wrap(middleware::ValidatePaginationArguments)
      .route("/", web::get().to(|| HttpResponse::Ok())),
  )
  .await;
  let req = test::TestRequest::get().uri(querystring).to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();

  assert_eq!(status, StatusCode::BAD_REQUEST);

  let querystring = "/?last=abc";
  let req = test::TestRequest::get().uri(querystring).to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();

  assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
async fn validation_should_fail_when_querystring_has_both_after_and_before() {
  let querystring = "/?after=a1a1a1&before=b2b2b2";
  let app = test::init_service(
    App::new()
      .wrap(middleware::ValidatePaginationArguments)
      .route("/", web::get().to(|| HttpResponse::Ok())),
  )
  .await;
  let req = test::TestRequest::get().uri(querystring).to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();

  assert_eq!(status, StatusCode::BAD_REQUEST);
}
