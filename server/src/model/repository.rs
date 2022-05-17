use crate::{
  lib::cursor_connection::{CursorConnection, Direction, PaginationArguments},
  model::{utils, QueryParam},
};
use serde::{Deserialize, Serialize};
use sql_query_builder::SelectBuilder;
use tokio_postgres::{Client, Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Language {
  pub color: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct License {
  pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
  pub description: Option<String>,
  pub fork_count: i32,
  pub id: i32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub license_name: Option<String>,
  pub name: String,
  pub owner_login: String,
  pub owner_ref: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub primary_language: Option<String>,
  pub url: String,
}

impl From<Row> for Repository {
  fn from(row: Row) -> Self {
    Self {
      description: row.try_get("description").unwrap_or(None),
      fork_count: row.get("fork_count"),
      id: row.get("id"),
      license_name: row.try_get("license_name").unwrap_or(None),
      name: row.get("name"),
      owner_login: row.get("owner_login"),
      owner_ref: row.get("owner_ref"),
      primary_language: row.try_get("primary_language").unwrap_or(None),
      url: row.get("url"),
    }
  }
}

// Finds

pub async fn find_repositories_by_owner_login(
  db_client: &Client,
  owner_login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<Repository>, ClientError> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let repository_id = utils::parse_cursor(cursor);
  let (query, params) = query_find_repositories_by_owner_login(owner_login, &repository_id, &direction, &limit);
  let result = db_client.query(query.as_str(), &params[..]).await?;
  let repositories = result.into_iter().map(|row| Repository::from(row)).collect::<Vec<_>>();

  Ok(repositories)
}

// Cursor connections

pub async fn repositories_to_cursor_connection(
  db_client: &Client,
  owner_login: &String,
  result: Result<Vec<Repository>, ClientError>,
) -> Result<CursorConnection<Repository>, ClientError> {
  let result = result?;
  let reference_from = |item: &Repository| item.id.to_string();

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_item_id = result.first().unwrap().id;
  let last_item_id = result.last().unwrap().id;
  let (query, params) = query_pages_previous_and_next(owner_login, &first_item_id, &last_item_id);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(db_client, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

// Queries

fn query_find_repositories_by_owner_login<'a>(
  owner_login: &'a String,
  repository_id: &'a Option<i32>,
  direction: &'a Direction,
  limit: &'a i64,
) -> (String, Vec<QueryParam<'a>>) {
  let mut select_base = SelectBuilder::new()
    .select("*")
    .from("repositories")
    .where_clause("owner_login = $1")
    .limit("$2");

  let mut params: Vec<QueryParam> = vec![owner_login, limit];

  let query = match direction {
    Direction::Forward => {
      if let Some(id) = repository_id {
        select_base = select_base.and("id > $3 /* last_id */");
        params.push(id);
      }

      select_base.order_by("id asc").as_string()
    }
    Direction::Backward => {
      if let Some(id) = repository_id {
        select_base = select_base.and("id < $3 /* first_id */");
        params.push(id);
      }

      select_base = select_base.order_by("id desc");

      SelectBuilder::new()
        .with("repositories_reverse", select_base)
        .select("*")
        .from("repositories_reverse")
        .order_by("id asc")
        .as_string()
    }
  };

  (query, params)
}

fn query_pages_previous_and_next<'a>(
  owner_login: &'a String,
  first_item_id: &'a i32,
  last_item_id: &'a i32,
) -> (String, Vec<QueryParam<'a>>) {
  let select_base = SelectBuilder::new()
    .from("repositories")
    .where_clause("owner_login = $1")
    .limit("1");

  let select_previous = select_base
    .clone()
    .select("'previous' as page")
    .and("id < $2 /* first_id */");
  let select_next = select_base
    .clone()
    .select("'next' as page")
    .and("id > $3 /* last_id */");

  let query = select_previous.union(select_next).as_string();
  let params: Vec<QueryParam> = vec![owner_login, first_item_id, last_item_id];

  (query, params)
}
