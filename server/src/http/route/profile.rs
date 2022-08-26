use crate::http::{http_handler::HttpError, AppState};
use crate::model::{organization::find_organization_by_login, user::find_user_by_login};
use actix_web::{web, HttpResponse, Responder, Scope};
use futures::join;
use log;

pub fn scope() -> Scope {
  web::scope("/profile/{login}").route("", web::get().to(profile))
}

async fn profile(state: web::Data<AppState>, login: web::Path<String>) -> impl Responder {
  let (user, organization) = join!(
    find_user_by_login(&state.db, &login),
    find_organization_by_login(&state.db, &login)
  );

  match (user, organization) {
    (Err(err), _) | (_, Err(err)) => {
      log::error!("Internal server error: {:#?}", err);
      let result_error = HttpError::new("Internal server error".to_string());
      HttpResponse::InternalServerError().json(result_error)
    }
    (Ok(Some(user)), _) => HttpResponse::Ok().json(user),
    (_, Ok(Some(organization))) => HttpResponse::Ok().json(organization),
    _ => {
      log::info!("Profile {} not found", login);
      let result_error = HttpError::new("Profile not found".to_string());
      HttpResponse::NotFound().json(result_error)
    }
  }
}
