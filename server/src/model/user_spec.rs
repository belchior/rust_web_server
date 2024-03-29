use crate::lib::cursor_connection::PaginationArguments;
use crate::model::{organization, user::*};
use crate::setup::mock;
use base64;
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_an_existing_user() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let db = mock::setup(&sufix).await;

  let user = find_user_by_login(&db, &login).await.unwrap().unwrap();

  assert_eq!(user.email, "foo@email.com".to_owned());
}

#[actix_rt::test]
async fn should_dont_panic_when_user_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("user_xxx_{sufix}");
  let db = mock::setup(&sufix).await;

  let user = find_user_by_login(&db, &login).await.unwrap();

  assert_eq!(user, None);
}

#[actix_rt::test]
async fn should_find_users_organizations() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, format!("organization_foo_{sufix}"));
}

#[actix_rt::test]
async fn should_dont_panic_when_organization_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("empty_user_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 0);
}

#[actix_rt::test]
async fn should_find_users_starred_repositories() {
  let sufix = mock::random_sufix();
  let login = format!("user_bar_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_tux_{sufix}"));
}

#[actix_rt::test]
async fn should_dont_panic_when_starred_reposiotry_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("empty_user_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_argument)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_find_users_followers() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(2),
    after: None,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db, &login, pagination_argument).await.unwrap();

  assert_eq!(users.len(), 2);
  assert_eq!(users[0].login, format!("user_bar_{sufix}"));
  assert_eq!(users[1].login, format!("user_dee_{sufix}"));
}

#[actix_rt::test]
async fn should_dont_panic_when_follower_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("empty_user_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_followers_by_login(&db, &login, pagination_argument).await.unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_find_users_following() {
  let sufix = mock::random_sufix();
  let login = format!("user_dee_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(2),
    after: None,
    last: None,
    before: None,
  };

  let users = find_following_by_login(&db, &login, pagination_argument).await.unwrap();

  assert_eq!(users.len(), 2);
  assert_eq!(users[0].login, format!("user_foo_{sufix}"));
  assert_eq!(users[1].login, format!("user_bar_{sufix}"));
}

#[actix_rt::test]
async fn should_dont_panic_when_following_is_not_found() {
  let sufix = mock::random_sufix();
  let login = format!("empty_user_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_following_by_login(&db, &login, pagination_argument).await.unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_convert_a_user_list_into_cursor_connection_of_users() {
  let sufix = mock::random_sufix();
  let organization_login = format!("organization_acme_{sufix}");
  let db = mock::setup(&sufix).await;
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };
  let users = organization::find_people_by_login(&db, &organization_login, pagination_argument)
    .await
    .unwrap();

  let cursor_connection = users_to_cursor_connection(&db, &organization_login, Ok(users))
    .await
    .unwrap();

  assert_eq!(cursor_connection.edges.len(), 1);
  assert_eq!(cursor_connection.edges[0].node.login, format!("user_foo_{sufix}"));
  assert_eq!(cursor_connection.page_info.has_previous_page, false);
  assert_eq!(cursor_connection.page_info.has_next_page, true);
}

/// Paginating Organizations

#[actix_rt::test]
async fn should_paginating_organizations_from_start_to_end() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the first organization

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, format!("organization_foo_{sufix}"));

  let end_cursor = Some(base64::encode(organizations[0]._id.to_hex()));

  // should find the last organization

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, format!("organization_acme_{sufix}"));

  let end_cursor = Some(base64::encode(organizations[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_organizations_from_end_to_start() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the last organization

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, format!("organization_acme_{sufix}"));

  let start_cursor = Some(base64::encode(organizations[0]._id.to_hex()));

  // should find the first organization

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 1);
  assert_eq!(organizations[0].login, format!("organization_foo_{sufix}"));

  let start_cursor = Some(base64::encode(organizations[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let organizations = find_organizations_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(organizations.len(), 0);
}

/// Paginating Starred Repositories

#[actix_rt::test]
async fn should_paginating_starred_repositories_from_start_to_end() {
  let sufix = mock::random_sufix();
  let login = format!("user_bar_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the first starred repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_tux_{sufix}"));

  let end_cursor = Some(base64::encode(repositories[0]._id.to_hex()));

  // should find the last starred repository

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_dee_{sufix}"));

  let end_cursor = Some(base64::encode(repositories[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_starred_repositories_from_end_to_start() {
  let sufix = mock::random_sufix();
  let login = format!("user_bar_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the last starred repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_dee_{sufix}"));

  let start_cursor = Some(base64::encode(repositories[0]._id.to_hex()));

  // should find the first starred repository

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 1);
  assert_eq!(repositories[0].name, format!("repository_tux_{sufix}"));

  let start_cursor = Some(base64::encode(repositories[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let repositories = find_starred_repositories_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(repositories.len(), 0);
}

/// Paginating Followers

#[actix_rt::test]
async fn should_paginating_followers_from_start_to_end() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_bar_{sufix}"));

  let end_cursor = Some(base64::encode(users[0]._id.to_hex()));

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

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

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_followers_from_end_to_start() {
  let sufix = mock::random_sufix();
  let login = format!("user_foo_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

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

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_bar_{sufix}"));

  let start_cursor = Some(base64::encode(users[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_followers_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

/// Paginating Following

#[actix_rt::test]
async fn should_paginating_following_from_start_to_end() {
  let sufix = mock::random_sufix();
  let login = format!("user_dee_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

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

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_bar_{sufix}"));

  let end_cursor = Some(base64::encode(users[0]._id.to_hex()));

  // should return an empty list

  let pagination_arguments = PaginationArguments {
    first: Some(1),
    after: end_cursor,
    last: None,
    before: None,
  };

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}

#[actix_rt::test]
async fn should_paginating_following_from_end_to_start() {
  let sufix = mock::random_sufix();
  let login = format!("user_dee_{sufix}");
  let db = mock::setup(&sufix).await;

  // should find the last user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: None,
  };

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 1);
  assert_eq!(users[0].login, format!("user_bar_{sufix}"));

  let start_cursor = Some(base64::encode(users[0]._id.to_hex()));

  // should find the first user

  let pagination_arguments = PaginationArguments {
    first: None,
    after: None,
    last: Some(1),
    before: start_cursor,
  };

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

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

  let users = find_following_by_login(&db, &login, pagination_arguments)
    .await
    .unwrap();

  assert_eq!(users.len(), 0);
}
