use crate::http::{http_handler::HttpError, route::user};
use crate::lib::cursor_connection::CursorConnection;
use crate::model::{organization::Organization, repository::Repository, user::User};
use crate::setup::mock;
use actix_web::{http::StatusCode, test};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_a_specified_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let res = mock::make_request(mock::HttpMethod::Get, &format!("/user/{login}"), user::scope(), &sufix).await;
  let status = res.status();
  let body: User = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, format!("user_foo_{sufix}"));
}

// Organizations

#[actix_rt::test]
async fn should_find_organizations_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/organizations"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.login, format!("organization_foo_{sufix}"));
}

#[actix_rt::test]
async fn should_not_find_organizations_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("empty_user_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/organizations"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_organizations_of_a_unknown_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_xxx_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/organizations"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Repositories

#[actix_rt::test]
async fn should_find_repositories_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_bar_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/repositories"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.name, format!("repository_bar_{sufix}"));
}

#[actix_rt::test]
async fn should_not_find_repositories_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("empty_user_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/repositories"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_repositories_of_a_unknown_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_xxx_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/repositories"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Starred Repositories

#[actix_rt::test]
async fn should_find_starred_repositories_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_bar_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/starred-repositories"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 2);
  assert_eq!(body.edges[0].node.name, format!("repository_tux_{sufix}"));
}

#[actix_rt::test]
async fn should_not_find_starred_repositories_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_dee_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/starred-repositories"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_starred_repositories_of_a_unknown_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_xxx_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/starred-repositories"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Followers

#[actix_rt::test]
async fn should_find_followers_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_bar_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/followers"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 1);
  assert_eq!(body.edges[0].node.login, format!("user_dee_{sufix}"));
}

#[actix_rt::test]
async fn should_not_find_followers_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("empty_user_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/followers"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_followers_of_a_unknown_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_xxx_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/followers"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Following

#[actix_rt::test]
async fn should_find_following_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_bar_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/following"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 1);
  assert_eq!(body.edges[0].node.login, format!("user_foo_{sufix}"));
}

#[actix_rt::test]
async fn should_not_find_following_of_the_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/following"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_following_of_a_unknown_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_xxx_{sufix}");
  let res = mock::make_request(
    mock::HttpMethod::Get,
    &format!("/user/{login}/following"),
    user::scope(),
    &sufix,
  )
  .await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}
