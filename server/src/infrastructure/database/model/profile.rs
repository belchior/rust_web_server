use crate::infrastructure::database::{
  self, QueryParam,
  cursor_connection::{CursorConnection, Direction, PaginationArguments, cursor_to_reference},
  user::User,
  utils,
};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use sql_query_builder as sql;
use tokio_postgres::{Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Follower {
  #[serde(flatten)]
  pub user: User,
  pub following_at: DateTime<Utc>,
}
impl From<Row> for Follower {
  fn from(row: Row) -> Self {
    Self {
      following_at: row.get("following_at"),
      user: User::from(row),
    }
  }
}

// Finds

pub async fn find_followers_by_login(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<Follower>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let (query, params) = query_find_followers_by_login(login, direction, &limit, cursor);
  let result = db.query(&query, &params[..]).await?;
  let followers = result.into_iter().map(|row| Follower::from(row)).collect::<Vec<_>>();

  Ok(followers)
}

// Cursor connections

pub async fn followers_to_cursor_connection(
  user_login: &String,
  result: Result<Vec<Follower>, ClientError>,
) -> Result<CursorConnection<Follower>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let result = result?;
  let reference_from = |item: &Follower| item.following_at.to_rfc3339_opts(SecondsFormat::Millis, true);

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_reference = result.first().unwrap().following_at;
  let last_reference = result.last().unwrap().following_at;
  let (query, params) = query_followers_pages_previous_and_next(user_login, &first_reference, &last_reference);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(&db, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

// Queries

fn query_find_followers_by_login<'a>(
  login: &'a String,
  direction: Direction,
  limit: &'a i64,
  cursor: Option<String>,
) -> (String, Vec<QueryParam<'a>>) {
  let mut query = sql::Select::new()
    .select("u.*")
    .select("uf.created_at AS following_at")
    .from("users_following uf")
    .left_join("users u ON u.login = uf.user_login")
    .left_join("organizations o ON o.login = uf.user_login")
    .where_clause("uf.following_login = $1")
    .limit("$2");

  let params: Vec<QueryParam> = vec![login, limit];

  let reference = if let Some(cursor) = cursor
    && let Ok(refe) = cursor_to_reference(cursor)
  {
    Some(refe)
  } else {
    None
  };
  match direction {
    Direction::Backward => {
      query = query.order_by("uf.created_at DESC");

      if let Some(reference) = reference {
        query = query.where_and(format!("uf.created_at < '{reference}'::timestamptz").as_str())
      }
    }
    Direction::Forward => {
      query = query.order_by("uf.created_at ASC");

      if let Some(reference) = reference {
        query = query.where_and(format!("uf.created_at > '{reference}'::timestamptz").as_str())
      }
    }
  };

  let query = sql::Select::new()
    .select("*")
    .from(format!("({query})").as_str())
    .order_by("following_at ASC")
    .as_string();

  (query, params)
}

fn query_followers_pages_previous_and_next<'a>(
  login: &'a String,
  first_reference: &'a DateTime<Utc>,
  last_reference: &'a DateTime<Utc>,
) -> (String, Vec<QueryParam<'a>>) {
  let base_query = sql::Select::new()
    .from("users_following uf")
    .left_join("users u ON u.login = uf.user_login")
    .left_join("organizations o ON o.login = uf.user_login")
    .where_clause("uf.following_login = $1::varchar")
    .limit("1");
  let query_prev = base_query
    .clone()
    .select("coalesce(u.login, o.login) login, 'previous' AS page")
    .where_and("uf.created_at < $2::timestamptz")
    .order_by("uf.created_at DESC");
  let query_next = base_query
    .clone()
    .select("coalesce(u.login, o.login) login, 'next' AS page")
    .where_and("uf.created_at > $3::timestamptz")
    .order_by("uf.created_at ASC");

  let query = query_prev.union(query_next).as_string();

  let params: Vec<QueryParam> = vec![login, first_reference, last_reference];

  (query, params)
}
