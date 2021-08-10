use crate::http::http_handler::HttpError;
use crate::repository::user::find_user_by_login;
use actix_web::{get, web, HttpResponse, Responder};
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

  if let Err(err) = result {
    println!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError {
      status: 500,
      message: "Internal Server Error".to_owned(),
    });
  }

  let maybe_document = result.unwrap();
  if let None = maybe_document {
    println!("User {} not found", login);
    return HttpResponse::NotFound().json(HttpError {
      status: 400,
      message: "User not found".to_owned(),
    });
  }

  HttpResponse::Ok().json(maybe_document)
}
