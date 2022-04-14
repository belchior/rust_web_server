use crate::{
  http::{
    http_handler::{into_response_list, into_response_object},
    rest::{middleware, AppState},
  },
  lib::cursor_connection::PaginationArguments,
  model::{repository, user},
};
use actix_web::{web, Responder, Scope};

pub fn scope() -> Scope {
  web::scope("/user/{login}")
    .route("", web::get().to(user))
    .service(
      web::resource("/organizations")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(organizations)),
    )
    .service(
      web::resource("/repositories")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(repositories)),
    )
    .service(
      web::resource("/starred-repositories")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(starred_repositories)),
    )
    .service(
      web::resource("/followers")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(followers)),
    )
    .service(
      web::resource("/following")
        .wrap(middleware::ValidatePaginationArguments)
        .route(web::get().to(following)),
    )
}

async fn user(state: web::Data<AppState>, login: web::Path<String>) -> impl Responder {
  let db_client = state.poll.get().await.unwrap();
  let result = user::find_user_by_login(&db_client, &login).await;

  into_response_object(result, "User")
}

async fn organizations(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db_client = state.poll.get().await.unwrap();
  let result = user::find_user_by_login(&db_client, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(_)) => {
      let result = user::find_organizations_by_user_login(&db_client, &login, pagination_arguments).await;
      let result = user::users_organizations_to_cursor_connection(&db_client, &login, result).await;
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
  let result = user::find_user_by_login(&db_client, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(owner)) => {
      let result = repository::find_repositories_by_owner_login(&db_client, &owner.login, pagination_arguments).await;
      let result = repository::repositories_to_cursor_connection(&db_client, &owner.login, result).await;
      into_response_list(result)
    }
  }
}

async fn starred_repositories(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db_client = state.poll.get().await.unwrap();
  let result = user::find_user_by_login(&db_client, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(owner)) => {
      let result = user::find_starred_repositories_by_user_login(&db_client, &login, pagination_arguments).await;
      let result = repository::repositories_to_cursor_connection(&db_client, &owner.login, result).await;
      into_response_list(result)
    }
  }
}

async fn followers(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db_client = state.poll.get().await.unwrap();
  let result = user::find_user_by_login(&db_client, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(_)) => {
      let result = user::find_followers_by_login(&db_client, &login, pagination_arguments).await;
      let result = user::followers_to_cursor_connection(&db_client, &login, result).await;
      into_response_list(result)
    }
  }
}

async fn following(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db_client = state.poll.get().await.unwrap();
  let result = user::find_user_by_login(&db_client, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(_)) => {
      let result = user::find_followed_by_login(&db_client, &login, pagination_arguments).await;
      let result = user::followed_to_cursor_connection(&db_client, &login, result).await;
      into_response_list(result)
    }
  }
}
