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
