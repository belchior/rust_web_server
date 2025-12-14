use crate::{
  application,
  infrastructure::{
    database::cursor_connection::PaginationArguments,
    http_server::{AppState, middleware, utils::into_response},
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

async fn organization(state: web::Data<AppState>, login: web::Path<String>) -> impl Responder {
  let db = state.poll.get().await.unwrap();
  let result = application::organization::find_organization(&db, &login).await;

  into_response(result, "Organization")
}

async fn people(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db = state.poll.get().await.unwrap();
  let result = application::organization::find_people(&db, &login, pagination_arguments).await;

  into_response(result, "Organization")
}

async fn repositories(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db = state.poll.get().await.unwrap();
  let result = application::organization::find_repositories(&db, &login, pagination_arguments).await;

  into_response(result, "Organization")
}
