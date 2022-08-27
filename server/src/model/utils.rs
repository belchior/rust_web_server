use crate::lib::cursor_connection::cursor_to_reference;
use crate::model;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ProfileType {
  User,
  Organization,
}

pub fn parse_cursor(cursor: Option<String>) -> Option<i32> {
  if cursor.is_none() {
    return None;
  }
  let reference = cursor_to_reference(cursor.unwrap());
  if reference.is_err() {
    return None;
  }
  let id = reference.unwrap().parse::<i32>();
  if id.is_err() {
    return None;
  }
  let id = id.unwrap();

  Some(id)
}

pub async fn pages_previous_and_next<'a>(
  db: &Client,
  query: String,
  params: Vec<model::QueryParam<'a>>,
) -> Result<(bool, bool), ClientError> {
  let result = db.query(query.as_str(), &params[..]).await?;
  let handler = |acc: (bool, bool), row: Row| match row.try_get::<'_, _, &str>("page") {
    Ok("previous") => (true, acc.1),
    Ok("next") => (acc.0, true),
    _ => acc,
  };
  let result = result.into_iter().fold((false, false), handler);

  Ok(result)
}
