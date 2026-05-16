use crate::infrastructure::database::{
  self, QueryParam,
  cursor_connection::{CursorConnection, Direction, PaginationArguments, cursor_to_reference},
  organization::Organization,
  user::User,
  utils,
};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use sql_query_builder as sql;
use tokio_postgres::{Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(tag = "type")]
pub enum Profile {
  User(User),
  Organization(Organization),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Following {
  #[serde(flatten)]
  pub profile: Profile,
  pub followed_since: DateTime<Utc>,
}
impl From<Row> for Following {
  fn from(row: Row) -> Self {
    let followed_since = row.get("followed_since");
    let profile = match (
      row.try_get::<_, i64>("user_id"),
      row.try_get::<_, i64>("organization_id"),
    ) {
      (Err(_), Err(_)) => panic!("profile not found"),
      (Ok(_), _) => Profile::User(User::from(row)),
      (_, Ok(_)) => Profile::Organization(Organization::from(row)),
    };

    Self {
      followed_since,
      profile,
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Follower {
  pub user: User,
  pub follows_since: DateTime<Utc>,
}
impl From<Row> for Follower {
  fn from(row: Row) -> Self {
    Self {
      follows_since: row.get("follows_since"),
      user: User::from(row),
    }
  }
}

// Finds

pub async fn find_following_by_login(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<Following>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let (query, params) = query_find_following_by_login(login, direction, &limit, cursor);
  let result = db.query(&query, &params[..]).await?;
  let followeds = result.into_iter().map(|row| Following::from(row)).collect::<Vec<_>>();

  Ok(followeds)
}

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

pub async fn following_to_cursor_connection(
  user_login: &String,
  result: Result<Vec<Following>, ClientError>,
) -> Result<CursorConnection<Following>, ClientError> {
  let result = result?;
  let db = database::get_connection().await.get().await.unwrap();
  let reference_from = |item: &Following| item.followed_since.to_rfc3339_opts(SecondsFormat::Millis, true);

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_reference = result.first().unwrap().followed_since;
  let last_reference = result.last().unwrap().followed_since;
  let (query, params) = query_following_pages_previous_and_next(user_login, &first_reference, &last_reference);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(&db, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

pub async fn followers_to_cursor_connection(
  user_login: &String,
  result: Result<Vec<Follower>, ClientError>,
) -> Result<CursorConnection<Follower>, ClientError> {
  let result = result?;
  let db = database::get_connection().await.get().await.unwrap();
  let reference_from = |item: &Follower| item.follows_since.to_rfc3339_opts(SecondsFormat::Millis, true);

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_reference = result.first().unwrap().follows_since;
  let last_reference = result.last().unwrap().follows_since;
  let (query, params) = query_followers_pages_previous_and_next(user_login, &first_reference, &last_reference);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(&db, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

// Queries

fn query_find_following_by_login<'a>(
  login: &'a String,
  direction: Direction,
  limit: &'a i64,
  cursor: Option<String>,
) -> (String, Vec<QueryParam<'a>>) {
  let mut query = sql::Select::new()
    .select("coalesce(u.avatar_url, o.avatar_url) avatar_url")
    .select("coalesce(u.email, o.email) email")
    .select("coalesce(u.location, o.location) location")
    .select("coalesce(u.login, o.login) login")
    .select("coalesce(u.name, o.name) name")
    .select("coalesce(u.url, o.url) url")
    .select("coalesce(u.website_url, o.website_url) website_url")
    .select("o.description")
    .select("o.organization_id")
    .select("u.bio")
    .select("u.company")
    .select("u.user_id")
    .select("uf.created_at AS followed_since")
    .from("users_following uf")
    .left_join("users u ON u.login = uf.following_login")
    .left_join("organizations o ON o.login = uf.following_login")
    .where_clause("uf.user_login = $1")
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
    .order_by("followed_since ASC")
    .as_string();

  (query, params)
}

fn query_find_followers_by_login<'a>(
  login: &'a String,
  direction: Direction,
  limit: &'a i64,
  cursor: Option<String>,
) -> (String, Vec<QueryParam<'a>>) {
  let mut query = sql::Select::new()
    .select("coalesce(u.avatar_url, o.avatar_url) avatar_url")
    .select("coalesce(u.email, o.email) email")
    .select("coalesce(u.location, o.location) location")
    .select("coalesce(u.login, o.login) login")
    .select("coalesce(u.name, o.name) name")
    .select("coalesce(u.url, o.url) url")
    .select("coalesce(u.website_url, o.website_url) website_url")
    .select("o.description")
    .select("o.organization_id")
    .select("u.bio")
    .select("u.company")
    .select("u.user_id")
    .select("uf.created_at AS follows_since")
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
    .order_by("follows_since ASC")
    .as_string();

  (query, params)
}

fn query_following_pages_previous_and_next<'a>(
  login: &'a String,
  first_reference: &'a DateTime<Utc>,
  last_reference: &'a DateTime<Utc>,
) -> (String, Vec<QueryParam<'a>>) {
  let base_query = sql::Select::new()
    .from("users_following uf")
    .left_join("users u ON u.login = uf.following_login")
    .left_join("organizations o ON o.login = uf.following_login")
    .where_clause("uf.user_login = $1::varchar")
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
