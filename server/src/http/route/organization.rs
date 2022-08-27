use crate::http::{
  http_handler::{into_response_list, into_response_object},
  {middleware, AppState},
};
use crate::lib::cursor_connection::PaginationArguments;
use crate::model::{organization, repository};
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
  let db = state.poll.get().await.unwrap();
  let result = organization::find_organization_by_login(&db, &login).await;

  into_response_object(result, "Organization")
}

async fn people(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db_client = state.poll.get().await.unwrap();
  let result = organization::find_organization_by_login(&db_client, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "Organization"),
    Ok(Some(_)) => {
      let result = organization::find_people_by_login(&db_client, &login, pagination_arguments).await;
      let result = organization::organizations_users_to_cursor_connection(&db_client, &login, result).await;
      into_response_list(result)
    }
  }
}

async fn repositories(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db_client = state.poll.get().await.unwrap();
  let result = organization::find_organization_by_login(&db_client, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "Organization"),
    Ok(Some(owner)) => {
      let result = repository::find_repositories_by_owner_login(&db_client, &owner.login, pagination_arguments).await;
      let result = repository::repositories_to_cursor_connection(&db_client, &owner.login, result).await;
      into_response_list(result)
    }
  }
}
