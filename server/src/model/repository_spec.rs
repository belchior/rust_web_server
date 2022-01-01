use super::{repository::*, user::find_user_by_login};
use crate::{lib::cursor_connection::PaginationArguments, mock};
use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn should_find_users_repositories() {
  let db = mock::setup().await;
  let login = "user_bar".to_owned();
  let user = find_user_by_login(&db, &login).await.unwrap().unwrap();
  let pagination_argument = PaginationArguments {
    first: Some(1),
    after: None,
    last: None,
    before: None,
  };

  let repositories = find_repositories_by_user_id(&db, &user._id, pagination_argument)
    .await
    .unwrap()
    .unwrap();

  assert_eq!(repositories.edges.len(), 1);
  assert_eq!(repositories.edges[0].node.name, "repository_bar");
}
