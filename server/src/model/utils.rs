use crate::{
  lib::cursor_connection::{cursor_to_reference, CursorConnection, Direction, ReferenceFrom},
  model,
};
use mongodb::{
  bson::{self, doc, oid::ObjectId, Document},
  error::Error as MongodbError,
};
use serde::de::DeserializeOwned;
use tokio_stream::StreamExt;

pub fn to_object_id(cursor: Option<String>) -> Option<ObjectId> {
  if cursor.is_none() {
    return None;
  }

  let reference = cursor_to_reference(cursor.unwrap());
  if reference.is_err() {
    return None;
  }

  let id = ObjectId::parse_str(&reference.unwrap());
  if id.is_err() {
    return None;
  }

  let id = id.unwrap();
  Some(id)
}

pub fn to_order(direction: &Direction) -> i32 {
  match direction {
    Direction::Forward => 1,
    _ => -1,
  }
}

pub fn to_operator(direction: &Direction) -> &'static str {
  match direction {
    Direction::Forward => "$gt",
    _ => "$lt",
  }
}

pub fn to_cursor_connection<T: DeserializeOwned>(
  result: Vec<Result<Document, MongodbError>>,
  has_previous_page: bool,
  has_next_page: bool,
  reference_from: ReferenceFrom<T>,
) -> CursorConnection<T> {
  let items = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<T>>();

  CursorConnection::new(items, has_previous_page, has_next_page, reference_from)
}

pub async fn pages_previous_and_next(
  db: &mongodb::Database,
  login: &String,
  result: &Vec<Result<Document, MongodbError>>,
  collection_name: &str,
  field_name: &str,
) -> (bool, bool) {
  let ids = ids_first_and_last(result);
  if let None = ids {
    return (false, false);
  }

  let (first_item_id, last_item_id) = ids.unwrap();
  let pipeline = pipeline_pages_previous_and_next(login, first_item_id, last_item_id, field_name);
  let cursor = db
    .collection::<Document>(collection_name)
    .aggregate(pipeline, None)
    .await
    .unwrap();
  let result_has_pages = cursor.collect::<Vec<Result<Document, MongodbError>>>().await;
  let document = result_has_pages.first().unwrap().as_ref().unwrap();
  let has_previous_page = document.get_bool("has_previous_page").unwrap();
  let has_next_page = document.get_bool("has_next_page").unwrap();

  (has_previous_page, has_next_page)
}

pub fn ids_first_and_last(result: &Vec<Result<Document, MongodbError>>) -> Option<(ObjectId, ObjectId)> {
  if result.len() == 0 {
    return None;
  }

  let first_item = result.first().as_ref().unwrap().as_ref().unwrap();
  let last_item = result.last().as_ref().unwrap().as_ref().unwrap();
  let first_item_id = first_item.get_object_id("_id").unwrap().clone();
  let last_item_id = last_item.get_object_id("_id").unwrap().clone();

  Some((first_item_id, last_item_id))
}

pub fn pipeline_convert_result_values_into_booleans(
  pipeline_previous_page: model::Pipeline,
  pipeline_next_page: model::Pipeline,
) -> model::Pipeline {
  vec![
    doc! { "$facet": {
      "previous": pipeline_previous_page,
      "next": pipeline_next_page,
    } },
    doc! { "$project": {
      "has_previous_page": {
        "$toBool": { "$size": "$previous" }
      },
      "has_next_page": {
        "$toBool": { "$size": "$next" }
      }
    } },
  ]
}

fn pipeline_pages_previous_and_next(
  login: &String,
  first_item_id: ObjectId,
  last_item_id: ObjectId,
  field_name: &str,
) -> model::Pipeline {
  let field_name = format!("${}", field_name);
  let field_name = field_name.as_str();

  let has_previous_page = vec![
    doc! { "$match": { "login": login }},
    doc! { "$unwind": field_name },
    doc! { "$replaceRoot": { "newRoot": field_name } },
    doc! { "$match": { "_id": { "$lt": first_item_id } } },
    doc! { "$limit": 1 },
    doc! { "$count": "count" },
  ];

  let has_next_page = vec![
    doc! { "$match": { "login": login }},
    doc! { "$unwind": field_name },
    doc! { "$replaceRoot": { "newRoot": field_name } },
    doc! { "$match": { "_id": { "$gt": last_item_id } } },
    doc! { "$limit": 1 },
    doc! { "$count": "count" },
  ];

  pipeline_convert_result_values_into_booleans(has_previous_page, has_next_page)
}
