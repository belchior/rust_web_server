use crate::{lib::cursor_connection::PaginationArguments, mock, model::organization::*};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_un_existing_organization() {
  let db = mock::setup().await;
  let login = "organization_foo".to_owned();
  let organization = find_organization_by_login(&db, &login).await.unwrap().unwrap();

  assert_eq!(organization.login, "organization_foo");
}

#[actix_rt::test]
async fn should_find_organizations_people() {
  let db = mock::setup().await;
  let login = "organization_acme".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_argument)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_foo");
}

#[actix_rt::test]
async fn should_find_organizations_repositories() {
  let db = mock::setup().await;
  let login = "organization_acme".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_login(&db, &login, pagination_argument)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 1);
  assert_eq!(repositories.edges[0].node.name, "repository_tux");
}

/// Paginating People

#[actix_rt::test]
async fn should_paginating_people_from_start_to_end() {
  let db = mock::setup().await;
  let login = "organization_acme".to_owned();

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_foo");

  let end_cursor = Some(base64::encode(users.edges[0].node._id.to_hex()));

  assert_eq!(users.page_info.has_next_page, true);
  assert_eq!(users.page_info.end_cursor, end_cursor);

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_dee");

  let end_cursor = Some(base64::encode(users.edges[0].node._id.to_hex()));

  assert_eq!(users.page_info.has_next_page, false);
  assert_eq!(users.page_info.end_cursor, end_cursor);

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 0);
  assert_eq!(users.page_info.has_next_page, false);
}

#[actix_rt::test]
async fn should_paginating_people_from_end_to_start() {
  let db = mock::setup().await;
  let login = "organization_acme".to_owned();

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_dee");

  let start_cursor = Some(base64::encode(users.edges[0].node._id.to_hex()));

  assert_eq!(users.page_info.has_previous_page, true);
  assert_eq!(users.page_info.start_cursor, start_cursor);

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_foo");

  let start_cursor = Some(base64::encode(users.edges[0].node._id.to_hex()));

  assert_eq!(users.page_info.has_previous_page, false);
  assert_eq!(users.page_info.start_cursor, start_cursor);

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 0);
  assert_eq!(users.page_info.has_previous_page, false);
}
