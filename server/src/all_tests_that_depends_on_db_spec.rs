#[tokio::test]
async fn main() {
  connection_spec::should_connect_to_database().await;
  model_organization::should_convert_a_organization_members_into_cursor_connection_of_users().await;
  model_organization::should_dont_panic_when_organization_is_not_found().await;
  model_organization::should_dont_panic_when_person_is_not_found().await;
  model_organization::should_find_organizations_people().await;
  model_organization::should_find_un_existing_organization().await;
  model_organization::should_paginating_people_from_end_to_start().await;
  model_organization::should_paginating_people_from_start_to_end().await;
  model_repository::should_convert_a_repository_list_into_cursor_connection_of_repositories().await;
  model_repository::should_dont_panic_when_repository_is_not_found().await;
  model_repository::should_find_owners_repositories().await;
  model_repository::should_paginating_repositories_from_end_to_start().await;
  model_repository::should_paginating_repositories_from_start_to_end().await;
  model_user::should_convert_a_follower_list_into_cursor_connection_of_users().await;
  model_user::should_convert_a_following_list_into_cursor_connection_of_users().await;
  model_user::should_dont_panic_when_follower_is_not_found().await;
  model_user::should_dont_panic_when_following_is_not_found().await;
  model_user::should_dont_panic_when_organization_is_not_found().await;
  model_user::should_dont_panic_when_starred_reposiotry_is_not_found().await;
  model_user::should_dont_panic_when_user_is_not_found().await;
  model_user::should_find_an_existing_user().await;
  model_user::should_find_users_followers().await;
  model_user::should_find_users_following().await;
  model_user::should_find_users_organizations().await;
  model_user::should_find_users_starred_repositories().await;
  model_user::should_paginating_followers_from_end_to_start().await;
  model_user::should_paginating_followers_from_start_to_end().await;
  model_user::should_paginating_following_from_end_to_start().await;
  model_user::should_paginating_following_from_start_to_end().await;
  model_user::should_paginating_organizations_from_end_to_start().await;
  model_user::should_paginating_organizations_from_start_to_end().await;
  model_user::should_paginating_starred_repositories_from_end_to_start().await;
  model_user::should_paginating_starred_repositories_from_start_to_end().await;
  route_organization::should_find_people_of_the_organization().await;
  route_organization::should_find_repositories_of_the_organization().await;
  route_organization::should_match_an_organization().await;
  route_organization::should_not_find_people_of_a_unknown_organization().await;
  route_organization::should_not_find_people_of_the_organization_when_the_org_is_empty().await;
  route_organization::should_not_find_repositories_of_a_unknown_organization().await;
  route_organization::should_not_find_repositories_of_the_organization_when_the_org_does_not_have_one().await;
  route_profile::should_match_a_user_profile().await;
  route_profile::should_match_an_organization_profile().await;
  route_profile::should_return_profile_not_found_when_the_login_is_unknown().await;
  route_spec::should_dont_panic_when_configure_on_new_app().await;
  route_spec::should_return_a_client_error().await;
  route_spec::should_return_the_message_resource_not_found().await;
  route_user::should_find_followers_of_the_user().await;
  route_user::should_find_following_of_the_user().await;
  route_user::should_find_organizations_of_the_user().await;
  route_user::should_find_repositories_of_the_user().await;
  route_user::should_find_starred_repositories_of_the_user().await;
  route_user::should_match_a_specified_user().await;
  route_user::should_not_find_followers_of_a_unknown_user().await;
  route_user::should_not_find_followers_of_the_user().await;
  route_user::should_not_find_following_of_a_unknown_user().await;
  route_user::should_not_find_following_of_the_user().await;
  route_user::should_not_find_organizations_of_a_unknown_user().await;
  route_user::should_not_find_organizations_of_the_user().await;
  route_user::should_not_find_repositories_of_a_unknown_user().await;
  route_user::should_not_find_repositories_of_the_user().await;
  route_user::should_not_find_starred_repositories_of_a_unknown_user().await;
  route_user::should_not_find_starred_repositories_of_the_user().await;
}

