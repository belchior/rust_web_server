use crate::{http::AppState, setup::db::db_client_connection};
use actix_web::{dev::ServiceResponse, test, web, App, Scope};
use mongodb::{
  bson::{self, doc, oid::ObjectId},
  error::Error as ModelError,
  results::InsertOneResult,
  Database,
};

pub enum HttpMethod {
  Get,
}

pub async fn make_request(method: HttpMethod, uri: &str, scope: Scope, sufix: &str) -> ServiceResponse {
  let db = setup(sufix).await;
  let app = test::init_service(
    App::new()
      .app_data(web::Data::new(AppState { db: db.clone() }))
      .service(scope),
  )
  .await;

  let req = match method {
    HttpMethod::Get => test::TestRequest::get().uri(uri).to_request(),
  };

  test::call_service(&app, req).await
}

pub async fn setup(sufix: &str) -> Database {
  let db = db_client_connection().await.unwrap();
  insert_mocked_data(&db, sufix).await.unwrap();

  db
}

fn random_id() -> ObjectId {
  ObjectId::new()
}

pub fn random_sufix() -> String {
  ObjectId::new().to_string()
}

async fn insert_mocked_data(db: &Database, sufix: &str) -> Result<(), ModelError> {
  let organization_foo_id = random_id();
  let organization_acme_id = random_id();
  let user_foo_id = random_id();
  let user_bar_id = random_id();
  let user_dee_id = random_id();
  let repository_tux_id = random_id();
  let repository_mar_id = random_id();
  let repository_bar_id = random_id();
  let repository_dee_id = random_id();

  let organization_foo = doc! {
    "__typename": "Organization",
    "_id": organization_foo_id,
    "avatarUrl": "https://foo.com/avatar.jpg",
    "login": format!("organization_foo_{sufix}"),
    "people": vec![doc!{ "_id": user_foo_id, "ref": "users" }],
    "url": "https://github.com/foo",
  };
  let organization_acme = doc! {
    "__typename": "Organization",
    "_id": organization_acme_id,
    "avatarUrl": "https://acme.com/avatar.jpg",
    "login": format!("organization_acme_{sufix}"),
    "people": vec![
      doc!{ "_id": user_foo_id, "ref": "users" },
      doc!{ "_id": user_dee_id, "ref": "users" },
    ],
    "url": "https://github.com/acme",
  };
  let organization_empty_org = doc! {
    "__typename": "Organization",
    "_id": random_id(),
    "avatarUrl": "https://empty_org.com/avatar.jpg",
    "login": format!("empty_org_{sufix}"),
    "people": vec![] as Vec<bson::Document>,
    "url": "https://github.com/empty_org",
  };

  let user_foo = doc! {
    "__typename": "User",
    "_id": user_foo_id,
    "avatarUrl": "https://foo.com/avatar.jpg",
    "email": "foo@email.com",
    "followers": vec![
      doc! { "_id": user_bar_id },
      doc! { "_id": user_dee_id },
    ],
    "login": format!("user_foo_{sufix}"),
    "organizations": vec![
      doc! { "_id": organization_foo_id },
      doc! { "_id": organization_acme_id },
    ],
    "url":"https://github.com/foo",
  };
  let user_bar = doc! {
    "__typename": "User",
    "_id": user_bar_id,
    "avatarUrl": "https://bar.com/avatar.jpg",
    "email": "bar@email.com",
    "following": vec![
      doc! { "_id": user_foo_id }
    ],
    "followers": vec![
      doc! { "_id": user_dee_id },
    ],
    "login": format!("user_bar_{sufix}"),
    "starredRepositories": vec![
      doc! { "_id": repository_tux_id },
      doc! { "_id": repository_dee_id },
    ],
    "url":"https://github.com/bar",
  };
  let user_dee = doc! {
    "__typename": "User",
    "_id": user_dee_id,
    "avatarUrl": "https://dee.com/avatar.jpg",
    "email": "dee@email.com",
    "following": vec![
      doc! { "_id": user_foo_id },
      doc! { "_id": user_bar_id },
    ],
    "login": format!("user_dee_{sufix}"),
    "organizations": vec![doc! { "_id": organization_acme_id }],
    "url":"https://github.com/bar",
  };
  let user_empty_user = doc! {
    "__typename": "User",
    "_id": random_id(),
    "avatarUrl": "https://empty_user.com/avatar.jpg",
    "email": "empty_user@email.com",
    "login": format!("empty_user_{sufix}"),
    "url":"https://github.com/empty_user",
  };

  let repository_tux = doc! {
    "_id": repository_tux_id,
    "forkCount": 9.0,
    "name": format!("repository_tux_{sufix}"),
    "owner": { "_id": organization_acme_id, "ref": "organizations" }
  };
  let repository_mar = doc! {
    "_id": repository_mar_id,
    "forkCount": 12.0,
    "name": format!("repository_mar_{sufix}"),
    "owner": { "_id": organization_acme_id, "ref": "organizations" }
  };
  let repository_bar = doc! {
    "_id": repository_bar_id,
    "forkCount": 2.0,
    "name": format!("repository_bar_{sufix}"),
    "owner": { "_id": user_bar_id, "ref": "users" }
  };
  let repository_dee = doc! {
    "_id": repository_dee_id,
    "forkCount": 2.0,
    "name": format!("repository_dee_{sufix}"),
    "owner": { "_id": user_dee_id, "ref": "users" }
  };

  insert_organization(db, organization_acme).await?;
  insert_organization(db, organization_foo).await?;
  insert_organization(db, organization_empty_org).await?;
  insert_user(db, user_foo).await?;
  insert_user(db, user_bar).await?;
  insert_user(db, user_dee).await?;
  insert_user(db, user_empty_user).await?;
  insert_repository(db, repository_tux).await?;
  insert_repository(db, repository_mar).await?;
  insert_repository(db, repository_bar).await?;
  insert_repository(db, repository_dee).await?;

  Ok(())
}

async fn insert_organization(db: &Database, document: bson::Document) -> Result<InsertOneResult, ModelError> {
  db.collection::<bson::Document>("organizations")
    .insert_one(document, None)
    .await
}

async fn insert_repository(db: &Database, document: bson::Document) -> Result<InsertOneResult, ModelError> {
  db.collection::<bson::Document>("repositories")
    .insert_one(document, None)
    .await
}

async fn insert_user(db: &Database, document: bson::Document) -> Result<InsertOneResult, ModelError> {
  db.collection::<bson::Document>("users")
    .insert_one(document, None)
    .await
}
