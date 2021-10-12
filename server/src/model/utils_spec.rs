use super::{
  organization::Organization,
  repository::{Owner, Repository},
  user::User,
  utils::*,
};
use crate::lib::cursor_connection::{CursorConnection, Direction};
use mongodb::bson;
use pretty_assertions::assert_eq;

#[cfg(test)]
mod describe_to_object_id {
  use super::*;
  use base64;
  use pretty_assertions::assert_eq;

  #[test]
  fn should_convert_cursor_to_object_id() {
    let cursor = Some("NWU4MGVjN2Q1MmYxNTcyMGJlMmJiYTc5".to_string());
    let object_id = to_object_id(cursor);
    let expected_object_id = bson::oid::ObjectId::parse_str("5e80ec7d52f15720be2bba79").unwrap();

    assert_eq!(object_id, Some(expected_object_id));
  }

  #[test]
  fn should_return_none_when_cursor_is_none() {
    let cursor = None;
    let object_id = to_object_id(cursor);

    assert_eq!(object_id, None);
  }

  #[test]
  fn should_return_none_when_cursor_is_not_a_valid_cursor() {
    let cursor = Some("invalid_cursor".to_string());
    let object_id = to_object_id(cursor);

    assert_eq!(object_id, None);
  }

  #[test]
  fn should_return_none_when_cursor_is_not_a_valid_object_id() {
    let cursor = Some(base64::encode("invalid_object_id"));
    let object_id = to_object_id(cursor);

    assert_eq!(object_id, None);
  }
}

#[test]
fn should_convert_direction_to_order() {
  let order = to_order(&Direction::Forward);
  let expected_order = 1;
  assert_eq!(order, expected_order);

  let order = to_order(&Direction::Backward);
  let expected_order = -1;
  assert_eq!(order, expected_order);
}

#[test]
fn should_convert_direction_to_operator() {
  let operator = to_operator(&Direction::Forward);
  let expected_operator = "$gt";
  assert_eq!(operator, expected_operator);

  let operator = to_operator(&Direction::Backward);
  let expected_operator = "$lt";
  assert_eq!(operator, expected_operator);
}

#[test]
fn should_convert_a_list_repositories_to_cursor_connection() {
  let repo_id = bson::oid::ObjectId::new();
  let owner_id = bson::oid::ObjectId::new();
  let reference_from = |item: &Repository| item._id.to_hex();
  let expected_cursor_connection = CursorConnection::new(
    vec![Repository {
      _id: repo_id,
      description: None,
      fork_count: 0,
      license_info: None,
      name: "Repo".to_string(),
      owner: Owner { _id: owner_id },
      primary_language: None,
    }],
    reference_from,
  );
  let cursor_connection = repositories_to_cursor_connection(vec![Repository {
    _id: repo_id,
    description: None,
    fork_count: 0,
    license_info: None,
    name: "Repo".to_string(),
    owner: Owner { _id: owner_id },
    primary_language: None,
  }]);

  assert_eq!(cursor_connection, expected_cursor_connection);
}

#[test]
fn should_convert_a_list_users_to_cursor_connection() {
  let user_id = bson::oid::ObjectId::new();
  let reference_from = |item: &User| item._id.to_hex();
  let expected_cursor_connection = CursorConnection::new(
    vec![User {
      _id: user_id,
      avatar_url: "https://image.com/avatar".to_string(),
      bio: None,
      email: "email@email.com".to_string(),
      login: "login".to_string(),
      name: None,
      organizations: None,
      url: "https://image.com/avatar".to_string(),
      website_url: None,
      typename: "User".to_string(),
    }],
    reference_from,
  );
  let cursor_connection = users_to_cursor_connection(vec![User {
    _id: user_id,
    avatar_url: "https://image.com/avatar".to_string(),
    bio: None,
    email: "email@email.com".to_string(),
    login: "login".to_string(),
    name: None,
    organizations: None,
    url: "https://image.com/avatar".to_string(),
    website_url: None,
    typename: "User".to_string(),
  }]);

  assert_eq!(cursor_connection, expected_cursor_connection);
}

#[test]
fn should_convert_a_list_organizations_to_cursor_connection() {
  let organization_id = bson::oid::ObjectId::new();
  let reference_from = |item: &Organization| item._id.to_hex();
  let expected_cursor_connection = CursorConnection::new(
    vec![Organization {
      _id: organization_id,
      avatar_url: "https://image.com/avatar".to_string(),
      description: None,
      location: None,
      login: "login".to_string(),
      name: None,
      people: None,
      url: "https://image.com/avatar".to_string(),
      website_url: None,
      typename: "Organization".to_string(),
    }],
    reference_from,
  );
  let cursor_connection = organizations_to_cursor_connection(vec![Organization {
    _id: organization_id,
    avatar_url: "https://image.com/avatar".to_string(),
    description: None,
    location: None,
    login: "login".to_string(),
    name: None,
    people: None,
    url: "https://image.com/avatar".to_string(),
    website_url: None,
    typename: "Organization".to_string(),
  }]);

  assert_eq!(cursor_connection, expected_cursor_connection);
}
