use crate::http::{http_handler::HttpError, route::user, AppState};
use crate::lib::cursor_connection::CursorConnection;
use crate::model::{organization::Organization, repository::Repository, user::User};
use crate::setup::mock;
use actix_web::{http::StatusCode, test, web, App};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_a_specified_user() {
  // TODO find a way to reuse this block of code without start the type dependencies war
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/user_foo").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: User = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, "user_foo");
}

// Organizations

#[actix_rt::test]
async fn should_find_organizations_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/user/user_foo/organizations")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.login, "organization_acme");
}

#[actix_rt::test]
async fn should_not_find_organizations_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/user/empty_user/organizations")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_organizations_of_a_unknown_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/user/user_xxx/organizations")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Repositories

#[actix_rt::test]
async fn should_find_repositories_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/user_bar/repositories").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.name, "repository_bar");
}

#[actix_rt::test]
async fn should_not_find_repositories_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/user/empty_user/repositories")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_repositories_of_a_unknown_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/user_xxx/repositories").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Starred Repositories

#[actix_rt::test]
async fn should_find_starred_repositories_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/user/user_bar/starred-repositories")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 2);
  assert_eq!(body.edges[0].node.name, "repository_tux");
}

#[actix_rt::test]
async fn should_not_find_starred_repositories_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/user/user_dee/starred-repositories")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_starred_repositories_of_a_unknown_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get()
    .uri("/user/user_xxx/starred-repositories")
    .to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Followers

#[actix_rt::test]
async fn should_find_followers_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/user_bar/followers").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 1);
  assert_eq!(body.edges[0].node.login, "user_dee");
}

#[actix_rt::test]
async fn should_not_find_followers_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/empty_user/followers").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_followers_of_a_unknown_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/user_xxx/followers").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Following

#[actix_rt::test]
async fn should_find_following_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/user_bar/following").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 1);
  assert_eq!(body.edges[0].node.login, "user_foo");
}

#[actix_rt::test]
async fn should_not_find_following_of_the_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/user_foo/following").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_following_of_a_unknown_user() {
  let poll = mock::setup().await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { poll }))
      .service(user::scope()),
  )
  .await;
  let req = test::TestRequest::get().uri("/user/user_xxx/following").to_request();
  let res = test::call_service(&app, req).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}
