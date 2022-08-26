use crate::http::{
  http_handler::{into_response_list, into_response_object},
  {middleware, AppState},
};
use crate::lib::cursor_connection::PaginationArguments;
use crate::model::{
  organization::{find_organization_by_login, find_people_by_login},
  repository::{find_repositories_by_owner_id, repositories_to_cursor_connection},
  user::users_to_cursor_connection,
};
use actix_web::{web, Responder, Scope};

pub fn scope() -> Scope {
  web::scope("/organization/{login}")
    .route("", web::get().to(organization))
    .service(
      web::resource("/people")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(people)),
    )
    .service(
      web::resource("/repositories")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(repositories)),
    )
}

async fn organization(state: web::Data<AppState>, login: web::Path<String>) -> impl Responder {
  let result = find_organization_by_login(&state.db, &login).await;

  into_response_object(result, "Organization")
}

async fn people(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_organization_by_login(&state.db, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "Organization"),
    Ok(Some(_)) => {
      let result = find_people_by_login(&state.db, &login, pagination_arguments).await;
      let result = users_to_cursor_connection(&state.db, &login, result).await;
      into_response_list(result)
    }
  }
}

async fn repositories(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_organization_by_login(&state.db, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "Organization"),
    Ok(Some(owner)) => {
      let result = find_repositories_by_owner_id(&state.db, &owner._id, pagination_arguments).await;
      let result = repositories_to_cursor_connection(&state.db, &owner._id, result).await;
      into_response_list(result)
    }
  }
}
