use crate::cursor_connection::{cursor_to_reference, CursorConnection, PaginationOptions};
use crate::model::repository::Repository;
use crate::model::user::User;
use mongodb::{
  bson::{self, doc},
  error::Error as MongodbError,
  Collection,
};
use tokio_stream::StreamExt;

fn to_filter(cursor: Option<String>, user: &User) -> bson::Document {
  let default_doc = doc! { "owner._id": &user._id };

  if cursor.is_none() {
    return default_doc;
  }

  let reference = cursor_to_reference(cursor.unwrap());
  if reference.is_err() {
    return default_doc;
  }

  let repo_id = bson::oid::ObjectId::with_string(&reference.unwrap());
  if repo_id.is_err() {
    return default_doc;
  }
  let repo_id = repo_id.unwrap();

  doc! {
    "owner._id": &user._id,
    "_id": {
      "$gt": repo_id
    }
  }
}

fn to_options(limit: Option<i64>) -> mongodb::options::FindOptions {
  let limit = std::cmp::min(100, limit.unwrap_or(10));
  mongodb::options::FindOptions::builder()
    .limit(limit)
    .build()
}

fn to_cursor_connection(repositories: Vec<Repository>) -> CursorConnection<Repository> {
  let reference_from = |item: &Repository| item._id.to_hex();
  CursorConnection::new(repositories, reference_from)
}

async fn find_user(db: &mongodb::Database, login: &String) -> Result<Option<User>, MongodbError> {
  let user_collection: Collection<User> = db.collection_with_type("users");
  user_collection
    .find_one(doc! { "login": login }, None)
    .await
}

async fn find_repositories(
  db: &mongodb::Database,
  user: User,
  pagination_options: PaginationOptions,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let repo_collection = db.collection("repositories");

  let filter = to_filter(pagination_options.cursor, &user);
  let options = to_options(pagination_options.limit);
  let cursor = repo_collection.find(filter, options).await?;

  let result = cursor
    .collect::<Vec<Result<bson::Document, MongodbError>>>()
    .await;

  let repositories = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<Repository>>();

  let repositories = to_cursor_connection(repositories);

  Ok(Some(repositories))
}

pub async fn find_repositories_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_options: PaginationOptions,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let result = find_user(db, login).await?;

  match result {
    Some(user) => find_repositories(db, user, pagination_options).await,
    None => Ok(None),
  }
}
