use crate::{
  http::{http_handler::HttpError, rest::AppState},
  model::{organization::find_organization_by_login, user::find_user_by_login},
};
use actix_web::{web, HttpResponse, Responder, Scope};
use futures::join;
use log;

pub fn scope() -> Scope {
  web::scope("/profile/{login}").route("", web::get().to(profile))
}

async fn profile(path: web::Path<String>, state: web::Data<AppState>) -> impl Responder {
  let login = path.into_inner();
  let db_client = state.poll.get().await.unwrap();
  let (user, organization) = join!(
    find_user_by_login(&db_client, &login),
    find_organization_by_login(&db_client, &login)
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