mod connection_spec {
  use crate::infrastructure::database;
  use pretty_assertions::assert_eq;

  pub async fn should_connect_to_database() {
    let pool = database::get_connection().await;
    let db = pool.get().await.unwrap();
    let result = db.query_one("SELECT current_database()", &[]).await.unwrap();
    let db_name: &str = result.get(0);

    assert_eq!(db_name, "database");
  }
}

mod model_organization {
  use crate::infrastructure::{
    database::{self, cursor_connection::PaginationArguments},
    mock,
  };
  use pretty_assertions::assert_eq;

  pub async fn should_find_un_existing_organization() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("organization_foo_{suffix}");
    let organization = database::organization::find_organization_by_login(&login)
      .await
      .unwrap()
      .unwrap();

    assert_eq!(organization.login, login);
  }

  pub async fn should_dont_panic_when_organization_is_not_found() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("organization_???_{suffix}");
    let organization = database::organization::find_organization_by_login(&login)
      .await
      .unwrap();

    assert_eq!(organization, None);
  }

  pub async fn should_find_organizations_people() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("organization_acme_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let users = database::organization::find_people_by_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_foo_{suffix}"));
  }

  pub async fn should_dont_panic_when_person_is_not_found() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("organization_empty_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let users = database::organization::find_people_by_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(users.len(), 0);
  }

  pub async fn should_convert_a_organization_members_into_cursor_connection_of_users() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let organization_login = format!("organization_acme_{suffix}");
    let organization = database::organization::find_organization_by_login(&organization_login)
      .await
      .unwrap()
      .unwrap();

    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let users = database::organization::find_people_by_login(&organization.login, pagination_argument)
      .await
      .unwrap();
    let first_user_login = users[0].login.clone();
    let cursor_connection =
      database::organization::organizations_users_to_cursor_connection(&organization_login, Ok(users))
        .await
        .unwrap();

    assert_eq!(cursor_connection.edges.len(), 1);
    assert_eq!(cursor_connection.edges[0].node.login, first_user_login);
    assert_eq!(cursor_connection.page_info.has_previous_page, false);
    assert_eq!(cursor_connection.page_info.has_next_page, true);
  }

  /// Paginating People

  pub async fn should_paginating_people_from_start_to_end() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("organization_acme_{suffix}");

    // should find the first user

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let users = database::organization::find_people_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_foo_{suffix}"));

    let end_cursor = Some(base64::encode(users[0].id.to_string()));

    // should find the last user

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let users = database::organization::find_people_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_dee_{suffix}"));

    let end_cursor = Some(base64::encode(users[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let users = database::organization::find_people_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 0);
  }

  pub async fn should_paginating_people_from_end_to_start() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("organization_acme_{suffix}");

    // should find the last user

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: None,
    };

    let users = database::organization::find_people_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_dee_{suffix}"));

    let start_cursor = Some(base64::encode(users[0].id.to_string()));

    // should find the first user

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let users = database::organization::find_people_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_foo_{suffix}"));

    let start_cursor = Some(base64::encode(users[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let users = database::organization::find_people_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 0);
  }
}

mod model_repository {
  use crate::infrastructure::{
    database::{self, cursor_connection::PaginationArguments},
    mock,
  };
  use pretty_assertions::assert_eq;

