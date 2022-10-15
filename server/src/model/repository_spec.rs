use crate::lib::cursor_connection::PaginationArguments;
use crate::model::repository::*;
use crate::setup::mock;
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_owners_repositories() {
  let sufix = mock::random_sufix();
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let owner_login = format!("organization_acme_{sufix}");
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_tux_{sufix}"));
}

#[actix_rt::test]
async fn should_dont_panic_when_repository_is_not_found() {
  let sufix = mock::random_sufix();
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let owner_login = format!("empty_user_{sufix}");
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_convert_a_repository_list_into_cursor_connection_of_repositories() {
  let sufix = mock::random_sufix();
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let owner_login = format!("organization_acme_{sufix}");

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_arguments)
    .await
    .unwrap();

  let first_repository_name = repositories[0].name.clone();

  let cursor_connection = repositories_to_cursor_connection(&db_client, &owner_login, Ok(repositories))
    .await
    .unwrap();

  assert_eq!(cursor_connection.edges.len(), 1);
  assert_eq!(cursor_connection.edges[0].node.name, first_repository_name);
  assert_eq!(cursor_connection.page_info.has_previous_page, true);
  assert_eq!(cursor_connection.page_info.has_next_page, false);
}

/// Paginating Repositories

#[actix_rt::test]
async fn should_paginating_repositories_from_start_to_end() {
  let sufix = mock::random_sufix();
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let owner_login = format!("organization_acme_{sufix}");

  // should find the first repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_tux_{sufix}"));

  let end_cursor = Some(base64::encode(repositories[0].id.to_string()));

  // should find the last repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_mar_{sufix}"));

  let end_cursor = Some(base64::encode(repositories[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_repositories_from_end_to_start() {
  let sufix = mock::random_sufix();
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let owner_login = format!("organization_acme_{sufix}");

  // should find the last repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_mar_{sufix}"));

  let start_cursor = Some(base64::encode(repositories[0].id.to_string()));

  // should find the first repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_tux_{sufix}"));

  let start_cursor = Some(base64::encode(repositories[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_repositories_by_owner_login(&db_client, &owner_login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}
