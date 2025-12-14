use crate::{
  application,
  infrastructure::{
    database::cursor_connection::PaginationArguments,
    http_server::{AppState, middleware, utils::into_response},
  },
};
use actix_web::{Responder, Scope, web};

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
  let db = state.poll.get().await.unwrap();
  let result = application::user::find_user(&db, &login).await;

  into_response(result, "User")
}

async fn organizations(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db = state.poll.get().await.unwrap();
  let result = application::user::find_organizations(&db, &login, pagination_arguments).await;

  into_response(result, "User")
}

async fn repositories(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db = state.poll.get().await.unwrap();
  let result = application::user::find_repositories(&db, &login, pagination_arguments).await;

  into_response(result, "User")
}

async fn starred_repositories(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db = state.poll.get().await.unwrap();
  let result = application::user::find_starred_repositories(&db, &login, pagination_arguments).await;

  into_response(result, "User")
}

async fn followers(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db = state.poll.get().await.unwrap();
  let result = application::user::find_followers(&db, &login, pagination_arguments).await;

  into_response(result, "User")
}

async fn following(
  state: web::Data<AppState>,
  login: web::Path<String>,
  web::Query(pagination_arguments): web::Query<PaginationArguments>,
) -> impl Responder {
  let db = state.poll.get().await.unwrap();
  let result = application::user::find_following(&db, &login, pagination_arguments).await;

  into_response(result, "User")
}
