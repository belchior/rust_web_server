use crate::infrastructure::database::{
  self, DBConnection, QueryParam, organization::Organization, repository::Repository, user::User, utils,
};
use actix_web::{App, Scope, dev::ServiceResponse, test};
use rand;
use sql_query_builder as sql;
use tokio_postgres::Error as ClientError;

pub enum HttpMethod {
  Get,
}

pub async fn make_request(method: HttpMethod, uri: &str, scope: Scope, suffix: &str) -> ServiceResponse {
  let _ = setup(suffix).await;
  let app = test::init_service(App::new().service(scope)).await;
  let req = match method {
    HttpMethod::Get => test::TestRequest::get().uri(uri).to_request(),
  };
  test::call_service(&app, req).await
}

pub async fn setup(suffix: &str) {
  let pool = database::get_connection().await;
  let db = pool.get().await.unwrap();
  insert_mocked_data(&db, suffix).await.unwrap();
}

pub fn random_i64() -> i64 {
  rand::random::<i64>()
}

pub fn random_suffix() -> String {
  rand::random::<u32>().to_string()
}

async fn insert_mocked_data(db: &DBConnection, suffix: &str) -> Result<(), ClientError> {
  let organization_foo = Organization {
    avatar_url: "https://foo.com/avatar.jpg".to_owned(),
    description: None,
    id: random_i64(),
    location: None,
    login: format!("organization_foo_{suffix}"),
    name: None,
    url: "https://github.com/foo".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::Organization,
  };
  let organization_acme = Organization {
    avatar_url: "https://acme.com/avatar.jpg".to_owned(),
    description: None,
    id: random_i64(),
    location: None,
    login: format!("organization_acme_{suffix}"),
    name: None,
    url: "https://github.com/acme".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::Organization,
  };
  let organization_empty_org = Organization {
    avatar_url: "https://empty_org.com/avatar.jpg".to_owned(),
    description: None,
    id: random_i64(),
    location: None,
    login: format!("empty_org_{suffix}"),
    name: None,
    url: "https://github.com/empty_org".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::Organization,
  };

  let user_foo = User {
    avatar_url: "https://foo.com/avatar.jpg".to_owned(),
    bio: None,
    email: "foo@email.com".to_owned(),
    id: random_i64(),
    login: format!("user_foo_{suffix}"),
    name: None,
    url: "https://github.com/foo".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::User,
  };
  let user_bar = User {
    avatar_url: "https://bar.com/avatar.jpg".to_owned(),
    bio: None,
    email: "bar@email.com".to_owned(),
    id: random_i64(),
    login: format!("user_bar_{suffix}"),
    name: None,
    url: "https://github.com/bar".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::User,
  };
  let user_dee = User {
    avatar_url: "https://dee.com/avatar.jpg".to_owned(),
    bio: None,
    email: "dee@email.com".to_owned(),
    id: random_i64(),
    login: format!("user_dee_{suffix}"),
    name: None,
    url: "https://github.com/bar".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::User,
  };
  let user_empty_user = User {
    avatar_url: "https://empty_user.com/avatar.jpg".to_owned(),
    bio: None,
    email: "empty_user@email.com".to_owned(),
    id: random_i64(),
    login: format!("empty_user_{suffix}"),
    name: None,
    url: "https://github.com/empty_user".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::User,
  };

  let mut repository_tux = Repository {
    description: None,
    fork_count: 9,
    id: -1,
    licenses: vec![],
    name: format!("repository_tux_{suffix}"),
    owner_login: format!("organization_acme_{suffix}"),
    owner_ref: "organizations".to_owned(),
    primary_language: None,
    url: "https://github.com/user_bar/repository_tux".to_owned(),
  };
  let mut repository_mar = Repository {
    description: None,
    fork_count: 12,
    id: -1,
    licenses: vec![],
    name: format!("repository_mar_{suffix}"),
    owner_login: format!("organization_acme_{suffix}"),
    owner_ref: "organizations".to_owned(),
    primary_language: None,
    url: "https://github.com/user_bar/repository_mar".to_owned(),
  };
  let mut repository_bar = Repository {
    description: None,
    fork_count: 2,
    id: -1,
    licenses: vec![],
    name: format!("repository_bar_{suffix}"),
    owner_login: format!("user_bar_{suffix}"),
    owner_ref: "users".to_owned(),
    primary_language: None,
    url: "https://github.com/user_bar/repository_bar".to_owned(),
  };
  let mut repository_dee = Repository {
    description: None,
    fork_count: 2,
    id: -1,
    licenses: vec![],
    name: format!("repository_dee_{suffix}"),
    owner_login: format!("user_dee_{suffix}"),
    owner_ref: "users".to_owned(),
    primary_language: None,
    url: "https://github.com/user_dee/repository_dee".to_owned(),
  };

  insert_organization(db, &organization_acme).await?;
  insert_organization(db, &organization_foo).await?;
  insert_organization(db, &organization_empty_org).await?;

  insert_repository(db, &mut repository_tux).await?;
  insert_repository(db, &mut repository_mar).await?;
  insert_repository(db, &mut repository_bar).await?;
  insert_repository(db, &mut repository_dee).await?;

  insert_user(db, &user_empty_user).await?;

  insert_user(db, &user_foo).await?;
  insert_user_organization(db, &user_foo, &organization_foo).await?;
  insert_user_organization(db, &user_foo, &organization_acme).await?;

  insert_user(db, &user_bar).await?;
  insert_user_following(db, &user_bar, &user_foo).await?;
  insert_user_starred_repository(db, &user_bar, &repository_tux).await?;
  insert_user_starred_repository(db, &user_bar, &repository_dee).await?;

  insert_user(db, &user_dee).await?;
  insert_user_organization(db, &user_dee, &organization_acme).await?;
  insert_user_following(db, &user_dee, &user_foo).await?;
  insert_user_following(db, &user_dee, &user_bar).await?;

  Ok(())
}

