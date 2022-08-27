use crate::lib::cursor_connection::{CursorConnection, PaginationArguments};
use crate::model;
use mongodb::{
  bson::{doc, oid::ObjectId, Document},
  error::Error as ModelError,
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
) -> Result<Vec<Repository>, ModelError> {
  let repo_collection = db.collection::<Repository>("repositories");
  let pipeline = pipeline_paginated_repositories(pagination_arguments, owner_id);
  let cursor = repo_collection.aggregate(pipeline, None).await?;
  let items = model::utils::collect_into_model(cursor).await;

  Ok(items)
}

pub async fn repositories_to_cursor_connection(
  db: &mongodb::Database,
  owner_id: &ObjectId,
  result: Result<Vec<Repository>, ModelError>,
) -> Result<CursorConnection<Repository>, ModelError> {
  let result = result?;
  let (has_previous_page, has_next_page) = if result.len() > 0 {
    let first_item_id = result.first().unwrap()._id;
    let last_item_id = result.first().unwrap()._id;

    pages_previous_and_next(db, owner_id, &first_item_id, &last_item_id).await
  } else {
    (false, false)
  };

  let reference_from = |item: &Repository| item._id.to_hex();
  let items = CursorConnection::new(result, has_previous_page, has_next_page, reference_from);

  Ok(items)
}

pub async fn pages_previous_and_next(
  db: &mongodb::Database,
  owner_id: &ObjectId,
  first_item_id: &ObjectId,
  last_item_id: &ObjectId,
) -> (bool, bool) {
  let pipeline = pipeline_pages_previous_and_next(owner_id, first_item_id, last_item_id);
  let cursor = db
    .collection::<Document>("repositories")
    .aggregate(pipeline, None)
    .await
    .unwrap();
  let result_has_pages = cursor.collect::<Vec<_>>().await;
  let document = result_has_pages.first().unwrap().as_ref().unwrap();
  let has_previous_page = document.get_bool("has_previous_page").unwrap();
  let has_next_page = document.get_bool("has_next_page").unwrap();

  (has_previous_page, has_next_page)
}

fn pipeline_pages_previous_and_next(
  owner_id: &ObjectId,
  first_item_id: &ObjectId,
  last_item_id: &ObjectId,
) -> model::Pipeline {
  let pipeline_previous_page = vec![
    doc! { "$match": {
      "owner._id": owner_id,
      "_id": { "$lt": first_item_id }
    } },
    doc! { "$limit": 1 },
    doc! { "$count": "count" },
  ];

  let pipeline_next_page = vec![
    doc! { "$match": {
      "owner._id": owner_id,
      "_id": { "$gt": last_item_id }
    } },
    doc! { "$limit": 1 },
    doc! { "$count": "count" },
  ];

  model::utils::pipeline_convert_result_values_into_booleans(pipeline_previous_page, pipeline_next_page)
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
