use crate::http::cursor_connection::PaginationArguments;
use crate::http::http_handler::HttpError;
use crate::repository::organization::{find_organization_by_login, find_people_by_login};
use actix_web::{get, web, HttpResponse, Responder};
use log;
use mongodb::Database;

#[get("/organization/{login}")]
pub async fn organization(db: web::Data<Database>, web::Path(login): web::Path<String>) -> impl Responder {
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

#[get("/organization/{login}/people")]
pub async fn people(
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
    return HttpResponse::NotFound().json(HttpError::new("People from not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_document)
}
