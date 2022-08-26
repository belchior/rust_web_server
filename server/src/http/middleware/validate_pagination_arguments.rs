use std::future::{ready, Ready};

use actix_web::{
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  web, Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

use crate::{http::http_handler::HttpError, lib::cursor_connection::PaginationArguments};

pub struct ValidatePaginationArguments;

impl<S> Transform<S, ServiceRequest> for ValidatePaginationArguments
where
  S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse;
  type Error = Error;
  type InitError = ();
  type Transform = ValidatePaginationArgumentsMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(ValidatePaginationArgumentsMiddleware { service }))
  }
}

pub struct ValidatePaginationArgumentsMiddleware<S> {
  service: S,
}

impl<S> Service<ServiceRequest> for ValidatePaginationArgumentsMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
  S::Future: 'static,
{
  type Response = ServiceResponse;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    if let Some(query) = req.uri().query() {
      let pagination_arguments = web::Query::<PaginationArguments>::from_query(query);
      let result_error = HttpError::new("Invalid pagination arguments".to_string());
      let response = HttpResponse::BadRequest().json(result_error);

      match pagination_arguments {
        Err(_) => {
          let res = req.into_response(response);
          return Box::pin(async { Ok(res) });
        }
        Ok(pagination_arguments) => {
          if PaginationArguments::is_valid(&pagination_arguments) == false {
            let res = req.into_response(response);
            return Box::pin(async { Ok(res) });
          }
        }
      }
    }

    let fut = self.service.call(req);
    Box::pin(async move {
      let res = fut.await?;
      Ok(res)
    })
  }
}
