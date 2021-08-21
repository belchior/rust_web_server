use crate::http::cursor_connection::{cursor_to_reference, CursorConnection, Direction};
use crate::model::repository::Repository;
use crate::model::user::User;
use mongodb::bson;

pub fn to_object_id(cursor: Option<String>) -> Option<bson::oid::ObjectId> {
  if cursor.is_none() {
    return None;
  }

  let reference = cursor_to_reference(cursor.unwrap());
  if reference.is_err() {
    return None;
  }

  let id = bson::oid::ObjectId::with_string(&reference.unwrap());
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

pub fn repository_to_cursor_connection(models_list: Vec<Repository>) -> CursorConnection<Repository> {
  let reference_from = |item: &Repository| item._id.to_hex();
  CursorConnection::new(models_list, reference_from)
}

pub fn users_to_cursor_connection(models_list: Vec<User>) -> CursorConnection<User> {
  let reference_from = |item: &User| item._id.to_hex();
  CursorConnection::new(models_list, reference_from)
}
