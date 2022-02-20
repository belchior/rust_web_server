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
