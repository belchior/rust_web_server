use crate::{
  lib::cursor_connection::{cursor_to_reference, CursorConnection, Direction, ReferenceFrom},
  model,
};
use mongodb::{
  bson::{self, doc, oid::ObjectId, Document},
  error::Error as MongodbError,
};
use serde::de::DeserializeOwned;

pub fn to_object_id(cursor: Option<String>) -> Option<ObjectId> {
  if cursor.is_none() {
    return None;
  }

  let reference = cursor_to_reference(cursor.unwrap());
  if reference.is_err() {
    return None;
  }

  let id = ObjectId::parse_str(&reference.unwrap());
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
  has_previous_page: bool,
  has_next_page: bool,
  reference_from: ReferenceFrom<T>,
) -> CursorConnection<T> {
  let items = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<T>>();

  CursorConnection::new(items, has_previous_page, has_next_page, reference_from)
}
