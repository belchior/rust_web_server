use crate::model::user::User;
use crate::repository::user::find_user_by_login;
use crate::route::http_handler::HttpError;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::Database;
use tokio_stream::StreamExt;

#[get("/user/{login}")]
pub async fn user(db: web::Data<Database>, web::Path(login): web::Path<String>) -> impl Responder {
  let result = find_user_by_login(db.as_ref(), &login).await;

  if let Err(err) = result {
    println!("Database error: {}", err.to_string());
    return HttpResponse::InternalServerError().json(HttpError {
      status: 500,
      message: "Internal Server Error".to_owned(),
    });
  }

  let maybe_document = result.unwrap().next().await;
  if let None = maybe_document {
    println!("User {} not found", login);
    return HttpResponse::NotFound().json(HttpError {
      status: 400,
      message: "User not found".to_owned(),
    });
  }

  let maybe_document = maybe_document.unwrap().unwrap();
  let maybe_document = User::deserialize(maybe_document);
  if let Err(err) = maybe_document {
    println!("Serialization error: {}", err.to_string());
    return HttpResponse::InternalServerError().json(HttpError {
      status: 500,
      message: "Internal Server Error".to_owned(),
    });
  }

  HttpResponse::Ok().json(maybe_document.unwrap())
}
