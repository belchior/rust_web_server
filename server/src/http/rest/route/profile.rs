use crate::http::http_handler::HttpError;
use crate::repository::organization::find_organization_by_login;
use crate::repository::user::find_user_by_login;
use actix_web::{get, web, HttpResponse, Responder};
use futures::join;
use log;
use mongodb::Database;

#[get("/profile/{login}")]
pub async fn profile(db: web::Data<Database>, web::Path(login): web::Path<String>) -> impl Responder {
  let (user, organization) = join!(
    find_user_by_login(db.as_ref(), &login),
    find_organization_by_login(db.as_ref(), &login)
  );

  match (user, organization) {
    (Err(err), _) | (_, Err(err)) => {
      log::error!("Database error: {:#?}", err);
      HttpResponse::InternalServerError().json(HttpError::new("Internal Server Error", None))
    }
    (Ok(Some(user)), _) => HttpResponse::Ok().json(user),
    (_, Ok(Some(organization))) => HttpResponse::Ok().json(organization),
    _ => {
      log::info!("Profile {} not found", login);
      HttpResponse::NotFound().json(HttpError::new("Profile not found", Some(404)))
    }
  }
}
