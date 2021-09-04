use crate::http::http_handler::HttpError;
use crate::lib::cursor_connection::PaginationArguments;
use crate::repository::repository::find_repositories_by_login;
use crate::repository::user::{
  find_followers_by_login, find_following_by_login, find_organizations_by_login, find_starred_repositories_by_login,
  find_user_by_login,
};
use actix_web::{web, HttpResponse, Responder, Scope};
use log;
use mongodb::Database;

pub fn scope() -> Scope {
  web::scope("/user/{login}")
    .route("", web::get().to(user))
    .route("/organizations", web::get().to(organizations))
    .route("/repositories", web::get().to(repositories))
    .route("/starred-repositories", web::get().to(starred_repositories))
    .route("/followers", web::get().to(followers))
    .route("/following", web::get().to(following))
}

async fn user(db: web::Data<Database>, web::Path(login): web::Path<String>) -> impl Responder {
  let result = find_user_by_login(db.as_ref(), &login).await;

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

async fn organizations(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  // TODO convert this common validation into a actix_web::middleware
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::BadRequest().json(HttpError::new("Invalid pagination arguments", Some(400)));
  }

  let result = find_organizations_by_login(db.as_ref(), &login, pagination_arguments).await;

  if let Err(err) = result {
    log::error!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None));
  }

  let maybe_documents = result.unwrap();
  if let None = maybe_documents {
    log::info!("Organizations not found");
    return HttpResponse::NotFound().json(HttpError::new("Organizations not found", Some(404)));
  }

  HttpResponse::Ok().json(maybe_documents)
}

async fn repositories(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::BadRequest().json(HttpError::new("Invalid pagination arguments", Some(400)));
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

async fn starred_repositories(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::BadRequest().json(HttpError::new("Invalid pagination arguments", Some(400)));
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

async fn followers(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::BadRequest().json(HttpError::new("Invalid pagination arguments", Some(400)));
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

async fn following(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    return HttpResponse::BadRequest().json(HttpError::new("Invalid pagination arguments", Some(400)));
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
