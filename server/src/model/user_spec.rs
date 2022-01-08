use super::user::*;
use crate::{lib::cursor_connection::PaginationArguments, mock};
use base64;
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_an_existing_user() {
  let db = mock::setup().await;
  let login = "user_foo".to_owned();

  let user = find_user_by_login(&db, &login).await.unwrap().unwrap();

  assert_eq!(user.email, "foo@email.com".to_owned());
}

#[actix_rt::test]
async fn should_find_users_organizations() {
  let db = mock::setup().await;
  let login = "user_foo".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_argument)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(organizations.edges.len(), 1);
  assert_eq!(organizations.edges[0].node.login, "organization_foo");
}

#[actix_rt::test]
async fn should_find_users_starred_repositories() {
  let db = mock::setup().await;
  let login = "user_bar".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_argument)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 1);
  assert_eq!(repositories.edges[0].node.name, "repository_tux");
}

#[actix_rt::test]
async fn should_find_users_followers() {
  let db = mock::setup().await;
  let login = "user_foo".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(2),
    after: None,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db, &login, pagination_argument)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 2);
  assert_eq!(users.edges[0].node.login, "user_bar");
  assert_eq!(users.edges[1].node.login, "user_dee");
}

#[actix_rt::test]
async fn should_find_users_following() {
  let db = mock::setup().await;
  let login = "user_dee".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(2),
    after: None,
    last: None,
    before: None,
  };

  let users = find_following_by_login(&db, &login, pagination_argument)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 2);
  assert_eq!(users.edges[0].node.login, "user_foo");
  assert_eq!(users.edges[1].node.login, "user_bar");
}

/*
  Paginating Organizations
*/

#[actix_rt::test]
async fn should_paginating_organizations_from_start_to_end() {
  let db = mock::setup().await;
  let login = "user_foo".to_owned();

  // should find the first organization

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(organizations.edges.len(), 1);
  assert_eq!(organizations.edges[0].node.login, "organization_foo");

  let end_cursor = Some(base64::encode(organizations.edges[0].node._id.to_hex()));

  assert_eq!(organizations.page_info.has_next_page, true);
  assert_eq!(organizations.page_info.end_cursor, end_cursor);

  // should find the last organization

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(organizations.edges.len(), 1);
  assert_eq!(organizations.edges[0].node.login, "organization_acme");

  let end_cursor = Some(base64::encode(organizations.edges[0].node._id.to_hex()));

  assert_eq!(organizations.page_info.has_next_page, false);
  assert_eq!(organizations.page_info.end_cursor, end_cursor);

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(organizations.edges.len(), 0);
  assert_eq!(organizations.page_info.has_next_page, false);
}

#[actix_rt::test]
async fn should_paginating_organizations_from_end_to_start() {
  let db = mock::setup().await;
  let login = "user_foo".to_owned();

  // should find the last organization

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(organizations.edges.len(), 1);
  assert_eq!(organizations.edges[0].node.login, "organization_acme");

  let start_cursor = Some(base64::encode(organizations.edges[0].node._id.to_hex()));

  assert_eq!(organizations.page_info.has_previous_page, true);
  assert_eq!(organizations.page_info.start_cursor, start_cursor);

  // should find the first organization

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(organizations.edges.len(), 1);
  assert_eq!(organizations.edges[0].node.login, "organization_foo");

  let start_cursor = Some(base64::encode(organizations.edges[0].node._id.to_hex()));

  assert_eq!(organizations.page_info.has_previous_page, false);
  assert_eq!(organizations.page_info.start_cursor, start_cursor);

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(organizations.edges.len(), 0);
  assert_eq!(organizations.page_info.has_previous_page, false);
}

/*
  Paginating Starred Repositories
*/

#[actix_rt::test]
async fn should_paginating_starred_repositories_from_start_to_end() {
  let db = mock::setup().await;
  let login = "user_bar".to_owned();

  // should find the first starred repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 1);
  assert_eq!(repositories.edges[0].node.name, "repository_tux");

  let end_cursor = Some(base64::encode(repositories.edges[0].node._id.to_hex()));

  assert_eq!(repositories.page_info.has_next_page, true);
  assert_eq!(repositories.page_info.end_cursor, end_cursor);

  // should find the last starred repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 1);
  assert_eq!(repositories.edges[0].node.name, "repository_dee");

  let end_cursor = Some(base64::encode(repositories.edges[0].node._id.to_hex()));

  assert_eq!(repositories.page_info.has_next_page, false);
  assert_eq!(repositories.page_info.end_cursor, end_cursor);

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 0);
  assert_eq!(repositories.page_info.has_next_page, false);
}

#[actix_rt::test]
async fn should_paginating_starred_repositories_from_end_to_start() {
  let db = mock::setup().await;
  let login = "user_bar".to_owned();

  // should find the last starred repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 1);
  assert_eq!(repositories.edges[0].node.name, "repository_dee");

  let start_cursor = Some(base64::encode(repositories.edges[0].node._id.to_hex()));

  assert_eq!(repositories.page_info.has_previous_page, true);
  assert_eq!(repositories.page_info.start_cursor, start_cursor);

  // should find the first starred repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 1);
  assert_eq!(repositories.edges[0].node.name, "repository_tux");

  let start_cursor = Some(base64::encode(repositories.edges[0].node._id.to_hex()));

  assert_eq!(repositories.page_info.has_previous_page, false);
  assert_eq!(repositories.page_info.start_cursor, start_cursor);

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 0);
  assert_eq!(repositories.page_info.has_previous_page, false);
}

/*
  Paginating Followers
*/

#[actix_rt::test]
async fn should_paginating_followers_from_start_to_end() {
  let db = mock::setup().await;
  let login = "user_foo".to_owned();

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_bar");

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

  let users = find_followers_by_login(&db, &login, pagination_arguments)
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

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 0);
  assert_eq!(users.page_info.has_next_page, false);
}

#[actix_rt::test]
async fn should_paginating_followers_from_end_to_start() {
  let db = mock::setup().await;
  let login = "user_foo".to_owned();

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_followers_by_login(&db, &login, pagination_arguments)
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

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_bar");

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

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 0);
  assert_eq!(users.page_info.has_previous_page, false);
}

/*
  Paginating Following
*/

#[actix_rt::test]
async fn should_paginating_following_from_start_to_end() {
  let db = mock::setup().await;
  let login = "user_dee".to_owned();

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_following_by_login(&db, &login, pagination_arguments)
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

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_bar");

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

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 0);
  assert_eq!(users.page_info.has_next_page, false);
}

#[actix_rt::test]
async fn should_paginating_following_from_end_to_start() {
  let db = mock::setup().await;
  let login = "user_dee".to_owned();

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 1);
  assert_eq!(users.edges[0].node.login, "user_bar");

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

  let users = find_following_by_login(&db, &login, pagination_arguments)
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

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(users.edges.len(), 0);
  assert_eq!(users.page_info.has_previous_page, false);
}