  pub async fn should_find_owners_repositories() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let owner_login = format!("organization_acme_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_tux_{suffix}"));
  }

  pub async fn should_dont_panic_when_repository_is_not_found() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let owner_login = format!("empty_user_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 0);
  }

  pub async fn should_convert_a_repository_list_into_cursor_connection_of_repositories() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let owner_login = format!("organization_acme_{suffix}");

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: None,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_arguments)
      .await
      .unwrap();

    let first_repository_name = repositories[0].name.clone();

    let cursor_connection = database::repository::repositories_to_cursor_connection(&owner_login, Ok(repositories))
      .await
      .unwrap();

    assert_eq!(cursor_connection.edges.len(), 1);
    assert_eq!(cursor_connection.edges[0].node.name, first_repository_name);
    assert_eq!(cursor_connection.page_info.has_previous_page, true);
    assert_eq!(cursor_connection.page_info.has_next_page, false);
  }

  /// Paginating Repositories

  pub async fn should_paginating_repositories_from_start_to_end() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let owner_login = format!("organization_acme_{suffix}");

    // should find the first repository

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_tux_{suffix}"));

    let end_cursor = Some(base64::encode(repositories[0].id.to_string()));

    // should find the last repository

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_mar_{suffix}"));

    let end_cursor = Some(base64::encode(repositories[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 0);
  }

  pub async fn should_paginating_repositories_from_end_to_start() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let owner_login = format!("organization_acme_{suffix}");

    // should find the last repository

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: None,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_mar_{suffix}"));

    let start_cursor = Some(base64::encode(repositories[0].id.to_string()));

    // should find the first repository

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_tux_{suffix}"));

    let start_cursor = Some(base64::encode(repositories[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let repositories = database::repository::find_repositories_by_owner_login(&owner_login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 0);
  }
}

mod model_user {
  use crate::infrastructure::{
    database::{self, cursor_connection::PaginationArguments},
    mock,
  };
  use pretty_assertions::assert_eq;

  pub async fn should_find_an_existing_user() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_foo_{suffix}");

    let user = database::user::find_user_by_login(&login).await.unwrap().unwrap();

    assert_eq!(user.email, "foo@email.com".to_owned());
  }

  pub async fn should_dont_panic_when_user_is_not_found() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_???_{suffix}");

    let user = database::user::find_user_by_login(&login).await.unwrap();

    assert_eq!(user, None);
  }

  pub async fn should_find_users_organizations() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_foo_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let organizations = database::user::find_organizations_by_user_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(organizations.len(), 1);
    assert_eq!(organizations[0].login, format!("organization_acme_{suffix}"));
  }

  pub async fn should_dont_panic_when_organization_is_not_found() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("empty_user_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let organizations = database::user::find_organizations_by_user_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(organizations.len(), 0);
  }

  pub async fn should_find_users_starred_repositories() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_bar_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let repositories = database::user::find_starred_repositories_by_user_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_tux_{suffix}"));
  }

  pub async fn should_dont_panic_when_starred_reposiotry_is_not_found() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("empty_user_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let repositories = database::user::find_starred_repositories_by_user_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 0);
  }

  pub async fn should_find_users_followers() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_foo_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(2),
      after: None,
      last: None,
      before: None,
    };

    let users = database::user::find_followers_by_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].login, format!("user_bar_{suffix}"));
    assert_eq!(users[1].login, format!("user_dee_{suffix}"));
  }

  pub async fn should_dont_panic_when_follower_is_not_found() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("empty_user_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let repositories = database::user::find_followers_by_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 0);
  }

  pub async fn should_find_users_following() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_dee_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(2),
      after: None,
      last: None,
      before: None,
    };

    let users = database::user::find_following_by_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].login, format!("user_foo_{suffix}"));
    assert_eq!(users[1].login, format!("user_bar_{suffix}"));
  }

  pub async fn should_dont_panic_when_following_is_not_found() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("empty_user_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let repositories = database::user::find_following_by_login(&login, pagination_argument)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 0);
  }

  pub async fn should_convert_a_follower_list_into_cursor_connection_of_users() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let user_login = format!("user_foo_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };
    let users = database::user::find_followers_by_login(&user_login, pagination_argument)
      .await
      .unwrap();

    let cursor_connection = database::user::followers_to_cursor_connection(&user_login, Ok(users))
      .await
      .unwrap();

    assert_eq!(cursor_connection.edges.len(), 1);
    assert_eq!(cursor_connection.edges[0].node.login, format!("user_bar_{suffix}"));
    assert_eq!(cursor_connection.page_info.has_previous_page, false);
    assert_eq!(cursor_connection.page_info.has_next_page, true);
  }

  pub async fn should_convert_a_following_list_into_cursor_connection_of_users() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let user_login = format!("user_dee_{suffix}");
    let pagination_argument = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };
    let users = database::user::find_following_by_login(&user_login, pagination_argument)
      .await
      .unwrap();

    let cursor_connection = database::user::following_to_cursor_connection(&user_login, Ok(users))
      .await
      .unwrap();

    assert_eq!(cursor_connection.edges.len(), 1);
    assert_eq!(cursor_connection.edges[0].node.login, format!("user_foo_{suffix}"));
    assert_eq!(cursor_connection.page_info.has_previous_page, false);
    assert_eq!(cursor_connection.page_info.has_next_page, true);
  }

  /// Paginating Organizations

  pub async fn should_paginating_organizations_from_start_to_end() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_foo_{suffix}");

    // should find the first organization

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let organizations = database::user::find_organizations_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(organizations.len(), 1);
    assert_eq!(organizations[0].login, format!("organization_acme_{suffix}"));

    let end_cursor = Some(base64::encode(organizations[0].id.to_string()));

    // should find the last organization

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let organizations = database::user::find_organizations_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(organizations.len(), 1);
    assert_eq!(organizations[0].login, format!("organization_foo_{suffix}"));

    let end_cursor = Some(base64::encode(organizations[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let organizations = database::user::find_organizations_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(organizations.len(), 0);
  }

  pub async fn should_paginating_organizations_from_end_to_start() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_foo_{suffix}");

    // should find the last organization

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: None,
    };

    let organizations = database::user::find_organizations_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(organizations.len(), 1);
    assert_eq!(organizations[0].login, format!("organization_foo_{suffix}"));

    let start_cursor = Some(base64::encode(organizations[0].id.to_string()));

    // should find the first organization

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let organizations = database::user::find_organizations_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(organizations.len(), 1);
    assert_eq!(organizations[0].login, format!("organization_acme_{suffix}"));

    let start_cursor = Some(base64::encode(organizations[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let organizations = database::user::find_organizations_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(organizations.len(), 0);
  }

  /// Paginating Starred Repositories

  pub async fn should_paginating_starred_repositories_from_start_to_end() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_bar_{suffix}");

    // should find the first starred repository

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let repositories = database::user::find_starred_repositories_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_tux_{suffix}"));

    let end_cursor = Some(base64::encode(repositories[0].id.to_string()));

    // should find the last starred repository

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let repositories = database::user::find_starred_repositories_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_dee_{suffix}"));

    let end_cursor = Some(base64::encode(repositories[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let repositories = database::user::find_starred_repositories_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 0);
  }

  pub async fn should_paginating_starred_repositories_from_end_to_start() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_bar_{suffix}");

    // should find the last starred repository

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: None,
    };

    let repositories = database::user::find_starred_repositories_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_dee_{suffix}"));

    let start_cursor = Some(base64::encode(repositories[0].id.to_string()));

    // should find the first starred repository

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let repositories = database::user::find_starred_repositories_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 1);
    assert_eq!(repositories[0].name, format!("repository_tux_{suffix}"));

    let start_cursor = Some(base64::encode(repositories[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let repositories = database::user::find_starred_repositories_by_user_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(repositories.len(), 0);
  }

  /// Paginating Followers

  pub async fn should_paginating_followers_from_start_to_end() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_foo_{suffix}");

    // should find the first user

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let users = database::user::find_followers_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_bar_{suffix}"));

    let end_cursor = Some(base64::encode(users[0].id.to_string()));

    // should find the last user

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let users = database::user::find_followers_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_dee_{suffix}"));

    let end_cursor = Some(base64::encode(users[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let users = database::user::find_followers_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 0);
  }

  pub async fn should_paginating_followers_from_end_to_start() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_foo_{suffix}");

    // should find the last user

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: None,
    };

    let users = database::user::find_followers_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_dee_{suffix}"));

    let start_cursor = Some(base64::encode(users[0].id.to_string()));

    // should find the first user

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let users = database::user::find_followers_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_bar_{suffix}"));

    let start_cursor = Some(base64::encode(users[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let users = database::user::find_followers_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 0);
  }

  /// Paginating Following

  pub async fn should_paginating_following_from_start_to_end() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_dee_{suffix}");

    // should find the first user

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: None,
      last: None,
      before: None,
    };

    let users = database::user::find_following_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_foo_{suffix}"));

    let end_cursor = Some(base64::encode(users[0].id.to_string()));

    // should find the last user

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let users = database::user::find_following_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_bar_{suffix}"));

    let end_cursor = Some(base64::encode(users[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: Some(1),
      after: end_cursor,
      last: None,
      before: None,
    };

    let users = database::user::find_following_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 0);
  }

  pub async fn should_paginating_following_from_end_to_start() {
    let suffix = mock::random_suffix();
    mock::setup(&suffix).await;
    let login = format!("user_dee_{suffix}");

    // should find the last user

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: None,
    };

    let users = database::user::find_following_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_bar_{suffix}"));

    let start_cursor = Some(base64::encode(users[0].id.to_string()));

    // should find the first user

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let users = database::user::find_following_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].login, format!("user_foo_{suffix}"));

    let start_cursor = Some(base64::encode(users[0].id.to_string()));

    // should return an empty list

    let pagination_arguments = PaginationArguments {
      first: None,
      after: None,
      last: Some(1),
      before: start_cursor,
    };

    let users = database::user::find_following_by_login(&login, pagination_arguments)
      .await
      .unwrap();

    assert_eq!(users.len(), 0);
  }
}

