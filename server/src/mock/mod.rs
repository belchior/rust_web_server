use crate::db::db_client_connection;
use mongodb::{
  bson::{self, doc},
  error::Error as MongodbError,
  results::InsertOneResult,
  Database,
};

pub async fn setup() -> (Database, InsertOneResult) {
  let db = db_client_connection().await.unwrap();
  drop_collections(&db).await;
  let users = insert_mocked_data(&db).await.unwrap();
  (db, users)
}

fn random_id() -> bson::oid::ObjectId {
  bson::oid::ObjectId::new()
}

async fn insert_mocked_data(db: &Database) -> Result<InsertOneResult, MongodbError> {
  let user_foo_id = random_id();
  let user_bar_id = random_id();
  let user_dee_id = random_id();
  let repository_bar_id = random_id();
  let organization_foo_id = random_id();

  let repository_foo = doc! {
    "_id": random_id(),
    "forkCount": 9.0,
    "name": "repository_foo",
    "owner": { "_id": organization_foo_id, "ref": "organizations" }
  };
  let user_foo = doc! {
    "__typename": "User",
    "_id": user_foo_id,
    "avatarUrl": "https://foo.com/avatar.jpg",
    "email": "foo@email.com",
    "login": "user_foo",
    "organizations": vec![doc! { "_id": organization_foo_id }],
    "url":"https://github.com/foo",
  };
  let organization_foo = doc! {
    "__typename": "Organization",
    "_id": organization_foo_id,
    "avatarUrl": "https://foo.com/avatar.jpg",
    "login": "organization_foo",
    "people": vec![doc!{ "_id": user_foo_id, "ref": "users" }],
    "url": "https://github.com/foo",
  };

  let user_dee = doc! {
    "__typename": "User",
    "_id": user_dee_id,
    "avatarUrl": "https://dee.com/avatar.jpg",
    "email": "dee@email.com",
    "followers": vec![doc! { "_id": user_bar_id }],
    "following": vec![doc! { "_id": user_bar_id }],
    "login": "user_dee",
    "url":"https://github.com/bar",
  };
  let repository_bar = doc! {
    "_id": repository_bar_id,
    "forkCount": 2.0,
    "name": "repository_bar",
    "owner": { "_id": user_bar_id, "ref": "users" }
  };
  let user_bar = doc! {
    "__typename": "User",
    "_id": user_bar_id,
    "avatarUrl": "https://bar.com/avatar.jpg",
    "email": "bar@email.com",
    "followers": vec![doc! { "_id": user_dee_id }],
    "following": vec![doc! { "_id": user_dee_id }],
    "login": "user_bar",
    "starredRepositories": vec![doc! { "_id": repository_bar_id }],
    "url":"https://github.com/bar",
  };

  insert_organization(db, organization_foo).await?;
  insert_repository(db, repository_foo).await?;
  insert_repository(db, repository_bar).await?;
  insert_user(db, user_bar).await?;
  insert_user(db, user_dee).await?;
  insert_user(db, user_foo).await
}

async fn drop_collections(db: &Database) {
  let orgs_collection = db.collection::<bson::Document>("organizations");
  let repo_collection = db.collection::<bson::Document>("repositories");
  let users_collection = db.collection::<bson::Document>("users");

  orgs_collection.delete_many(doc! {}, None).await.unwrap();
  repo_collection.delete_many(doc! {}, None).await.unwrap();
  users_collection.delete_many(doc! {}, None).await.unwrap();
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
