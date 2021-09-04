use crate::http::http_handler::HttpError;
use crate::repository::organization::find_organization_by_login;
use crate::repository::user::find_user_by_login;
use actix_web::{web, HttpResponse, Responder, Scope};
use futures::join;
use log;
use mongodb::Database;

pub fn scope() -> Scope {
  web::scope("/profile/{login}").route("", web::get().to(profile))
}

async fn profile(db: web::Data<Database>, web::Path(login): web::Path<String>) -> impl Responder {
  let (user, organization) = join!(
    find_user_by_login(db.as_ref(), &login),
    find_organization_by_login(db.as_ref(), &login)
  );

  match (user, organization) {
    (Err(err), _) | (_, Err(err)) => {
      log::error!("Database error: {:#?}", err);
      let result_error = HttpError::new("Internal Server Error".to_string()).status(500);
      HttpResponse::BadRequest().json(result_error)
    }
    (Ok(Some(user)), _) => HttpResponse::Ok().json(user),
    (_, Ok(Some(organization))) => HttpResponse::Ok().json(organization),
    _ => {
      log::info!("Profile {} not found", login);
      let result_error = HttpError::new("Profile not found".to_string()).status(404);
      HttpResponse::BadRequest().json(result_error)
    }
  }
}
