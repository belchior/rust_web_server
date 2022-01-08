use crate::db::db_client_connection;
use mongodb::{
  bson::{self, doc, oid::ObjectId},
  error::Error as MongodbError,
  results::InsertOneResult,
  Database,
};

pub async fn setup() -> Database {
  let db = db_client_connection().await.unwrap();
  drop_collections(&db).await;
  insert_mocked_data(&db).await.unwrap();
  db
}

fn random_id() -> ObjectId {
  ObjectId::new()
}

async fn insert_mocked_data(db: &Database) -> Result<(), MongodbError> {
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
    "login": "organization_foo",
    "people": vec![doc!{ "_id": user_foo_id, "ref": "users" }],
    "url": "https://github.com/foo",
  };
  let organization_acme = doc! {
    "__typename": "Organization",
    "_id": organization_acme_id,
    "avatarUrl": "https://acme.com/avatar.jpg",
    "login": "organization_acme",
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
    "login": "empty_org",
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
    "login": "user_foo",
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
    "login": "user_bar",
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
    "login": "user_dee",
    "organizations": vec![doc! { "_id": organization_acme_id }],
    "url":"https://github.com/bar",
  };
  let user_empty_user = doc! {
    "__typename": "User",
    "_id": random_id(),
    "avatarUrl": "https://empty_user.com/avatar.jpg",
    "email": "empty_user@email.com",
    "login": "empty_user",
    "url":"https://github.com/empty_user",
  };

  let repository_tux = doc! {
    "_id": repository_tux_id,
    "forkCount": 9.0,
    "name": "repository_tux",
    "owner": { "_id": organization_acme_id, "ref": "organizations" }
  };
  let repository_mar = doc! {
    "_id": repository_mar_id,
    "forkCount": 12.0,
    "name": "repository_mar",
    "owner": { "_id": organization_acme_id, "ref": "organizations" }
  };
  let repository_bar = doc! {
    "_id": repository_bar_id,
    "forkCount": 2.0,
    "name": "repository_bar",
    "owner": { "_id": user_bar_id, "ref": "users" }
  };
  let repository_dee = doc! {
    "_id": repository_dee_id,
    "forkCount": 2.0,
    "name": "repository_dee",
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

async fn drop_collections(db: &Database) {
  let orgs_collection = db.collection::<bson::Document>("organizations");
  let repo_collection = db.collection::<bson::Document>("repositories");
  let users_collection = db.collection::<bson::Document>("users");

  orgs_collection.drop(None).await.unwrap();
  repo_collection.drop(None).await.unwrap();
  users_collection.drop(None).await.unwrap();
}

async fn insert_organization(db: &Database, document: bson::Document) -> Result<InsertOneResult, MongodbError> {
  insert_one(db, document, "organizations").await
}

async fn insert_repository(db: &Database, document: bson::Document) -> Result<InsertOneResult, MongodbError> {
  insert_one(db, document, "repositories").await
}

async fn insert_user(db: &Database, document: bson::Document) -> Result<InsertOneResult, MongodbError> {
  insert_one(db, document, "users").await
}

async fn insert_one(
  db: &Database,
  document: bson::Document,
  collection_name: &str,
) -> Result<InsertOneResult, MongodbError> {
  let collection = db.collection::<bson::Document>(collection_name);
  collection.insert_one(document, None).await
}
