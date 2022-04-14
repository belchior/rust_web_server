use crate::{
  lib::{
    cursor_connection::{CursorConnection, Direction, PaginationArguments},
    sql_query_builder::SelectBuilder,
  },
  model::{organization::Organization, repository::Repository, utils, QueryParam},
};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bio: Option<String>,
  pub email: String,
  pub id: i32,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website_url: Option<String>,
  pub profile_type: utils::ProfileType,
}

impl From<Row> for User {
  fn from(row: Row) -> Self {
    Self {
      avatar_url: row.get("avatar_url"),
      bio: row.try_get("bio").unwrap_or(None),
      email: row.get("email"),
      id: row.get("id"),
      login: row.get("login"),
      name: row.try_get("name").unwrap_or(None),
      url: row.get("url"),
      website_url: row.try_get("website_url").unwrap_or(None),
      profile_type: utils::ProfileType::User,
    }
  }
}

// Finds

pub async fn find_followers_by_login(
  db_client: &Client,
  user_login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<User>, ClientError> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let follower_id = utils::parse_cursor(cursor);
  let (query, params) = query_find_followers_by_login(user_login, &follower_id, &direction, &limit);
  let result = db_client.query(query.as_str(), &params[..]).await?;
  let users = result.into_iter().map(|row| User::from(row)).collect::<Vec<_>>();

  Ok(users)
}

pub async fn find_followed_by_login(
  db_client: &Client,
  user_login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<User>, ClientError> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let follower_id = utils::parse_cursor(cursor);
  let (query, params) = query_find_followed_by_login(user_login, &follower_id, &direction, &limit);
  let result = db_client.query(query.as_str(), &params[..]).await?;
  let users = result.into_iter().map(|row| User::from(row)).collect::<Vec<_>>();

  Ok(users)
}

pub async fn find_organizations_by_user_login(
  db_client: &Client,
  user_login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<Organization>, ClientError> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let org_id = utils::parse_cursor(cursor);
  let (query, params) = query_find_organizations_by_user_login(user_login, &org_id, &direction, &limit);
  let result = db_client.query(query.as_str(), &params[..]).await?;
  let organizations = result
    .into_iter()
    .map(|row| Organization::from(row))
    .collect::<Vec<_>>();

  Ok(organizations)
}

pub async fn find_starred_repositories_by_user_login(
  db_client: &Client,
  user_login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<Repository>, ClientError> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let repo_id = utils::parse_cursor(cursor);
  let (query, params) = query_find_starred_repositories_by_user_login(user_login, &repo_id, &direction, &limit);
  let result = db_client.query(query.as_str(), &params[..]).await?;
  let repositories = result.into_iter().map(|row| Repository::from(row)).collect::<Vec<_>>();

  Ok(repositories)
}

pub async fn find_user_by_login(db_client: &Client, login: &String) -> Result<Option<User>, ClientError> {
  let query = SelectBuilder::new()
    .select("*")
    .from("users")
    .where_clause("login = $1")
    .as_string();

  let params: Vec<QueryParam> = vec![&login];
  let result = db_client.query_opt(query.as_str(), &params[..]).await?;
  match result {
    Some(row) => {
      let user = User::from(row);
      Ok(Some(user))
    }
    None => Ok(None),
  }
}

// Cursor connections

