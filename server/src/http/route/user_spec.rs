use crate::http::{http_handler::HttpError, route::user};
use crate::lib::cursor_connection::CursorConnection;
use crate::model::{organization::Organization, repository::Repository, user::User};
use crate::setup::mock::{make_request, HttpMethod};
use actix_web::{http::StatusCode, test};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_match_a_specified_user() {
  let res = make_request(HttpMethod::Get, "/user/user_foo", user::scope()).await;
  let status = res.status();
  let body: User = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.login, "user_foo");
}

// Organizations

#[actix_rt::test]
async fn should_find_organizations_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/user_foo/organizations", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.login, "organization_acme");
}

#[actix_rt::test]
async fn should_not_find_organizations_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/empty_user/organizations", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<Organization> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_organizations_of_a_unknown_user() {
  let res = make_request(HttpMethod::Get, "/user/user_xxx/organizations", user::scope()).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Repositories

#[actix_rt::test]
async fn should_find_repositories_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/user_bar/repositories", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges[0].node.name, "repository_bar");
}

#[actix_rt::test]
async fn should_not_find_repositories_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/empty_user/repositories", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_repositories_of_a_unknown_user() {
  let res = make_request(HttpMethod::Get, "/user/user_xxx/repositories", user::scope()).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Starred Repositories

#[actix_rt::test]
async fn should_find_starred_repositories_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/user_bar/starred-repositories", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 2);
  assert_eq!(body.edges[0].node.name, "repository_tux");
}

#[actix_rt::test]
async fn should_not_find_starred_repositories_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/user_dee/starred-repositories", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<Repository> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_starred_repositories_of_a_unknown_user() {
  let res = make_request(HttpMethod::Get, "/user/user_xxx/starred-repositories", user::scope()).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Followers

#[actix_rt::test]
async fn should_find_followers_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/user_bar/followers", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 1);
  assert_eq!(body.edges[0].node.login, "user_dee");
}

#[actix_rt::test]
async fn should_not_find_followers_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/empty_user/followers", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_followers_of_a_unknown_user() {
  let res = make_request(HttpMethod::Get, "/user/user_xxx/followers", user::scope()).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}

// Following

#[actix_rt::test]
async fn should_find_following_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/user_bar/following", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 1);
  assert_eq!(body.edges[0].node.login, "user_foo");
}

#[actix_rt::test]
async fn should_not_find_following_of_the_user() {
  let res = make_request(HttpMethod::Get, "/user/user_foo/following", user::scope()).await;
  let status = res.status();
  let body: CursorConnection<User> = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::OK);
  assert_eq!(body.edges.len(), 0);
}

#[actix_rt::test]
async fn should_not_find_following_of_a_unknown_user() {
  let res = make_request(HttpMethod::Get, "/user/user_xxx/following", user::scope()).await;
  let status = res.status();
  let body: HttpError = test::read_body_json(res).await;

  assert_eq!(status, StatusCode::NOT_FOUND);
  assert_eq!(body.message, "User not found");
}
