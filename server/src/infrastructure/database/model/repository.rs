use crate::infrastructure::database::{
  self, QueryParam,
  cursor_connection::{CursorConnection, Direction, PaginationArguments},
  model::license::{License, find_licenses_by_repository_ids},
  utils,
};
use serde::{Deserialize, Serialize};
use sql_query_builder as sql;
use tokio_postgres::{Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Language {
  pub color: String,
  pub name: String,
}

impl From<Row> for Language {
  fn from(row: Row) -> Self {
    Self {
      color: row.get("language_color"),
      name: row.get("language_name"),
    }
  }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Repository {
  pub description: Option<String>,
  pub fork_count: i32,
  pub star_count: i32,
  pub repository_id: i64,
  pub licenses: Vec<License>,
  pub name: String,
  pub owner_login: String,
  pub owner_ref: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub primary_language: Option<Language>,
  pub url: String,
}

impl From<Row> for Repository {
  fn from(row: Row) -> Self {
    Self {
      description: row.try_get("description").unwrap_or(None),
      fork_count: row.get("fork_count"),
      star_count: row.get("star_count"),
      repository_id: row.get("repository_id"),
      licenses: vec![],
      name: row.get("name"),
      owner_login: row.get("owner_login"),
      owner_ref: row.get("owner_ref"),
      url: row.get("url"),
      primary_language: {
        if row.try_get::<_, &str>("primary_language").is_ok() {
          Some(Language::from(row.clone()))
        } else {
          None
        }
      },
    }
  }
}

// Finds

pub async fn find_repositories_by_owner_login(
  owner_login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<Repository>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let repository_id = utils::parse_cursor(cursor);

  let (query, params) = query_find_repositories_by_owner_login(owner_login, &repository_id, &direction, &limit);
  let result = db.query(&query, &params[..]).await?;
  let repositories = result.into_iter().map(|row| Repository::from(row)).collect::<Vec<_>>();

  let repository_ids = repositories.iter().map(|repo| repo.repository_id).collect::<Vec<_>>();
  let mut licenses = find_licenses_by_repository_ids(&repository_ids).await?;
  let repositories = repositories
    .into_iter()
    .map(|mut repo| {
      if let Some(list) = licenses.remove(&repo.repository_id) {
        repo.licenses = list;
      }
      repo
    })
    .collect::<Vec<_>>();

  Ok(repositories)
}

// Cursor connections

pub async fn repositories_to_cursor_connection(
  owner_login: &String,
  result: Result<Vec<Repository>, ClientError>,
) -> Result<CursorConnection<Repository>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let result = result?;
  let reference_from = |item: &Repository| item.repository_id.to_string();

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_item_id = result.first().unwrap().repository_id;
  let last_item_id = result.last().unwrap().repository_id;
  let (query, params) = query_pages_previous_and_next(owner_login, &first_item_id, &last_item_id);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(&db, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

// Queries

fn query_find_repositories_by_owner_login<'a>(
  owner_login: &'a String,
  repository_id: &'a Option<i64>,
  direction: &'a Direction,
  limit: &'a i64,
) -> (String, Vec<QueryParam<'a>>) {
  let mut select_base = sql::Select::new()
    .select("repository_id, name, owner_login, description, fork_count, star_count, owner_ref, primary_language, url, created_at")
    .select("l.*")
    .from("repositories r")
    .left_join("languages l on l.language_name = r.primary_language")
    .where_clause("r.owner_login = $1")
    .limit("$2");

  let mut params: Vec<QueryParam> = vec![owner_login, limit];

  let query = match direction {
    Direction::Forward => {
      if let Some(id) = repository_id {
        select_base = select_base.where_clause("repository_id > $3 /* last_id */");
        params.push(id);
      }

      select_base.order_by("repository_id asc").as_string()
    }
    Direction::Backward => {
      if let Some(id) = repository_id {
        select_base = select_base.where_clause("repository_id < $3 /* first_id */");
        params.push(id);
      }

      select_base = select_base.order_by("repository_id desc");

      sql::Select::new()
        .with("repositories_reverse", select_base)
        .select("*")
        .from("repositories_reverse")
        .order_by("repository_id asc")
        .as_string()
    }
  };

  (query, params)
}

fn query_pages_previous_and_next<'a>(
  owner_login: &'a String,
  first_item_id: &'a i64,
  last_item_id: &'a i64,
) -> (String, Vec<QueryParam<'a>>) {
  let select_base = sql::Select::new()
    .from("repositories")
    .where_clause("owner_login = $1")
    .limit("1");

  let select_previous = select_base
    .clone()
    .select("'previous' as page")
    .where_clause("repository_id < $2 /* first_id */");
  let select_next = select_base
    .clone()
    .select("'next' as page")
    .where_clause("repository_id > $3 /* last_id */");

  let query = select_previous.union(select_next).as_string();
  let params: Vec<QueryParam> = vec![owner_login, first_item_id, last_item_id];

  (query, params)
}
