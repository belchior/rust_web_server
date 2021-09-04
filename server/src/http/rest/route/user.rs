use crate::http::http_handler::{to_response, HttpError};
use crate::lib::cursor_connection::PaginationArguments;
use crate::repository::repository::find_repositories_by_login;
use crate::repository::user::{
  find_followers_by_login, find_following_by_login, find_organizations_by_login, find_starred_repositories_by_login,
  find_user_by_login,
};
use actix_web::{web, HttpResponse, Responder, Scope};
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

  to_response(result, "User")
}

async fn organizations(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  // TODO convert this common validation into a actix_web::middleware
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    let result_error = HttpError::new("Invalid pagination arguments".to_string()).status(400);
    return HttpResponse::BadRequest().json(result_error);
  }

  let result = find_organizations_by_login(db.as_ref(), &login, pagination_arguments).await;

  to_response(result, "Organizations")
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

  to_response(result, "Repositories")
}

async fn starred_repositories(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    let result_error = HttpError::new("Invalid pagination arguments".to_string()).status(400);
    return HttpResponse::BadRequest().json(result_error);
  }

  let result = find_starred_repositories_by_login(db.as_ref(), &login, pagination_arguments).await;

  to_response(result, "Starred repositories")
}

async fn followers(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    let result_error = HttpError::new("Invalid pagination arguments".to_string()).status(400);
    return HttpResponse::BadRequest().json(result_error);
  }

  let result = find_followers_by_login(db.as_ref(), &login, pagination_arguments).await;

  to_response(result, "Followers")
}

async fn following(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  if PaginationArguments::is_valid(&pagination_arguments) == false {
    let result_error = HttpError::new("Invalid pagination arguments".to_string()).status(400);
    return HttpResponse::BadRequest().json(result_error);
  }

  let result = find_following_by_login(db.as_ref(), &login, pagination_arguments).await;

  to_response(result, "Following")
}