async fn insert_organization(db: &DBConnection, document: &Organization) -> Result<u64, ClientError> {
  let query = sql::Insert::new()
    .insert_into("organizations (avatar_url, description, location, login, name, url, website_url)")
    .values("($1::VARCHAR, $2::VARCHAR, $3::VARCHAR, $4::VARCHAR, $5::VARCHAR, $6::VARCHAR, $7::VARCHAR)")
    .as_string();

  let params: Vec<QueryParam> = vec![
    &document.avatar_url,
    &document.description,
    &document.location,
    &document.login,
    &document.name,
    &document.url,
    &document.website_url,
  ];

  db.execute(&query, &params).await
}

async fn insert_user_organization(
  db: &DBConnection,
  user: &User,
  organization: &Organization,
) -> Result<u64, ClientError> {
  let query = sql::Insert::new()
    .insert_into("users_organizations (user_login,  organization_login)")
    .values("($1::VARCHAR, $2::VARCHAR)")
    .as_string();

  let params: Vec<QueryParam> = vec![&user.login, &organization.login];

  db.execute(&query, &params).await
}

async fn insert_repository(db: &DBConnection, document: &mut Repository) -> Result<(), ClientError> {
  let query = sql::Insert::new()
    .insert_into("repositories (description, fork_count, name, owner_login, owner_ref, primary_language, url)")
    .values("($1::VARCHAR, $2::INT4, $3::VARCHAR, $4::VARCHAR, $5::VARCHAR, $6::VARCHAR, $7::VARCHAR)")
    .returning("repository_id")
    .as_string();

  let primary_language = if let Some(language) = &document.primary_language {
    Some(language.name.clone())
  } else {
    None
  };

  let params: Vec<QueryParam> = vec![
    &document.description,
    &document.fork_count,
    &document.name,
    &document.owner_login,
    &document.owner_ref,
    &primary_language,
    &document.url,
  ];

  let result = db.query_opt(&query, &params).await;

  match result {
    Err(err) => Err(err),
    Ok(None) => Ok(()),
    Ok(Some(row)) => {
      document.id = row.get("repository_id");
      Ok(())
    }
  }
}

async fn insert_user(db: &DBConnection, document: &User) -> Result<u64, ClientError> {
  let query = sql::Insert::new()
    .insert_into("users (avatar_url, bio, email, login, name, url, website_url)")
    .values("($1::VARCHAR, $2::VARCHAR, $3::VARCHAR, $4::VARCHAR,$5::VARCHAR, $6::VARCHAR, $7::VARCHAR)")
    .as_string();

  let params: Vec<QueryParam> = vec![
    &document.avatar_url,
    &document.bio,
    &document.email,
    &document.login,
    &document.name,
    &document.url,
    &document.website_url,
  ];
  db.execute(&query, &params).await
}

async fn insert_user_following(db: &DBConnection, user: &User, following: &User) -> Result<u64, ClientError> {
  let query = sql::Insert::new()
    .insert_into("users_following (user_login,  following_login)")
    .values("($1::VARCHAR, $2::VARCHAR)")
    .as_string();

  let params: Vec<QueryParam> = vec![&user.login, &following.login];

  db.execute(&query, &params).await
}

async fn insert_user_starred_repository(
  db: &DBConnection,
  user: &User,
  repository: &Repository,
) -> Result<u64, ClientError> {
  let query = sql::Insert::new()
    .insert_into("users_starred_repositories (user_login,  repository_id)")
    .values("($1::VARCHAR, $2::BIGINT)")
    .as_string();

  let params: Vec<QueryParam> = vec![&user.login, &repository.id];

  db.execute(&query, &params).await
}
