use crate::infrastructure::database::{DBConnection, QueryParam, cursor_connection::cursor_to_reference};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ProfileType {
  User,
  Organization,
}

pub fn parse_cursor(cursor: Option<String>) -> Option<i64> {
  let Some(cursor) = cursor else {
    return None;
  };
  let Ok(reference) = cursor_to_reference(cursor) else {
    return None;
  };
  let Ok(id) = reference.parse::<i64>() else {
    return None;
  };

  Some(id)
}

pub async fn pages_previous_and_next<'a>(
  db: &DBConnection,
  query: String,
  params: Vec<QueryParam<'a>>,
) -> Result<(bool, bool), ClientError> {
  let result = db.query(&query, &params[..]).await?;
  let handler = |acc: (bool, bool), row: Row| match row.try_get::<'_, _, &str>("page") {
    Ok("previous") => (true, acc.1),
    Ok("next") => (acc.0, true),
    _ => acc,
  };
  let result = result.into_iter().fold((false, false), handler);

  Ok(result)
}
