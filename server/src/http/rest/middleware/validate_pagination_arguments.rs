use std::pin::Pin;
use std::task::{Context, Poll};

use crate::http::http_handler::HttpError;
use crate::lib::cursor_connection::PaginationArguments;
use actix_service::{Service, Transform};
use actix_web::web::Query;
use actix_web::HttpResponse;
use actix_web::{
  dev::{ServiceRequest, ServiceResponse},
  Error,
};
use futures::future::{ok, Ready};
use std::future::Future;

pub struct ValidatePaginationArguments;

impl<S> Transform<S> for ValidatePaginationArguments
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
  S::Future: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse;
  type Error = Error;
  type InitError = ();
  type Transform = ValidatePaginationArgumentsMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(ValidatePaginationArgumentsMiddleware { service })
  }
}

pub struct ValidatePaginationArgumentsMiddleware<S> {
  service: S,
}
impl<S> Service for ValidatePaginationArgumentsMiddleware<S>
where
  S: Service<Request = ServiceRequest, Response = ServiceResponse, Error = Error>,
  S::Future: 'static,
{
  type Request = ServiceRequest;
  type Response = ServiceResponse;
  type Error = Error;
  type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

  fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&mut self, req: ServiceRequest) -> Self::Future {
    if let Some(query) = req.uri().query() {
      let pagination_arguments = Query::<PaginationArguments>::from_query(query).unwrap();
      if PaginationArguments::is_valid(&pagination_arguments) == false {
        let (request, _) = req.into_parts();
        let result_error = HttpError::new("Invalid pagination arguments".to_string());
        let response = HttpResponse::BadRequest().json(result_error);
        let service_response = ServiceResponse::new(request, response);
        return Box::pin(async { Ok(service_response) });
      }
    }

    let fut = self.service.call(req);
    Box::pin(async {
      let res = fut.await?;
      Ok(res)
    })
  }
}