pub async fn followers_to_cursor_connection(
  db_client: &Client,
  user_login: &String,
  result: Result<Vec<User>, ClientError>,
) -> Result<CursorConnection<User>, ClientError> {
  let result = result?;
  let reference_from = |item: &User| item.id.to_string();

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_item_id = result.first().unwrap().id;
  let last_item_id = result.last().unwrap().id;
  let (query, params) = query_followers_pages_previous_and_next(user_login, &first_item_id, &last_item_id);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(db_client, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

pub async fn followed_to_cursor_connection(
  db_client: &Client,
  user_login: &String,
  result: Result<Vec<User>, ClientError>,
) -> Result<CursorConnection<User>, ClientError> {
  let result = result?;
  let reference_from = |item: &User| item.id.to_string();

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_item_id = result.first().unwrap().id;
  let last_item_id = result.last().unwrap().id;
  let (query, params) = query_followed_pages_previous_and_next(user_login, &first_item_id, &last_item_id);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(db_client, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

pub async fn users_organizations_to_cursor_connection(
  db_client: &Client,
  owner_login: &String,
  result: Result<Vec<Organization>, ClientError>,
) -> Result<CursorConnection<Organization>, ClientError> {
  let result = result?;
  let reference_from = |item: &Organization| item.id.to_string();

  if result.len() == 0 {
    let items = CursorConnection::new(result, reference_from, false, false);
    return Ok(items);
  }

  let first_item_id = result.first().unwrap().id;
  let last_item_id = result.last().unwrap().id;
  let (query, params) = query_organizations_pages_previous_and_next(owner_login, &first_item_id, &last_item_id);
  let (has_previous_page, has_next_page) = utils::pages_previous_and_next(db_client, query, params).await?;
  let items = CursorConnection::new(result, reference_from, has_previous_page, has_next_page);

  Ok(items)
}

// Queries

fn query_find_followers_by_login<'a>(
  user_login: &'a String,
  follower_id: &'a Option<i32>,
  direction: &'a Direction,
  limit: &'a i64,
) -> (String, Vec<QueryParam<'a>>) {
  let mut select_followers = SelectBuilder::new()
    .select("u.*")
    .from("users u")
    .inner_join("users_following uf", "uf.user_login = u.login")
    .where_clause("uf.following_login = $1")
    .order_by("u.id asc");

  let mut params: Vec<QueryParam> = vec![user_login];

  let query = match direction {
    Direction::Backward => {
      let mut select_followers_reverse = SelectBuilder::new()
        .select("*")
        .from("followers")
        .order_by("id desc")
        .limit("$2");

      params.push(limit);

      if let Some(follower_id) = follower_id {
        select_followers_reverse = select_followers_reverse.and("id < $3::int");
        params.push(follower_id);
      }

      SelectBuilder::new()
        .with("followers", select_followers)
        .with("followers_reverse", select_followers_reverse)
        .select("*")
        .from("followers_reverse")
        .order_by("id asc")
        .as_string()
    }
    Direction::Forward => {
      select_followers = select_followers.limit("$2");
      params.push(limit);

      if let Some(follower_id) = follower_id {
        select_followers = select_followers.and("u.id > $3");
        params.push(follower_id);
      }

      select_followers.as_string()
    }
  };

  (query, params)
}

fn query_find_followed_by_login<'a>(
  user_login: &'a String,
  followed_id: &'a Option<i32>,
  direction: &'a Direction,
  limit: &'a i64,
) -> (String, Vec<QueryParam<'a>>) {
  let mut select_following = SelectBuilder::new()
    .select("u.*")
    .from("users u")
    .inner_join("users_following uf", "uf.following_login = u.login")
    .where_clause("uf.user_login = $1")
    .order_by("u.id asc");

  let mut params: Vec<QueryParam> = vec![user_login];

  let query = match direction {
    Direction::Backward => {
      let mut select_following_reverse = SelectBuilder::new()
        .select("*")
        .from("following")
        .order_by("id desc")
        .limit("$2");

      params.push(limit);

      if let Some(followed_id) = followed_id {
        select_following_reverse = select_following_reverse.and("id < $3::int");
        params.push(followed_id);
      }

      SelectBuilder::new()
        .with("following", select_following)
        .with("following_reverse", select_following_reverse)
        .select("*")
        .from("following_reverse")
        .order_by("id asc")
        .as_string()
    }
    Direction::Forward => {
      select_following = select_following.limit("$2");
      params.push(limit);

      if let Some(followed_id) = followed_id {
        select_following = select_following.and("u.id > $3");
        params.push(followed_id);
      }

      select_following.as_string()
    }
  };

  (query, params)
}

fn query_find_organizations_by_user_login<'a>(
  user_login: &'a String,
  org_id: &'a Option<i32>,
  direction: &'a Direction,
  limit: &'a i64,
) -> (String, Vec<QueryParam<'a>>) {
  let mut select_org = SelectBuilder::new()
    .select("o.*, uo.created_at as joined_at")
    .from("users u")
    .inner_join("users_organizations uo", "uo.user_login = u.login")
    .inner_join("organizations o", "o.login = uo.organization_login")
    .where_clause("u.login = $1")
    .order_by("o.id asc");

  let mut params: Vec<QueryParam> = vec![user_login];

  let query = match direction {
    Direction::Backward => {
      let mut select_org_reverse = SelectBuilder::new()
        .select("*")
        .from("orgs")
        .order_by("id desc")
        .limit("$2");

      params.push(limit);

      if let Some(org_id) = org_id {
        select_org_reverse = select_org_reverse.and("id < $3::int");
        params.push(org_id);
      }

      SelectBuilder::new()
        .with("orgs", select_org)
        .with("orgs_reverse", select_org_reverse)
        .select("*")
        .from("orgs_reverse")
        .order_by("id asc")
        .as_string()
    }
    Direction::Forward => {
      select_org = select_org.limit("$2");
      params.push(limit);

      if let Some(org_id) = org_id {
        select_org = select_org.and("o.id > $3");
        params.push(org_id);
      }

      select_org.as_string()
    }
  };

  (query, params)
}

fn query_followed_pages_previous_and_next<'a>(
  user_login: &'a String,
  first_item_id: &'a i32,
  last_item_id: &'a i32,
) -> (String, Vec<QueryParam<'a>>) {
  let select_base = SelectBuilder::new()
    .from("users u")
    .inner_join("users_following uf", "uf.following_login = u.login")
    .where_clause("uf.user_login = $1")
    .order_by("u.id asc")
    .limit("1");

  let select_previous = select_base
    .clone()
    .select("'previous' as page")
    .and("u.id < $2 /* first_id */");
  let select_next = select_base
    .clone()
    .select("'next' as page")
    .and("u.id > $3 /* last_id */");
  let query = select_previous.union(select_next).as_string();
  let params: Vec<QueryParam> = vec![user_login, first_item_id, last_item_id];

  (query, params)
}

