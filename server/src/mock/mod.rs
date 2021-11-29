use crate::lib::cursor_connection::CursorConnection;
use crate::model::user::User;
use futures::TryStreamExt;
use mongodb::{
  bson::{self, doc, Bson},
  error::Error as MongodbError,
  results::{InsertManyResult, InsertOneResult},
  Cursor, Database,
};

pub async fn insert_users(db: &Database) -> Result<InsertManyResult, MongodbError> {
  let foo_org = doc! { "_id": bson::oid::ObjectId::new(), "login": "bar-org", "avatar_url": "https://foo.com/avatar.jpg", "url":"https://github.com/foo", "__typename": "Organization" };
  let users = vec![
    doc! { "login": "foo", "email": "foo@email.com", "avatarUrl": "https://foo.com/avatar.jpg", "url":"https://github.com/foo", "__typename": "User" },
    doc! { "login": "bar", "email": "bar@email.com", "avatarUrl": "https://bar.com/avatar.jpg", "url":"https://github.com/bar", "__typename": "User", "documents": vec![doc! { "_id": foo_org.get_object_id("_id").unwrap() }] },
  ];

  insert_one_organization(db, foo_org).await?;
  insert_many_users(db, users).await
}

pub async fn delete_users(db: &Database, user_ids: InsertManyResult) {
  let ids: Vec<_> = user_ids.inserted_ids.into_values().collect();
  let cursor = find_users_by_ids(db, ids.clone()).await.unwrap();
  let users: Vec<_> = cursor.try_collect().await.unwrap();
  let org_ids = organization_ids_from_users(users);

  delete_many(db, org_ids, "organizations").await;
  delete_many(db, ids, "users").await;
}

async fn delete_many(db: &Database, ids: Vec<Bson>, collection_name: &str) {
  let collection = db.collection::<bson::Document>(collection_name);
  let query = doc! { "_id": { "$in": ids } };

  collection.delete_many(query, None).await.unwrap();
}

async fn find_users_by_ids(db: &Database, ids: Vec<Bson>) -> Result<Cursor<User>, MongodbError> {
  let collection = db.collection::<User>("users");
  let filter = doc! { "_id": { "$in": ids } };

  collection.find(filter, None).await
}

async fn insert_one_organization(db: &Database, document: bson::Document) -> Result<InsertOneResult, MongodbError> {
  let collection = db.collection::<bson::Document>("organizations");

  collection.insert_one(document, None).await
}

async fn insert_many_users(db: &Database, documents: Vec<bson::Document>) -> Result<InsertManyResult, MongodbError> {
  let user_collection = db.collection::<bson::Document>("users");

  user_collection.insert_many(documents, None).await
}

fn organization_ids_from_users(users: Vec<User>) -> Vec<bson::Bson> {
  users
    .into_iter()
    .map(|user| match user.organizations {
      None => vec![],
      Some(orgs) => CursorConnection::to_vec(orgs),
    })
    .flatten()
    .map(|org| bson::Bson::ObjectId(org._id))
    .collect()
}
