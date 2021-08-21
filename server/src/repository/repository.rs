use super::utils;
use crate::http::cursor_connection::{CursorConnection, PaginationArguments};
use crate::model::repository::Repository;
use crate::model::user::User;
use mongodb::{
  bson::{self, doc, Document},
  error::Error as MongodbError,
  Collection,
};
use tokio_stream::StreamExt;

pub async fn find_repositories_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let result = find_user(db, login).await?;

  match result {
    Some(user) => find_repositories(db, &user._id, pagination_arguments).await,
    None => Ok(None),
  }
}

async fn find_user(db: &mongodb::Database, login: &String) -> Result<Option<User>, MongodbError> {
  let user_collection: Collection<User> = db.collection_with_type("users");
  user_collection.find_one(doc! { "login": login }, None).await
}

async fn find_repositories(
  db: &mongodb::Database,
  user_id: &bson::oid::ObjectId,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let repo_collection = db.collection("repositories");
  let pipeline = pipeline_paginated_repositories(pagination_arguments, &user_id);

  let cursor = repo_collection.aggregate(pipeline, None).await?;

  let result = cursor.collect::<Vec<Result<Document, MongodbError>>>().await;

  let repositories = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<Repository>>();

  let repositories = utils::repository_to_cursor_connection(repositories);

  Ok(Some(repositories))
}

fn pipeline_paginated_repositories(
  pagination_arguments: PaginationArguments,
  user_id: &bson::oid::ObjectId,
) -> Vec<bson::Document> {
  log::info!("{:?}", &pagination_arguments);
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);
  let repository_id = utils::to_object_id(cursor);

  let filter_by_user_id = match repository_id {
    None => vec![doc! { "$match": { "owner._id": user_id } }],
    Some(repository_id) => vec![doc! { "$match": { "owner._id": user_id, "_id": { operator: repository_id } } }],
  };

  let paginate = vec![
    doc! { "$sort": { "_id": order } },
    doc! { "$limit": limit },
    doc! { "$sort": { "_id": 1 } },
  ];

  vec![].into_iter().chain(filter_by_user_id).chain(paginate).collect()
}
