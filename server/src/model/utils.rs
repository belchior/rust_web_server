use crate::lib::cursor_connection::{cursor_to_reference, CursorConnection, Direction, ReferenceFrom};
use mongodb::{
  bson::{self, Document},
  error::Error as MongodbError,
};
use serde::de::DeserializeOwned;

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

pub fn to_cursor_connection<T: DeserializeOwned>(
  result: Vec<Result<Document, MongodbError>>,
  reference_from: ReferenceFrom<T>,
) -> CursorConnection<T> {
  let repositories = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<T>>();

  CursorConnection::new(repositories, reference_from)
}
