use super::route;
use actix_web::{http::StatusCode, test, App};

#[cfg(test)]
mod describe_config_route {
  use super::*;

  #[actix_rt::test]
  async fn should_not_panic_when_configure_on_new_app() {
    test::init_service(App::new().configure(route::config_route)).await;
  }
}

#[cfg(test)]
mod describe_route_not_found {
  use super::*;
  use crate::http::http_handler::HttpError;

  #[actix_rt::test]
  async fn should_return_a_client_error() {
    let mut app = test::init_service(App::new().route("/", route::not_found())).await;
    let req = test::TestRequest::get().to_request();
    let res = test::call_service(&mut app, req).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
  }

  #[actix_rt::test]
  async fn should_return_the_message_resource_not_found() {
    let mut app = test::init_service(App::new().route("/", route::not_found())).await;
    let req = test::TestRequest::get().to_request();
    let res = test::call_service(&mut app, req).await;
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(body.message, "Resource not found");
  }
}
