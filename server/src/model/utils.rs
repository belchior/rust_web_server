use super::{organization::Organization, repository::Repository, user::User};
use crate::lib::cursor_connection::{cursor_to_reference, CursorConnection, Direction};
use mongodb::{
  bson::{self, Document},
  error::Error as MongodbError,
};

pub fn to_object_id(cursor: Option<String>) -> Option<bson::oid::ObjectId> {
  if cursor.is_none() {
    return None;
  }

  let reference = cursor_to_reference(cursor.unwrap());
  if reference.is_err() {
    return None;
  }

  let id = bson::oid::ObjectId::parse_str(&reference.unwrap());
  if id.is_err() {
    return None;
  }

  let id = id.unwrap();
  Some(id)
}

pub fn to_order(direction: &Direction) -> i32 {
  match direction {
    Direction::Forward => 1,
    _ => -1,
  }
}

pub fn to_operator(direction: &Direction) -> &'static str {
  match direction {
    Direction::Forward => "$gt",
    _ => "$lt",
  }
}

pub fn repositories_to_cursor_connection(result: Vec<Result<Document, MongodbError>>) -> CursorConnection<Repository> {
  let reference_from = |item: &Repository| item._id.to_hex();
  let repositories = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<Repository>>();

  CursorConnection::new(repositories, reference_from)
}

pub fn users_to_cursor_connection(result: Vec<Result<Document, MongodbError>>) -> CursorConnection<User> {
  let reference_from = |item: &User| item._id.to_hex();
  let users = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<User>>();

  CursorConnection::new(users, reference_from)
}

pub fn organizations_to_cursor_connection(
  result: Vec<Result<Document, MongodbError>>,
) -> CursorConnection<Organization> {
  let reference_from = |item: &Organization| item._id.to_hex();
  let organizations = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<Organization>>();

  CursorConnection::new(organizations, reference_from)
}
