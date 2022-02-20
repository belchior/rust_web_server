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

  let users = find_people_by_login(&db, &login, pagination_argument).await.unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_foo");
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

  let users = find_people_by_login(&db, &login, pagination_arguments).await.unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_foo");

  let end_cursor = Some(base64::encode(users[0]._id.to_hex()));

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments).await.unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_dee");

  let end_cursor = Some(base64::encode(users[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments).await.unwrap();

  assert_eq!(users.len(), 0);
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

  let users = find_people_by_login(&db, &login, pagination_arguments).await.unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_dee");

  let start_cursor = Some(base64::encode(users[0]._id.to_hex()));

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments).await.unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_foo");

  let start_cursor = Some(base64::encode(users[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments).await.unwrap();

  assert_eq!(users.len(), 0);
}
