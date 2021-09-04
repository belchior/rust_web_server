use crate::http::http_handler::HttpError;
use crate::lib::cursor_connection::PaginationArguments;
use crate::repository::organization::{find_organization_by_login, find_people_by_login, find_repositories_by_login};
use actix_web::{web, HttpResponse, Responder, Scope};
use log;
use mongodb::Database;

pub fn scope() -> Scope {
  web::scope("/organization/{login}")
    .route("", web::get().to(organization))
    .route("/people", web::get().to(people))
    .route("/repositories", web::get().to(repositories))
}

async fn organization(db: web::Data<Database>, web::Path(login): web::Path<String>) -> impl Responder {
  let result = find_organization_by_login(db.as_ref(), &login).await;

  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_document = result.unwrap();
  if let None = maybe_document {
    log::info!("Organization {} not found", login);
    return HttpResponse::NotFound().json(HttpError::new("Organization not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_document)
}

async fn people(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_people_by_login(db.as_ref(), &login, pagination_arguments).await;

  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_document = result.unwrap();
  if let None = maybe_document {
    log::info!("People from {} not found", login);
    return HttpResponse::NotFound().json(HttpError::new("People from organization not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_document)
}

async fn repositories(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_repositories_by_login(db.as_ref(), &login, pagination_arguments).await;

  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_document = result.unwrap();
  if let None = maybe_document {
    log::info!("Repositories from {} not found", login);
    return HttpResponse::NotFound().json(HttpError::new("Repositories from organization not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_document)
}
