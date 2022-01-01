use super::{organization::Organization, repository::Repository, utils};
use crate::lib::cursor_connection::{CursorConnection, PaginationArguments};
use mongodb::{
  bson::{self, doc, oid::ObjectId, Document},
  error::Error as MongodbError,
  options::FindOneOptions,
};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
  // TODO find a way to serialize _id into id with hex version
  #[serde(rename = "_id")]
  pub _id: ObjectId,
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bio: Option<String>,
  pub email: String,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
  pub organizations: Option<CursorConnection<Organization>>,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website_url: Option<String>,
  #[serde(rename = "__typename")]
  pub typename: String,
}

pub async fn find_user_by_login(db: &mongodb::Database, login: &String) -> Result<Option<User>, MongodbError> {
  let user_collection = db.collection::<User>("users");
  let filter = doc! { "login": login };
  let options = FindOneOptions::builder()
    .projection(doc! { "organizations": 0 })
    .build();

  let user = user_collection.find_one(filter, options).await?;

  Ok(user)
}

pub async fn find_organizations_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Organization>>, MongodbError> {
  let user_collection = db.collection::<User>("users");
  let pipeline = pipeline_paginated_organization(&login, pagination_arguments);
  let cursor = user_collection.aggregate(pipeline, None).await?;
  let result = cursor.collect::<Vec<Result<Document, MongodbError>>>().await;
  let (has_previous_page, has_next_page) = has_pages_previous_and_next(db, login, &result, "organizations").await;
  let reference_from = |item: &Organization| item._id.to_hex();

  let organizations = utils::to_cursor_connection(result, has_previous_page, has_next_page, reference_from);

  Ok(Some(organizations))
}

pub async fn find_starred_repositories_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let user_collection = db.collection::<User>("users");
  let pipeline = pipeline_paginated_starred_repositories(login, pagination_arguments);
  let cursor = user_collection.aggregate(pipeline, None).await?;
  let result = cursor.collect::<Vec<Result<Document, MongodbError>>>().await;
  let (has_previous_page, has_next_page) = has_pages_previous_and_next(db, login, &result, "starredRepositories").await;
  let reference_from = |item: &Repository| item._id.to_hex();

  let repositories = utils::to_cursor_connection(result, has_previous_page, has_next_page, reference_from);

  Ok(Some(repositories))
}

pub async fn find_followers_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, MongodbError> {
  let user_collection = db.collection::<User>("users");
  let pipeline = pipeline_paginated_followers(login, pagination_arguments);
  let cursor = user_collection.aggregate(pipeline, None).await?;
  let result = cursor.collect::<Vec<Result<Document, MongodbError>>>().await;
  let (has_previous_page, has_next_page) = has_pages_previous_and_next(db, login, &result, "followers").await;
  let reference_from = |item: &User| item._id.to_hex();

  let followers = utils::to_cursor_connection(result, has_previous_page, has_next_page, reference_from);

  Ok(Some(followers))
}

pub async fn find_following_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, MongodbError> {
  let user_collection = db.collection::<User>("users");
  let pipeline = pipeline_paginated_following(login, pagination_arguments);
  let cursor = user_collection.aggregate(pipeline, None).await?;
  let result = cursor.collect::<Vec<Result<Document, MongodbError>>>().await;
  let (has_previous_page, has_next_page) = has_pages_previous_and_next(db, login, &result, "following").await;
  let reference_from = |item: &User| item._id.to_hex();

  let following = utils::to_cursor_connection(result, has_previous_page, has_next_page, reference_from);

  Ok(Some(following))
}

fn ids_first_and_last(result: &Vec<Result<Document, MongodbError>>) -> Option<(ObjectId, ObjectId)> {
  if result.len() == 0 {
    return None;
  }

  let first_item = result.first().as_ref().unwrap().as_ref().unwrap();
  let last_item = result.last().as_ref().unwrap().as_ref().unwrap();
  let first_item_id = first_item.get_object_id("_id").unwrap().clone();
  let last_item_id = last_item.get_object_id("_id").unwrap().clone();

  Some((first_item_id, last_item_id))
}

