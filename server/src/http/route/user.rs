use crate::http::{
  http_handler::{into_response_list, into_response_object},
  {middleware, AppState},
};
use crate::lib::cursor_connection::PaginationArguments;
use crate::model::{
  organization::organizations_to_cursor_connection,
  repository::{find_repositories_by_owner_id, repositories_to_cursor_connection},
  user::{
    find_followers_by_login, find_following_by_login, find_organizations_by_login, find_starred_repositories_by_login,
    find_user_by_login, users_to_cursor_connection,
  },
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

async fn user(state: web::Data<AppState>, web::Path(login): web::Path<String>) -> impl Responder {
  let result = find_user_by_login(&state.db, &login).await;

  into_response_object(result, "User")
}

async fn organizations(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_user_by_login(&state.db, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(_)) => {
      let result = find_organizations_by_login(&state.db, &login, pagination_arguments).await;
      let result = organizations_to_cursor_connection(&state.db, &login, result).await;
      into_response_list(result)
    }
  }
}

async fn repositories(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_user_by_login(&state.db, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(owner)) => {
      let result = find_repositories_by_owner_id(&state.db, &owner._id, pagination_arguments).await;
      let result = repositories_to_cursor_connection(&state.db, &owner._id, result).await;
      into_response_list(result)
    }
  }
}

async fn starred_repositories(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_user_by_login(&state.db, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(owner)) => {
      let result = find_starred_repositories_by_login(&state.db, &login, pagination_arguments).await;
      let result = repositories_to_cursor_connection(&state.db, &owner._id, result).await;
      into_response_list(result)
    }
  }
}

async fn followers(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_user_by_login(&state.db, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(_)) => {
      let result = find_followers_by_login(&state.db, &login, pagination_arguments).await;
      let result = users_to_cursor_connection(&state.db, &login, result).await;
      into_response_list(result)
    }
  }
}

async fn following(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_user_by_login(&state.db, &login).await;

  match result {
    Err(_) => into_response_object(result, ""),
    Ok(None) => into_response_object(result, "User"),
    Ok(Some(_)) => {
      let result = find_following_by_login(&state.db, &login, pagination_arguments).await;
      let result = users_to_cursor_connection(&state.db, &login, result).await;
      into_response_list(result)
    }
  }
}
