use crate::http::AppState;
use crate::model::{organization::Organization, repository::Repository, user::User, utils};
use crate::setup::db::db_connection_poll;
use actix_web::dev::ServiceResponse;
use actix_web::Scope;
use actix_web::{test, web, App};
use deadpool_postgres;
use rand;
use tokio_postgres::{Client, Error as ClientError};

pub enum HttpMethod {
  Get,
}

pub async fn make_request(method: HttpMethod, uri: &str, scope: Scope) -> ServiceResponse {
  let poll = setup().await;
  let app = test::init_service(App::new().app_data(web::Data::new(AppState { poll })).service(scope)).await;
  let req = match method {
    HttpMethod::Get => test::TestRequest::get().uri(uri).to_request(),
  };
  test::call_service(&app, req).await
}

pub async fn setup() -> deadpool_postgres::Pool {
  let poll = db_connection_poll().await.unwrap();
  let client = poll.get().await.unwrap();
  drop_collections(&client).await.unwrap();
  insert_mocked_data(&client).await.unwrap();

  poll
}

fn random_id() -> i32 {
  rand::random::<i32>()
}

async fn insert_mocked_data(db: &Client) -> Result<(), ClientError> {
  let organization_foo = Organization {
    avatar_url: "https://foo.com/avatar.jpg".to_owned(),
    description: None,
    id: random_id(),
    location: None,
    login: "organization_foo".to_owned(),
    name: None,
    url: "https://github.com/foo".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::Organization,
  };
  let organization_acme = Organization {
    avatar_url: "https://acme.com/avatar.jpg".to_owned(),
    description: None,
    id: random_id(),
    location: None,
    login: "organization_acme".to_owned(),
    name: None,
    url: "https://github.com/acme".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::Organization,
  };
  let organization_empty_org = Organization {
    avatar_url: "https://empty_org.com/avatar.jpg".to_owned(),
    description: None,
    id: random_id(),
    location: None,
    login: "empty_org".to_owned(),
    name: None,
    url: "https://github.com/empty_org".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::Organization,
  };

  let user_foo = User {
    avatar_url: "https://foo.com/avatar.jpg".to_owned(),
    bio: None,
    email: "foo@email.com".to_owned(),
    id: random_id(),
    login: "user_foo".to_owned(),
    name: None,
    url: "https://github.com/foo".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::User,
  };
  let user_bar = User {
    avatar_url: "https://bar.com/avatar.jpg".to_owned(),
    bio: None,
    email: "bar@email.com".to_owned(),
    id: random_id(),
    login: "user_bar".to_owned(),
    name: None,
    url: "https://github.com/bar".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::User,
  };
  let user_dee = User {
    avatar_url: "https://dee.com/avatar.jpg".to_owned(),
    bio: None,
    email: "dee@email.com".to_owned(),
    id: random_id(),
    login: "user_dee".to_owned(),
    name: None,
    url: "https://github.com/bar".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::User,
  };
  let user_empty_user = User {
    avatar_url: "https://empty_user.com/avatar.jpg".to_owned(),
    bio: None,
    email: "empty_user@email.com".to_owned(),
    id: random_id(),
    login: "empty_user".to_owned(),
    name: None,
    url: "https://github.com/empty_user".to_owned(),
    website_url: None,
    profile_type: utils::ProfileType::User,
  };

  let repository_tux = Repository {
    description: None,
    fork_count: 9,
    id: random_id(),
    license_name: None,
    name: "repository_tux".to_owned(),
    owner_login: "organization_acme".to_owned(),
    owner_ref: "organizations".to_owned(),
    primary_language: None,
    url: "https://github.com/user_bar/repository_tux".to_owned(),
  };
  let repository_mar = Repository {
    description: None,
    fork_count: 12,
    id: random_id(),
    license_name: None,
    name: "repository_mar".to_owned(),
    owner_login: "organization_acme".to_owned(),
    owner_ref: "organizations".to_owned(),
    primary_language: None,
    url: "https://github.com/user_bar/repository_mar".to_owned(),
  };
  let repository_bar = Repository {
    description: None,
    fork_count: 2,
    id: random_id(),
    license_name: None,
    name: "repository_bar".to_owned(),
    owner_login: "user_bar".to_owned(),
    owner_ref: "users".to_owned(),
    primary_language: None,
    url: "https://github.com/user_bar/repository_bar".to_owned(),
  };
  let repository_dee = Repository {
    description: None,
    fork_count: 2,
    id: random_id(),
    license_name: None,
    name: "repository_dee".to_owned(),
    owner_login: "user_dee".to_owned(),
    owner_ref: "users".to_owned(),
    primary_language: None,
    url: "https://github.com/user_dee/repository_dee".to_owned(),
  };

  insert_organization(db, &organization_acme).await?;
  insert_organization(db, &organization_foo).await?;
  insert_organization(db, &organization_empty_org).await?;

  insert_repository(db, &repository_tux).await?;
  insert_repository(db, &repository_mar).await?;
  insert_repository(db, &repository_bar).await?;
  insert_repository(db, &repository_dee).await?;

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

async fn drop_collections(db: &Client) -> Result<(), ClientError> {
  db.execute(
    "TRUNCATE users_starred_repositories, languages, licenses, repositories, users_organizations, organizations, users_following, users",
    &[],
  )
  .await?;

  Ok(())
}

async fn insert_organization(db: &Client, document: &Organization) -> Result<u64, ClientError> {
  let statement = "\
    INSERT INTO organizations (avatar_url,  description, location,    login,       name,        url,         website_url) \
                       VALUES ($1::VARCHAR, $2::VARCHAR, $3::VARCHAR, $4::VARCHAR, $5::VARCHAR, $6::VARCHAR, $7::VARCHAR)\
  ";
  db.execute(
    statement,
    &[
      &document.avatar_url,
      &document.description,
      &document.location,
      &document.login,
      &document.name,
      &document.url,
      &document.website_url,
    ],
  )
  .await
}

async fn insert_user_organization(db: &Client, user: &User, organization: &Organization) -> Result<u64, ClientError> {
  let statement = "\
    INSERT INTO users_organizations (user_login,  organization_login) \
                             VALUES ($1::VARCHAR, $2::VARCHAR)\
  ";
  db.execute(statement, &[&user.login, &organization.login]).await
}

async fn insert_repository(db: &Client, document: &Repository) -> Result<u64, ClientError> {
  let statement = "\
    INSERT INTO repositories (description, fork_count, license_name, name,        owner_login, owner_ref,   primary_language, url) \
                      VALUES ($1::VARCHAR, $2::INT4,   $3::VARCHAR,  $4::VARCHAR, $5::VARCHAR, $6::VARCHAR, $7::VARCHAR,      $8::VARCHAR)\
  ";
  db.execute(
    statement,
    &[
      &document.description,
      &document.fork_count,
      &document.license_name,
      &document.name,
      &document.owner_login,
      &document.owner_ref,
      &document.primary_language,
      &document.url,
    ],
  )
  .await
}

async fn insert_user(db: &Client, document: &User) -> Result<u64, ClientError> {
  let statement = "\
    INSERT INTO users (avatar_url,  bio,         email,       login,       name,        url,         website_url) \
               VALUES ($1::VARCHAR, $2::VARCHAR, $3::VARCHAR, $4::VARCHAR, $5::VARCHAR, $6::VARCHAR, $7::VARCHAR)\
  ";
  db.execute(
    statement,
    &[
      &document.avatar_url,
      &document.bio,
      &document.email,
      &document.login,
      &document.name,
      &document.url,
      &document.website_url,
    ],
  )
  .await
}

async fn insert_user_following(db: &Client, user: &User, following: &User) -> Result<u64, ClientError> {
  let statement = "\
    INSERT INTO users_following (user_login,  following_login) \
                         VALUES ($1::VARCHAR, $2::VARCHAR)\
  ";
  db.execute(statement, &[&user.login, &following.login]).await
}

async fn insert_user_starred_repository(db: &Client, user: &User, repository: &Repository) -> Result<u64, ClientError> {
  let statement = "\
    INSERT INTO users_starred_repositories (user_login,  repository_name) \
                                    VALUES ($1::VARCHAR, $2::VARCHAR)\
  ";
  db.execute(statement, &[&user.login, &repository.name]).await
}
