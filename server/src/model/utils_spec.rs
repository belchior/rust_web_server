use super::utils::*;
use crate::lib::cursor_connection::Direction;
use mongodb::bson;
use pretty_assertions::assert_eq;

#[test]
fn should_convert_cursor_to_object_id() {
  let cursor = Some("NWU4MGVjN2Q1MmYxNTcyMGJlMmJiYTc5".to_owned());
  let object_id = to_object_id(cursor);
  let expected_object_id = bson::oid::ObjectId::parse_str("5e80ec7d52f15720be2bba79").unwrap();

  assert_eq!(object_id, Some(expected_object_id));
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
