use crate::http::http_handler::to_response;
use crate::http::rest::{middleware, AppState};
use crate::lib::cursor_connection::PaginationArguments;
use crate::repository::organization::{find_organization_by_login, find_people_by_login, find_repositories_by_login};
use actix_web::{web, Responder, Scope};

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

async fn organization(state: web::Data<AppState>, web::Path(login): web::Path<String>) -> impl Responder {
  let result = find_organization_by_login(&state.db, &login).await;

  to_response(result, "Organization")
}

async fn people(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_people_by_login(&state.db, &login, pagination_arguments).await;

  to_response(result, "People from organization")
}

async fn repositories(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_repositories_by_login(&state.db, &login, pagination_arguments).await;

  to_response(result, "Repositories from organization")
}
