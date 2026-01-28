use crate::infrastructure::database::{
  self, QueryParam,
  cursor_connection::{CursorConnection, Direction, PaginationArguments},
  user::User,
  utils,
};
use serde::{Deserialize, Serialize};
use sql_query_builder as sql;
use tokio_postgres::{Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub id: i64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub location: Option<String>,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website_url: Option<String>,
  pub profile_type: utils::ProfileType,
}

impl From<Row> for Organization {
  fn from(row: Row) -> Self {
    Self {
      avatar_url: row.get("avatar_url"),
      description: row.try_get("description").unwrap_or(None),
      id: row.get("organization_id"),
      location: row.try_get("location").unwrap_or(None),
      login: row.get("login"),
      name: row.try_get("name").unwrap_or(None),
      url: row.get("url"),
      website_url: row.try_get("website_url").unwrap_or(None),
      profile_type: utils::ProfileType::Organization,
    }
  }
}

// Finds

pub async fn find_organization_by_login(login: &String) -> Result<Option<Organization>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let query = sql::Select::new()
    .select("*")
    .from("organizations")
    .where_clause("login = $1")
    .as_string();

  let params: Vec<QueryParam> = vec![&login];
  let result = db.query_opt(query.as_str(), &params[..]).await?;
  match result {
    Some(row) => {
      let organization = Organization::from(row);
      Ok(Some(organization))
    }
    None => Ok(None),
  }
}

pub async fn find_people_by_login(
  organization_login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<User>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let user_id = utils::parse_cursor(cursor);
  let (query, params) = query_find_people_by_login(organization_login, &user_id, &direction, &limit);
  let result = db.query(&query, &params[..]).await?;
  let people = result.into_iter().map(|row| User::from(row)).collect::<Vec<_>>();

  Ok(people)
}

// Cursor connections

pub async fn organizations_users_to_cursor_connection(
  owner_login: &String,
  result: Result<Vec<User>, ClientError>,
) -> Result<CursorConnection<User>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let result = result?;
  let reference_from = |item: &User| item.id.to_string();

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_item_id = result.first().unwrap().id;
  let last_item_id = result.last().unwrap().id;
  let (query, params) = query_pages_previous_and_next(owner_login, &first_item_id, &last_item_id);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(&db, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

// Queries

fn query_find_people_by_login<'a>(
  organization_login: &'a String,
  user_id: &'a Option<i64>,
  direction: &'a Direction,
  limit: &'a i64,
) -> (String, Vec<QueryParam<'a>>) {
  let mut select_people = sql::Select::new()
    .select("u.*, uo.created_at as joined_at")
    .from("organizations org")
    .inner_join("users_organizations uo on uo.organization_login = org.login")
    .inner_join("users u on u.login = uo.user_login")
    .where_clause("org.login = $1")
    .order_by("u.user_id asc");

  let mut params: Vec<QueryParam> = vec![organization_login];

  let query = match direction {
    Direction::Backward => {
      let mut select_people_reverse = sql::Select::new()
        .select("*")
        .from("people")
        .order_by("user_id desc")
        .limit("$2");

      params.push(limit);

      if let Some(user_id) = user_id {
        select_people_reverse = select_people_reverse.where_clause("user_id < $3");
        params.push(user_id);
      }

      sql::Select::new()
        .with("people", select_people)
        .with("people_reverse", select_people_reverse)
        .select("*")
        .from("people_reverse")
        .order_by("user_id asc")
        .as_string()
    }
    Direction::Forward => {
      select_people = select_people.limit("$2");
      params.push(limit);

      if let Some(user_id) = user_id {
        select_people = select_people.where_clause("u.user_id > $3");
        params.push(user_id);
      }

      select_people.as_string()
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
    .from("organizations o")
    .inner_join("users_organizations uo on uo.organization_login = o.login")
    .inner_join("users u on u.login = uo.user_login")
    .where_clause("o.login = $1")
    .order_by("u.user_id ASC")
    .limit("1");

  let select_previous = select_base
    .clone()
    .select("'previous' as page")
    .where_clause("u.user_id < $2 /* first_id */");
  let select_next = select_base
    .clone()
    .select("'next' as page")
    .where_clause("u.user_id > $3 /* last_id */");
  let query = select_previous.union(select_next).as_string();
  let params: Vec<QueryParam> = vec![owner_login, first_item_id, last_item_id];

  (query, params)
}
