use crate::lib::cursor_connection::PaginationArguments;
use crate::model::user::*;
use crate::setup::mock;
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_an_existing_user() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_foo".to_owned();

  let user = find_user_by_login(&db_client, &login).await.unwrap().unwrap();

  assert_eq!(user.email, "foo@email.com".to_owned());
}

#[actix_rt::test]
async fn should_dont_panic_when_user_is_not_found() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_xxx".to_owned();

  let user = find_user_by_login(&db_client, &login).await.unwrap();

  assert_eq!(user, None);
}

#[actix_rt::test]
async fn should_find_users_organizations() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_foo".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_user_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, "organization_acme");
}

#[actix_rt::test]
async fn should_dont_panic_when_organization_is_not_found() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "empty_user".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_user_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 0);
}

#[actix_rt::test]
async fn should_find_users_starred_repositories() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_bar".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_user_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_tux");
}

#[actix_rt::test]
async fn should_dont_panic_when_starred_reposiotry_is_not_found() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "empty_user".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_user_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_find_users_followers() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_foo".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(2),
    after: None,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(users.len(), 2);
  assert_eq!(users[0].login, "user_bar");
  assert_eq!(users[1].login, "user_dee");
}

#[actix_rt::test]
async fn should_dont_panic_when_follower_is_not_found() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "empty_user".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_followers_by_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_find_followed() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_dee".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(2),
    after: None,
    last: None,
    before: None,
  };

  let users = find_followed_by_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(users.len(), 2);
  assert_eq!(users[0].login, "user_foo");
  assert_eq!(users[1].login, "user_bar");
}

#[actix_rt::test]
async fn should_dont_panic_when_following_is_not_found() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "empty_user".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_followed_by_login(&db_client, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_convert_a_follower_list_into_cursor_connection_of_users() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let user_login = "user_foo".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };
  let users = find_followers_by_login(&db_client, &user_login, pagination_argument)
    .await
    .unwrap();

  let cursor_connection = followers_to_cursor_connection(&db_client, &user_login, Ok(users))
    .await
    .unwrap();

  assert_eq!(cursor_connection.edges.len(), 1);
  assert_eq!(cursor_connection.edges[0].node.login, "user_bar");
  assert_eq!(cursor_connection.page_info.has_previous_page, false);
  assert_eq!(cursor_connection.page_info.has_next_page, true);
}

#[actix_rt::test]
async fn should_convert_a_followed_list_into_cursor_connection_of_users() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let user_login = "user_dee".to_owned();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };
  let users = find_followed_by_login(&db_client, &user_login, pagination_argument)
    .await
    .unwrap();

  let cursor_connection = followed_to_cursor_connection(&db_client, &user_login, Ok(users))
    .await
    .unwrap();

  assert_eq!(cursor_connection.edges.len(), 1);
  assert_eq!(cursor_connection.edges[0].node.login, "user_foo");
  assert_eq!(cursor_connection.page_info.has_previous_page, false);
  assert_eq!(cursor_connection.page_info.has_next_page, true);
}

/// Paginating Organizations

#[actix_rt::test]
async fn should_paginating_organizations_from_start_to_end() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_foo".to_owned();

  // should find the first organization

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, "organization_acme");

  let end_cursor = Some(base64::encode(organizations[0].id.to_string()));

  // should find the last organization

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, "organization_foo");

  let end_cursor = Some(base64::encode(organizations[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_organizations_from_end_to_start() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_foo".to_owned();

  // should find the last organization

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let organizations = find_organizations_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, "organization_foo");

  let start_cursor = Some(base64::encode(organizations[0].id.to_string()));

  // should find the first organization

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let organizations = find_organizations_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, "organization_acme");

  let start_cursor = Some(base64::encode(organizations[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let organizations = find_organizations_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 0);
}

/// Paginating Starred Repositories

#[actix_rt::test]
async fn should_paginating_starred_repositories_from_start_to_end() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_bar".to_owned();

  // should find the first starred repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_tux");

  let end_cursor = Some(base64::encode(repositories[0].id.to_string()));

  // should find the last starred repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_dee");

  let end_cursor = Some(base64::encode(repositories[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_starred_repositories_from_end_to_start() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_bar".to_owned();

  // should find the last starred repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let repositories = find_starred_repositories_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_dee");

  let start_cursor = Some(base64::encode(repositories[0].id.to_string()));

  // should find the first starred repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_starred_repositories_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, "repository_tux");

  let start_cursor = Some(base64::encode(repositories[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_starred_repositories_by_user_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

/// Paginating Followers

#[actix_rt::test]
async fn should_paginating_followers_from_start_to_end() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_foo".to_owned();

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_bar");

  let end_cursor = Some(base64::encode(users[0].id.to_string()));

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_dee");

  let end_cursor = Some(base64::encode(users[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_followers_from_end_to_start() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_foo".to_owned();

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_followers_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_dee");

  let start_cursor = Some(base64::encode(users[0].id.to_string()));

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_followers_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_bar");

  let start_cursor = Some(base64::encode(users[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_followers_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

/// Paginating Following

#[actix_rt::test]
async fn should_paginating_following_from_start_to_end() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_dee".to_owned();

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_followed_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_foo");

  let end_cursor = Some(base64::encode(users[0].id.to_string()));

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_followed_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_bar");

  let end_cursor = Some(base64::encode(users[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_followed_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_following_from_end_to_start() {
  let poll = mock::setup().await;
  let db_client = poll.get().await.unwrap();
  let login = "user_dee".to_owned();

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_followed_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_bar");

  let start_cursor = Some(base64::encode(users[0].id.to_string()));

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_followed_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, "user_foo");

  let start_cursor = Some(base64::encode(users[0].id.to_string()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_followed_by_login(&db_client, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}
