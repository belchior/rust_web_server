use crate::http::cursor_connection::PaginationArguments;
use crate::http::http_handler::HttpError;
use crate::repository::repository::find_repositories_by_login;
use crate::repository::user::{
  find_followers_by_login, find_following_by_login, find_starred_repositories_by_login, find_user_by_login,
};
use actix_web::{get, web, HttpResponse, Responder};
use log;
use mongodb::Database;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserQueryString {
  pub orgs_limit: Option<u32>,
}

#[get("/user/{login}")]
pub async fn user(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  query_string: web::Query<UserQueryString>,
) -> impl Responder {
  let organizations_limit = std::cmp::min(100, query_string.orgs_limit.unwrap_or(10));
  let result = find_user_by_login(db.as_ref(), &login, &organizations_limit).await;

  // TODO find a better way to abtract common error handlers, maybe with a middeware
  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_document = result.unwrap();
  if let None = maybe_document {
    log::info!("User {} not found", login);
    return HttpResponse::NotFound().json(HttpError::new("User not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_document)
}

#[get("/user/{login}/repositories")]
pub async fn repositories(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::NotFound().json(HttpError::new("Invalid pagination arguments", Some(404)));
  }

  let result = find_repositories_by_login(db.as_ref(), &login, pagination_arguments).await;

  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_documents = result.unwrap();
  if let None = maybe_documents {
    log::info!("Repositories not found");
    return HttpResponse::NotFound().json(HttpError::new("Repositories not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_documents)
}

#[get("/user/{login}/starred-repositories")]
pub async fn starred_repositories(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::NotFound().json(HttpError::new("Invalid pagination arguments", Some(404)));
  }

  let result = find_starred_repositories_by_login(db.as_ref(), &login, pagination_arguments).await;

  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_documents = result.unwrap();
  if let None = maybe_documents {
    log::info!("Starred repositories not found");
    return HttpResponse::NotFound().json(HttpError::new("Starred repositories not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_documents)
}

#[get("/user/{login}/followers")]
pub async fn followers(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::NotFound().json(HttpError::new("Invalid pagination arguments", Some(404)));
  }

  let result = find_followers_by_login(db.as_ref(), &login, pagination_arguments).await;

  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_documents = result.unwrap();
  if let None = maybe_documents {
    log::info!("Followers not found");
    return HttpResponse::NotFound().json(HttpError::new("Followers not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_documents)
}

#[get("/user/{login}/following")]
pub async fn following(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::NotFound().json(HttpError::new("Invalid pagination arguments", Some(404)));
  }

  let result = find_following_by_login(db.as_ref(), &login, pagination_arguments).await;

  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_documents = result.unwrap();
  if let None = maybe_documents {
    log::info!("Following not found");
    return HttpResponse::NotFound().json(HttpError::new("Following not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_documents)
}
