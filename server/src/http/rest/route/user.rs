use crate::http::http_handler::to_response;
use crate::http::rest::{middleware, AppState};
use crate::lib::cursor_connection::PaginationArguments;
use crate::model::{
  repository::find_repositories_by_login,
  user::{
    find_followers_by_login, find_following_by_login, find_organizations_by_login, find_starred_repositories_by_login,
    find_user_by_login,
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

  to_response(result, "User")
}

async fn organizations(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_organizations_by_login(&state.db, &login, pagination_arguments).await;

  to_response(result, "Organizations")
}

async fn repositories(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_repositories_by_login(&state.db, &login, pagination_arguments).await;

  to_response(result, "Repositories")
}

async fn starred_repositories(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_starred_repositories_by_login(&state.db, &login, pagination_arguments).await;

  to_response(result, "Starred repositories")
}

async fn followers(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_followers_by_login(&state.db, &login, pagination_arguments).await;

  to_response(result, "Followers")
}

async fn following(
  state: web::Data<AppState>,
  web::Path(login): web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let result = find_following_by_login(&state.db, &login, pagination_arguments).await;

  to_response(result, "Following")
}