async fn has_pages_previous_and_next(
  db: &mongodb::Database,
  login: &String,
  result: &Vec<Result<Document, MongodbError>>,
  field_name: &str,
) -> (bool, bool) {
  let ids = ids_first_and_last(result);
  if let None = ids {
    return (false, false);
  }

  let (first_item_id, last_item_id) = ids.unwrap();
  let pipeline = pipeline_has_pages_previous_and_next(login, first_item_id, last_item_id, field_name);
  let cursor = db.collection::<User>("users").aggregate(pipeline, None).await.unwrap();
  let result_has_pages = cursor.collect::<Vec<Result<Document, MongodbError>>>().await;
  let document = result_has_pages.first().unwrap().as_ref().unwrap();
  let has_previous_page = document.get_bool("has_previous_page").unwrap();
  let has_next_page = document.get_bool("has_next_page").unwrap();

  (has_previous_page, has_next_page)
}

fn pipeline_has_pages_previous_and_next(
  login: &String,
  first_item_id: ObjectId,
  last_item_id: ObjectId,
  field_name: &str,
) -> Vec<bson::Document> {
  let field_name = format!("${}", field_name);
  let field_name = field_name.as_str();

  let has_previous_page = vec![
    doc! { "$match": { "login": login }},
    doc! { "$unwind": field_name },
    doc! { "$replaceRoot": { "newRoot": field_name } },
    doc! { "$sort": { "_id": 1 } },
    doc! { "$match": { "_id": { "$gt": last_item_id } } },
    doc! { "$limit": 1 },
    doc! { "$count": "count" },
  ];

  let has_next_page = vec![
    doc! { "$match": { "login": login }},
    doc! { "$unwind": field_name },
    doc! { "$replaceRoot": { "newRoot": field_name } },
    doc! { "$sort": { "_id": -1 } },
    doc! { "$match": { "_id": { "$lt": first_item_id } } },
    doc! { "$limit": 1 },
    doc! { "$count": "count" },
  ];

  let group_queries_in_the_same_result = vec![doc! { "$facet": {
    "previous": has_previous_page,
    "next": has_next_page,
  } }];

  let convert_result_values_into_booleans = vec![
    doc! { "$project": {
      "previous": {
        "$ifNull": [{ "$arrayElemAt": ["$previous.count", 0] }, 0 ]
      },
      "next": {
        "$ifNull": [{ "$arrayElemAt": ["$next.count", 0] }, 0 ]
      },
    } },
    doc! { "$project": {
      "has_previous_page": { "$toBool": "$previous" },
      "has_next_page": { "$toBool": "$next" }
    } },
  ];

  vec![]
    .into_iter()
    .chain(group_queries_in_the_same_result)
    .chain(convert_result_values_into_booleans)
    .collect()
}

fn pipeline_paginated_organization(login: &String, pagination_arguments: PaginationArguments) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let organization_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let keep_only_organizations = vec![
    doc! { "$project": { "_id": 0, "organizations": 1 } },
    doc! { "$unwind": "$organizations" },
  ];

  let lookup_with_organizations = vec![
    doc! { "$lookup": {
      "from": "organizations",
      "localField": "organizations._id",
      "foreignField": "_id",
      "as": "organizations",
    }},
    doc! { "$replaceRoot": {
      "newRoot": {
        "$arrayElemAt": [ "$organizations", 0 ]
      }
    } },
  ];

  let filter_by_organization_id = match organization_id {
    None => vec![],
    Some(_id) => vec![doc! { "$match": { "_id": { operator: _id } } }],
  };

  let paginate_items = vec![
    doc! { "$sort": { "_id": order } },
    doc! { "$limit": limit },
    doc! { "$sort": { "_id": 1 } },
  ];

  let project_organizations = vec![doc! { "$project": {
    "login": 1,
    "name": 1,
    "avatarUrl": 1,
    "url": 1,
    "__typename": 1,
  } }];

  vec![]
    .into_iter()
    .chain(filter_by_login)
    .chain(keep_only_organizations)
    .chain(lookup_with_organizations)
    .chain(filter_by_organization_id)
    .chain(paginate_items)
    .chain(project_organizations)
    .collect()
}