mod route_organization {
  use crate::infrastructure::{
    database::{cursor_connection::CursorConnection, organization::Organization, repository::Repository, user::User},
    http_server::{self, utils::HttpError},
    mock,
  };
  use actix_web::{http::StatusCode, test};
  use pretty_assertions::assert_eq;

  pub async fn should_match_an_organization() {
    let suffix = mock::random_suffix();
    let login = format!("organization_foo_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/organization/{login}"),
      http_server::route::organization::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: Organization = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.login, login);
  }

  pub async fn should_find_people_of_the_organization() {
    let suffix = mock::random_suffix();
    let login = format!("organization_foo_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/organization/{login}/people"),
      http_server::route::organization::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<User> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.login, format!("user_foo_{suffix}"));
  }

  pub async fn should_not_find_people_of_the_organization_when_the_org_is_empty() {
    let suffix = mock::random_suffix();
    let login = format!("empty_org_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/organization/{login}/people"),
      http_server::route::organization::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Organization> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 0);
  }

  pub async fn should_not_find_people_of_a_unknown_organization() {
    let suffix = mock::random_suffix();
    let login = format!("organization_???_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/organization/{login}/people"),
      http_server::route::organization::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "Organization not found");
  }

  pub async fn should_find_repositories_of_the_organization() {
    let suffix = mock::random_suffix();
    let login = format!("organization_acme_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/organization/{login}/repositories"),
      http_server::route::organization::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Repository> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.name, format!("repository_tux_{suffix}"));
  }

  pub async fn should_not_find_repositories_of_the_organization_when_the_org_does_not_have_one() {
    let suffix = mock::random_suffix();
    let login = format!("empty_org_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/organization/{login}/repositories"),
      http_server::route::organization::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Organization> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 0);
  }

  pub async fn should_not_find_repositories_of_a_unknown_organization() {
    let suffix = mock::random_suffix();
    let login = format!("organization_???_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/organization/{login}/repositories"),
      http_server::route::organization::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "Organization not found");
  }
}

