use super::{organization::Organization, repository::Repository, utils};
use crate::lib::cursor_connection::{CursorConnection, PaginationArguments};
use mongodb::{
  bson::{self, doc, Document},
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
  pub _id: bson::oid::ObjectId,
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
  let organizations = utils::to_cursor_connection(result, |item: &Organization| item._id.to_hex());

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
  let repositories = utils::to_cursor_connection(result, |item: &Repository| item._id.to_hex());

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
  let followers = utils::to_cursor_connection(result, |item: &User| item._id.to_hex());

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
  let following = utils::to_cursor_connection(result, |item: &User| item._id.to_hex());

  Ok(Some(following))
}

fn pipeline_paginated_organization(login: &String, pagination_arguments: PaginationArguments) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let organization_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let lookup_with_organizations = vec![
    doc! { "$lookup": {
      "from": "organizations",
      "localField": "organizations._id",
      "foreignField": "_id",
      "as": "organizations",
    } },
    doc! { "$unwind": "$organizations" },
  ];

  let paginate_organizations = match organization_id {
    Some(_id) => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$match": { "organizations._id": { operator: _id } } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
    None => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
  };

  let project_organizations = vec![
    doc! { "$replaceRoot": {
      "newRoot": "$organizations"
    } },
    doc! { "$project": {
      "login": 1,
      "name": 1,
      "avatarUrl": 1,
      "url": 1,
      "__typename": 1,
    } },
  ];

  vec![]
    .into_iter()
    .chain(filter_by_login)
    .chain(lookup_with_organizations)
    .chain(paginate_organizations)
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

  let project_starred_repositories = vec![
    doc! { "$unwind": "$starredRepositories" },
    doc! { "$project": { "_id": "$starredRepositories._id" } },
  ];

  let paginate_repositories = match repository_id {
    Some(repository_id) => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$match": { "_id": { operator: repository_id } } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
    None => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
  };

  let lookup_with_repositories = vec![
    doc! { "$lookup": {
      "from": "repositories",
      "localField": "_id",
      "foreignField": "_id",
      "as": "item"
    } },
    doc! { "$replaceRoot": {
      "newRoot": {
        "$arrayElemAt": [ "$item", 0 ]
      }
    } },
  ];

  vec![]
    .into_iter()
    .chain(filter_by_login)
    .chain(project_starred_repositories)
    .chain(paginate_repositories)
    .chain(lookup_with_repositories)
    .collect()
}

fn pipeline_paginated_followers(login: &String, pagination_arguments: PaginationArguments) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let user_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let project_followers = vec![
    doc! { "$unwind": "$followers" },
    doc! { "$project": { "_id": "$followers._id" } },
  ];

  let paginate_folowers = match user_id {
    Some(user_id) => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$match": { "_id": { operator: user_id } } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
    None => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
  };

  let lookup_with_users = vec![
    doc! { "$lookup": {
      "from": "users",
      "localField": "_id",
      "foreignField": "_id",
      "as": "item"
    } },
    doc! { "$replaceRoot": {
      "newRoot": {
        "$arrayElemAt": [ "$item", 0 ]
      }
    } },
  ];

  vec![]
    .into_iter()
    .chain(filter_by_login)
    .chain(project_followers)
    .chain(paginate_folowers)
    .chain(lookup_with_users)
    .collect()
}

fn pipeline_paginated_following(login: &String, pagination_arguments: PaginationArguments) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let user_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let project_following = vec![
    doc! { "$unwind": "$following" },
    doc! { "$project": { "_id": "$following._id" } },
  ];

  let paginate_folowing = match user_id {
    Some(user_id) => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$match": { "_id": { operator: user_id } } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
    None => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
  };

  let lookup_with_using = vec![
    doc! { "$lookup": {
      "from": "users",
      "localField": "_id",
      "foreignField": "_id",
      "as": "item"
    } },
    doc! { "$replaceRoot": {
      "newRoot": {
        "$arrayElemAt": [ "$item", 0 ]
      }
    } },
  ];

  vec![]
    .into_iter()
    .chain(filter_by_login)
    .chain(project_following)
    .chain(paginate_folowing)
    .chain(lookup_with_using)
    .collect()
}