fn pipeline_paginated_starred_repositories(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let repository_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let keep_only_starred_repositories = vec![
    doc! { "$project": { "_id": 0, "starredRepositories": 1 } },
    doc! { "$unwind": "$starredRepositories" },
  ];

  let lookup_with_repositories = vec![
    doc! { "$lookup": {
      "from": "repositories",
      "localField": "starredRepositories._id",
      "foreignField": "_id",
      "as": "item"
    } },
    doc! { "$replaceRoot": {
      "newRoot": {
        "$arrayElemAt": [ "$item", 0 ]
      }
    } },
  ];

  let filter_by_repository_id = match repository_id {
    None => vec![],
    Some(_id) => vec![doc! { "$match": { "_id": { operator: _id } } }],
  };

  let paginate_items = vec![
    doc! { "$sort": { "_id": order } },
    doc! { "$limit": limit },
    doc! { "$sort": { "_id": 1 } },
  ];

  vec![]
    .into_iter()
    .chain(filter_by_login)
    .chain(keep_only_starred_repositories)
    .chain(lookup_with_repositories)
    .chain(filter_by_repository_id)
    .chain(paginate_items)
    .collect()
}

fn pipeline_paginated_followers(login: &String, pagination_arguments: PaginationArguments) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let user_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let keep_only_followers = vec![
    doc! { "$project": { "_id": 0, "followers": 1 } },
    doc! { "$unwind": "$followers" },
  ];

  let lookup_with_users = vec![
    doc! { "$lookup": {
      "from": "users",
      "localField": "followers._id",
      "foreignField": "_id",
      "as": "item"
    } },
    doc! { "$replaceRoot": {
      "newRoot": {
        "$arrayElemAt": [ "$item", 0 ]
      }
    } },
  ];

  let filter_by_user_id = match user_id {
    None => vec![],
    Some(_id) => vec![doc! { "$match": { "_id": { operator: _id } } }],
  };

  let paginate_items = vec![
    doc! { "$sort": { "_id": order } },
    doc! { "$limit": limit },
    doc! { "$sort": { "_id": 1 } },
  ];

  vec![]
    .into_iter()
    .chain(filter_by_login)
    .chain(keep_only_followers)
    .chain(lookup_with_users)
    .chain(filter_by_user_id)
    .chain(paginate_items)
    .collect()
}

fn pipeline_paginated_following(login: &String, pagination_arguments: PaginationArguments) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let user_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let keep_only_following = vec![
    doc! { "$project": { "_id": 0, "following": 1 } },
    doc! { "$unwind": "$following" },
  ];

  let lookup_with_users = vec![
    doc! { "$lookup": {
      "from": "users",
      "localField": "following._id",
      "foreignField": "_id",
      "as": "item"
    } },
    doc! { "$replaceRoot": {
      "newRoot": {
        "$arrayElemAt": [ "$item", 0 ]
      }
    } },
  ];

  let filter_by_user_id = match user_id {
    None => vec![],
    Some(_id) => vec![doc! { "$match": { "_id": { operator: _id } } }],
  };

  let paginate_items = vec![
    doc! { "$sort": { "_id": order } },
    doc! { "$limit": limit },
    doc! { "$sort": { "_id": 1 } },
  ];

  vec![]
    .into_iter()
    .chain(filter_by_login)
    .chain(keep_only_following)
    .chain(lookup_with_users)
    .chain(filter_by_user_id)
    .chain(paginate_items)
    .collect()
}