mod route_profile {
  use crate::infrastructure::{
    database::{organization::Organization, user::User},
    http_server::{self, utils::HttpError},
    mock,
  };
  use actix_web::{http::StatusCode, test};
  use pretty_assertions::assert_eq;

  pub async fn should_match_a_user_profile() {
    let suffix = mock::random_suffix();
    let login = format!("user_bar_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/profile/{login}"),
      http_server::route::profile::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: User = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.login, format!("user_bar_{suffix}"));
  }

  pub async fn should_match_an_organization_profile() {
    let suffix = mock::random_suffix();
    let login = format!("organization_foo_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/profile/{login}"),
      http_server::route::profile::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: Organization = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.login, format!("organization_foo_{suffix}"));
  }

  pub async fn should_return_profile_not_found_when_the_login_is_unknown() {
    let suffix = mock::random_suffix();
    let login = format!("xpto_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/profile/{login}"),
      http_server::route::profile::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "Profile not found");
  }
}

mod route_spec {
  use crate::infrastructure::http_server::{self, utils::HttpError};
  use actix_web::{App, http::StatusCode, test};

  pub async fn should_dont_panic_when_configure_on_new_app() {
    test::init_service(App::new().configure(http_server::route::config_route)).await;
  }

  pub async fn should_return_a_client_error() {
    let app = test::init_service(App::new().route("/", http_server::route::not_found())).await;
    let req = test::TestRequest::get().to_request();
    let res = test::call_service(&app, req).await;

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
  }

