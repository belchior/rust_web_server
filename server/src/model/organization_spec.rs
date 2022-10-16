use crate::lib::cursor_connection::PaginationArguments;
use crate::model::organization::*;
use crate::setup::mock;
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_un_existing_organization() {
  let sufix = mock::random_sufix();
  let login = format!("organization_foo_{sufix}");
  let db = mock::setup(&sufix).await;
  let organization = find_organization_by_login(&db, &login).await.unwrap().unwrap();

  assert_eq!(organization.login, format!("organization_foo_{sufix}"));
}

#[actix_rt::test]
async fn should_not_panic_when_organization_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("organization_xxx_{sufix}");
  let db = mock::setup(&sufix).await;
  let organization = find_organization_by_login(&db, &login).await.unwrap();

  assert_eq!(organization, None);
}

#[actix_rt::test]
async fn should_find_organizations_people() {
  let sufix = mock::random_sufix();
  let login = format!("organization_acme_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_argument).await.unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_foo_{sufix}"));
}

#[actix_rt::test]
async fn should_dont_panic_when_person_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("organization_empty_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_argument).await.unwrap();

  assert_eq!(users.len(), 0);
}

#[actix_rt::test]
async fn should_convert_a_organization_list_into_cursor_connection_of_organizations() {
  let sufix = mock::random_sufix();
  let user_login = format!("user_foo_{sufix}");
  let organization_login = format!("organization_foo_{sufix}");
  let db = mock::setup(&sufix).await;
  let organization = find_organization_by_login(&db, &organization_login)
    .await
    .unwrap()
    .unwrap();
  let organization_list = Ok(vec![organization]);
  let cursor_connection = organizations_to_cursor_connection(&db, &user_login, organization_list)
    .await
    .unwrap();

  assert_eq!(cursor_connection.edges.len(), 1);
  assert_eq!(
    cursor_connection.edges[0].node.login,
    format!("organization_foo_{sufix}")
  );
  assert_eq!(cursor_connection.page_info.has_previous_page, false);
  assert_eq!(cursor_connection.page_info.has_next_page, true);
}

/// Paginating People

#[actix_rt::test]
async fn should_paginating_people_from_start_to_end() {
  let sufix = mock::random_sufix();
  let login = format!("organization_acme_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments).await.unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_foo_{sufix}"));

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
  assert_eq!(users[0].login, format!("user_dee_{sufix}"));

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
  let sufix = mock::random_sufix();
  let login = format!("organization_acme_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_people_by_login(&db, &login, pagination_arguments).await.unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_dee_{sufix}"));

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
  assert_eq!(users[0].login, format!("user_foo_{sufix}"));

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
