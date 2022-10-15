use crate::lib::cursor_connection::PaginationArguments;
use crate::model::organization::*;
use crate::setup::mock;
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_un_existing_organization() {
  let sufix = mock::random_sufix();
  let login = format!("organization_foo_{sufix}");
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let organization = find_organization_by_login(&db_client, &login).await.unwrap().unwrap();

  assert_eq!(organization.login, login);
}

#[actix_rt::test]
async fn should_not_panic_when_organization_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("organization_xxx_{sufix}");
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let organization = find_organization_by_login(&db_client, &login).await.unwrap();

  assert_eq!(organization, None);
}

#[actix_rt::test]
async fn should_find_organizations_people() {
  let sufix = mock::random_sufix();
  let login = format!("organization_acme_{sufix}");
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_foo_{sufix}"));
}

#[actix_rt::test]
async fn should_dont_panic_when_person_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("organization_empty_{sufix}");
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

#[actix_rt::test]
async fn should_convert_a_organization_list_into_cursor_connection_of_organizations() {
  let sufix = mock::random_sufix();
  let organization_login = format!("organization_acme_{sufix}");
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();
  let organization = find_organization_by_login(&db_client, &organization_login)
    .await
    .unwrap()
    .unwrap();

  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let user_list = find_people_by_login(&db_client, &organization.login, pagination_argument)
    .await
    .unwrap();
  let first_user_login = user_list[0].login.clone();
  let cursor_connection = organizations_users_to_cursor_connection(&db_client, &organization_login, Ok(user_list))
    .await
    .unwrap();

  assert_eq!(cursor_connection.edges.len(), 1);
  assert_eq!(cursor_connection.edges[0].node.login, first_user_login);
  assert_eq!(cursor_connection.page_info.has_previous_page, false);
  assert_eq!(cursor_connection.page_info.has_next_page, true);
}

/// Paginating People

#[actix_rt::test]
async fn should_paginating_people_from_start_to_end() {
  let sufix = mock::random_sufix();
  let login = format!("organization_acme_{sufix}");
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_people_by_login(&db_client, &login, pagination_arguments)
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

  let users = find_people_by_login(&db_client, &login, pagination_arguments)
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

  let users = find_people_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_people_from_end_to_start() {
  let sufix = mock::random_sufix();
  let login = format!("organization_acme_{sufix}");
  let poll = mock::setup(&sufix).await;
  let db_client = poll.get().await.unwrap();

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_people_by_login(&db_client, &login, pagination_arguments)
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

  let users = find_people_by_login(&db_client, &login, pagination_arguments)
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

  let users = find_people_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}
