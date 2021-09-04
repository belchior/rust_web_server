use crate::http::http_handler::{to_response, HttpError};
use crate::lib::cursor_connection::PaginationArguments;
use crate::repository::organization::{find_organization_by_login, find_people_by_login, find_repositories_by_login};
use actix_web::{web, HttpResponse, Responder, Scope};
use mongodb::Database;

pub fn scope() -> Scope {
  web::scope("/organization/{login}")
    .route("", web::get().to(organization))
    .route("/people", web::get().to(people))
    .route("/repositories", web::get().to(repositories))
}

async fn organization(db: web::Data<Database>, web::Path(login): web::Path<String>) -> impl Responder {
  let result = find_organization_by_login(db.as_ref(), &login).await;

  to_response(result, "Organization")
}

async fn people(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    let result_error = HttpError::new("Invalid pagination arguments".to_string()).status(400);
    return HttpResponse::BadRequest().json(result_error);
  }

  let result = find_people_by_login(db.as_ref(), &login, pagination_arguments).await;

  to_response(result, "People from organization")
}

async fn repositories(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    let result_error = HttpError::new("Invalid pagination arguments".to_string()).status(400);
    return HttpResponse::BadRequest().json(result_error);
  }

  let result = find_repositories_by_login(db.as_ref(), &login, pagination_arguments).await;

  to_response(result, "Repositories from organization")
}
