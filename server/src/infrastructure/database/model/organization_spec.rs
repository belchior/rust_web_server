use crate::infrastructure::{
  database::{self, cursor_connection::PaginationArguments},
  mock,
};
use pretty_assertions::assert_eq;

#[tokio::test]
async fn should_find_un_existing_organization() {
  let sufix = mock::random_sufix();
  let pool = mock::setup(&sufix).await;
  let db = pool.get().await.unwrap();
  let login = format!("organization_foo_{sufix}");
  let organization = database::organization::find_organization_by_login(&db, &login)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(organization.login, login);
}

#[tokio::test]
async fn should_not_panic_when_organization_is_not_found() {
  let sufix = mock::random_sufix();
  let pool = mock::setup(&sufix).await;
  let db = pool.get().await.unwrap();
  let login = format!("organization_xxx_{sufix}");
  let organization = database::organization::find_organization_by_login(&db, &login)
    .await
    .unwrap();

  assert_eq!(organization, None);
}

#[tokio::test]
async fn should_find_organizations_people() {
  let sufix = mock::random_sufix();
  let pool = mock::setup(&sufix).await;
  let db = pool.get().await.unwrap();
  let login = format!("organization_acme_{sufix}");
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = database::organization::find_people_by_login(&db, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_foo_{sufix}"));
}

#[tokio::test]
async fn should_dont_panic_when_person_is_not_found() {
  let sufix = mock::random_sufix();
  let pool = mock::setup(&sufix).await;
  let db = pool.get().await.unwrap();
  let login = format!("organization_empty_{sufix}");
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = database::organization::find_people_by_login(&db, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

#[tokio::test]
async fn should_convert_a_organization_list_into_cursor_connection_of_organizations() {
  let sufix = mock::random_sufix();
  let pool = mock::setup(&sufix).await;
  let db = pool.get().await.unwrap();
  let organization_login = format!("organization_acme_{sufix}");
  let organization = database::organization::find_organization_by_login(&db, &organization_login)
    .await
    .unwrap()
    .unwrap();

  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let user_list = database::organization::find_people_by_login(&db, &organization.login, pagination_argument)
    .await
    .unwrap();
  let first_user_login = user_list[0].login.clone();
  let cursor_connection =
    database::organization::organizations_users_to_cursor_connection(&db, &organization_login, Ok(user_list))
      .await
      .unwrap();

  assert_eq!(cursor_connection.edges.len(), 1);
  assert_eq!(cursor_connection.edges[0].node.login, first_user_login);
  assert_eq!(cursor_connection.page_info.has_previous_page, false);
  assert_eq!(cursor_connection.page_info.has_next_page, true);
}

/// Paginating People

#[tokio::test]
async fn should_paginating_people_from_start_to_end() {
  let sufix = mock::random_sufix();
  let pool = mock::setup(&sufix).await;
  let db = pool.get().await.unwrap();
  let login = format!("organization_acme_{sufix}");

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = database::organization::find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_foo_{sufix}"));

  let end_cursor = Some(base64::encode(users[0].id.to_string()));

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = database::organization::find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_dee_{sufix}"));

  let end_cursor = Some(base64::encode(users[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = database::organization::find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

#[tokio::test]
async fn should_paginating_people_from_end_to_start() {
  let sufix = mock::random_sufix();
  let pool = mock::setup(&sufix).await;
  let db = pool.get().await.unwrap();
  let login = format!("organization_acme_{sufix}");

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = database::organization::find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_dee_{sufix}"));

  let start_cursor = Some(base64::encode(users[0].id.to_string()));

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = database::organization::find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_foo_{sufix}"));

  let start_cursor = Some(base64::encode(users[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = database::organization::find_people_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}
