use crate::cursor_connection::PaginationArguments;
use crate::repository::repository::find_repositories_by_login;
use crate::route::http_handler::HttpError;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::Database;

#[get("/repositories/{login}")]
pub async fn repositories(
  db: web::Data<Database>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_repositories_by_login(db.as_ref(), &login, pagination_arguments).await;

  if let Err(err) = result {
    println!("Database error: {:#?}", err);
    return HttpResponse::InternalServerError().json(HttpError {
      status: 500,
      message: "Internal Server Error".to_owned(),
    });
  }

  let maybe_documents = result.unwrap();
  if let None = maybe_documents {
    println!("User {} not found", login);
    return HttpResponse::NotFound().json(HttpError {
      status: 400,
      message: "User not found".to_owned(),
    });
  }

  HttpResponse::Ok().json(maybe_documents)
}