  pub async fn should_return_the_message_resource_not_found() {
    let app = test::init_service(App::new().route("/", http_server::route::not_found())).await;
    let req = test::TestRequest::get().to_request();
    let res = test::call_service(&app, req).await;
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(body.message, "Resource not found");
  }
}

mod route_user {
  use crate::infrastructure::{
    database::{cursor_connection::CursorConnection, organization::Organization, repository::Repository, user::User},
    http_server::{self, utils::HttpError},
    mock,
  };
  use actix_web::{http::StatusCode, test};
  use pretty_assertions::assert_eq;

  pub async fn should_match_a_specified_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_foo_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: User = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.login, format!("user_foo_{suffix}"));
  }

  // Organizations

  pub async fn should_find_organizations_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_foo_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/organizations"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Organization> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.login, format!("organization_acme_{suffix}"));
  }

  pub async fn should_not_find_organizations_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("empty_user_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/organizations"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Organization> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 0);
  }

  pub async fn should_not_find_organizations_of_a_unknown_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_???_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/organizations"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "User not found");
  }

  // Repositories

  pub async fn should_find_repositories_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_bar_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/repositories"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Repository> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges[0].node.name, format!("repository_bar_{suffix}"));
  }

  pub async fn should_not_find_repositories_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("empty_user_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/repositories"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Repository> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 0);
  }

  pub async fn should_not_find_repositories_of_a_unknown_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_???_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/repositories"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "User not found");
  }

  // Starred Repositories

  pub async fn should_find_starred_repositories_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_bar_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/starred-repositories"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Repository> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 2);
    assert_eq!(body.edges[0].node.name, format!("repository_tux_{suffix}"));
  }

  pub async fn should_not_find_starred_repositories_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_dee_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/starred-repositories"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<Repository> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 0);
  }

  pub async fn should_not_find_starred_repositories_of_a_unknown_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_???_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/starred-repositories"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "User not found");
  }

  // Followers

  pub async fn should_find_followers_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_bar_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/followers"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<User> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 1);
    assert_eq!(body.edges[0].node.login, format!("user_dee_{suffix}"));
  }

  pub async fn should_not_find_followers_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("empty_user_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/followers"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<User> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 0);
  }

  pub async fn should_not_find_followers_of_a_unknown_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_???_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/followers"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "User not found");
  }

  // Following

  pub async fn should_find_following_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_bar_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/following"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<User> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 1);
    assert_eq!(body.edges[0].node.login, format!("user_foo_{suffix}"));
  }

  pub async fn should_not_find_following_of_the_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_foo_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/following"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: CursorConnection<User> = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.edges.len(), 0);
  }

  pub async fn should_not_find_following_of_a_unknown_user() {
    let suffix = mock::random_suffix();
    let login = format!("user_???_{suffix}");
    let res = mock::make_request(
      mock::HttpMethod::Get,
      &format!("/user/{login}/following"),
      http_server::route::user::scope(),
      &suffix,
    )
    .await;
    let status = res.status();
    let body: HttpError = test::read_body_json(res).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body.message, "User not found");
  }
}