fn query_followers_pages_previous_and_next<'a>(
  user_login: &'a String,
  first_item_id: &'a i32,
  last_item_id: &'a i32,
) -> (String, Vec<QueryParam<'a>>) {
  let select_base = SelectBuilder::new()
    .from("users u")
    .inner_join("users_following uf", "uf.user_login = u.login")
    .where_clause("uf.following_login = $1")
    .order_by("u.id asc")
    .limit("1");

  let select_previous = select_base
    .clone()
    .select("'previous' as page")
    .and("u.id < $2 /* first_id */");
  let select_next = select_base
    .clone()
    .select("'next' as page")
    .and("u.id > $3 /* last_id */");
  let query = select_previous.union(select_next).as_string();
  let params: Vec<QueryParam> = vec![user_login, first_item_id, last_item_id];

  (query, params)
}

fn query_organizations_pages_previous_and_next<'a>(
  owner_login: &'a String,
  first_item_id: &'a i32,
  last_item_id: &'a i32,
) -> (String, Vec<QueryParam<'a>>) {
  let select_base = SelectBuilder::new()
    .from("users u")
    .inner_join("users_organizations uo", "uo.organization_login = u.login")
    .inner_join("organizations o", "o.login = uo.organization_login")
    .where_clause("u.login = $1")
    .order_by("o.id asc")
    .limit("1");

  let select_previous = select_base
    .clone()
    .select("'previous' as page")
    .and("u.id < $2 /* first_id */");
  let select_next = select_base
    .clone()
    .select("'next' as page")
    .and("u.id > $3 /* last_id */");
  let query = select_previous.union(select_next).as_string();
  let params: Vec<QueryParam> = vec![owner_login, first_item_id, last_item_id];

  (query, params)
}

fn query_find_starred_repositories_by_user_login<'a>(
  user_login: &'a String,
  repo_id: &'a Option<i32>,
  direction: &'a Direction,
  limit: &'a i64,
) -> (String, Vec<QueryParam<'a>>) {
  let mut select_repo = SelectBuilder::new()
    .select("r.*")
    .from("users u")
    .inner_join("users_starred_repositories usr", "usr.user_login = u.login")
    .inner_join("repositories r", "r.name = usr.repository_name")
    .where_clause("u.login = $1")
    .order_by("r.id asc");

  let mut params: Vec<QueryParam> = vec![user_login];

  let query = match direction {
    Direction::Backward => {
      let mut select_repo_reverse = SelectBuilder::new()
        .select("*")
        .from("repos")
        .order_by("id desc")
        .limit("$2");

      params.push(limit);

      if let Some(repo_id) = repo_id {
        select_repo_reverse = select_repo_reverse.and("id < $3::int");
        params.push(repo_id);
      }

      SelectBuilder::new()
        .with("repos", select_repo)
        .with("repos_reverse", select_repo_reverse)
        .select("*")
        .from("repos_reverse")
        .order_by("id asc")
        .as_string()
    }
    Direction::Forward => {
      select_repo = select_repo.limit("$2");
      params.push(limit);

      if let Some(repo_id) = repo_id {
        select_repo = select_repo.and("r.id > $3");
        params.push(repo_id);
      }

      select_repo.as_string()
    }
  };

  (query, params)
}
