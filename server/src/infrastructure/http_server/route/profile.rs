use crate::{
  application::{self, profile::Profile},
  infrastructure::http_server::utils::HttpError,
};
use actix_web::{HttpResponse, Responder, Scope, web};
use tracing;

pub fn scope() -> Scope {
  web::scope("/profile/{login}").route("", web::get().to(profile))
}

async fn profile(login: web::Path<String>) -> impl Responder {
  let profile = application::profile::find_profile(&login).await;

  match profile {
    Ok(Some(Profile::User(user))) => {
      tracing::info!("User profile {login} was found");
      HttpResponse::Ok().json(user)
    }
    Ok(Some(Profile::Organization(org))) => {
      tracing::info!("Organization profile {login} was found");
      HttpResponse::Ok().json(org)
    }
    Err(err) => {
      tracing::error!("Internal server error: {:#?}", err);
      let result_error = HttpError::new("Internal server error".to_string());
      HttpResponse::InternalServerError().json(result_error)
    }
    _ => {
      tracing::info!("Profile {login} not found");
      let result_error = HttpError::new("Profile not found".to_string());
      HttpResponse::NotFound().json(result_error)
    }
  }
}
