use crate::{
  lib::cursor_connection::{CursorConnection, PaginationArguments},
  model,
};
use mongodb::{
  bson::{doc, oid::ObjectId, Document},
  error::Error as MongodbError,
};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Language {
  pub color: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct License {
  pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Owner {
  pub _id: ObjectId,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
  #[serde(rename = "_id")]
  pub _id: ObjectId,
  pub description: Option<String>,
  pub fork_count: f64,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub license_info: Option<License>,
  pub name: String,
  pub owner: Owner,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub primary_language: Option<Language>,
}

pub async fn find_repositories_by_owner_id(
  db: &mongodb::Database,
  owner_id: &ObjectId,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let repo_collection = db.collection::<Repository>("repositories");

  let pipeline = pipeline_paginated_repositories(pagination_arguments, owner_id);
  let cursor = repo_collection.aggregate(pipeline, None).await?;
  let result = cursor.collect::<Vec<Result<Document, MongodbError>>>().await;
  let repositories = utils::to_cursor_connection(result, false, false, |item: &Repository| item._id.to_hex());

  Ok(Some(repositories))
}

fn pipeline_paginated_repositories(pagination_arguments: PaginationArguments, owner_id: &ObjectId) -> model::Pipeline {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let order = model::utils::to_order(&direction);
  let operator = model::utils::to_operator(&direction);
  let repository_id = model::utils::to_object_id(cursor);

  let filter_by_owner_id = match repository_id {
    None => vec![doc! { "$match": { "owner._id": owner_id } }],
    Some(repository_id) => vec![doc! { "$match": { "owner._id": owner_id, "_id": { operator: repository_id } } }],
  };

  let paginate = vec![
    doc! { "$sort": { "_id": order } },
    doc! { "$limit": limit },
    doc! { "$sort": { "_id": 1 } },
  ];

  vec![].into_iter().chain(filter_by_owner_id).chain(paginate).collect()
}
