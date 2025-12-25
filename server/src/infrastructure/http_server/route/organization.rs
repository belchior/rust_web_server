use crate::{
  application,
  infrastructure::{
    database::cursor_connection::PaginationArguments,
    http_server::{middleware, utils::into_response},
  },
};
use actix_web::{Responder, Scope, web};

pub fn scope() -> Scope {
  web::scope("/organization/{login}")
    .route("", web::get().to(organization))
    .service(
      web::resource("/people")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(people)),
    )
    .service(
      web::resource("/repositories")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(repositories)),
    )
}

async fn organization(login: web::Path<String>) -> impl Responder {
  let result = application::organization::find_organization(&login).await;

  into_response(result, "Organization")
}

async fn people(
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = application::organization::find_people(&login, pagination_arguments).await;

  into_response(result, "Organization")
}

async fn repositories(
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = application::organization::find_repositories(&login, pagination_arguments).await;

  into_response(result, "Organization")
}
