use crate::{
  lib::cursor_connection::PaginationArguments,
  mock,
  model::{organization, repository::*, user},
};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_users_repositories() {
  let db = mock::setup().await;
  let login = "user_bar".to_owned();
  let user = user::find_user_by_login(&db, &login).await.unwrap().unwrap();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_id(&db, &user._id, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_bar");
}

#[actix_rt::test]
async fn should_dont_panic_when_repository_is_not_found() {
  let db = mock::setup().await;
  let login = "empty_user".to_owned();
  let user = user::find_user_by_login(&db, &login).await.unwrap().unwrap();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_id(&db, &user._id, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_convert_a_repository_list_into_cursor_connection_of_repositories() {
  let db = mock::setup().await;
  let owner_login = "organization_acme".to_owned();
  let organization = organization::find_organization_by_login(&db, &owner_login)
    .await
    .unwrap()
    .unwrap();

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_id(&db, &organization._id, pagination_arguments)
    .await
    .unwrap();

  let cursor_connection = repositories_to_cursor_connection(&db, &organization._id, Ok(repositories))
    .await
    .unwrap();

  assert_eq!(cursor_connection.edges.len(), 1);
  assert_eq!(cursor_connection.edges[0].node.name, "repository_tux");
  assert_eq!(cursor_connection.page_info.has_previous_page, false);
  assert_eq!(cursor_connection.page_info.has_next_page, true);
}

/// Paginating Repositories

#[actix_rt::test]
async fn should_paginating_repositories_from_start_to_end() {
  let db = mock::setup().await;
  let login = "organization_acme".to_owned();
  let owner = organization::find_organization_by_login(&db, &login).await;
  let owner = owner.unwrap().unwrap();

  // should find the first repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_id(&db, &owner._id, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_tux");

  let end_cursor = Some(base64::encode(repositories[0]._id.to_hex()));

  // should find the last repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_id(&db, &owner._id, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_mar");

  let end_cursor = Some(base64::encode(repositories[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_id(&db, &owner._id, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_repositories_from_end_to_start() {
  let db = mock::setup().await;
  let login = "organization_acme".to_owned();
  let owner = organization::find_organization_by_login(&db, &login).await;
  let owner = owner.unwrap().unwrap();

  // should find the last repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let repositories = find_repositories_by_owner_id(&db, &owner._id, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_mar");

  let start_cursor = Some(base64::encode(repositories[0]._id.to_hex()));

  // should find the first repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_repositories_by_owner_id(&db, &owner._id, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_tux");

  let start_cursor = Some(base64::encode(repositories[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_repositories_by_owner_id(&db, &owner._id, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}
