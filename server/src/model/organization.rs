use super::{repository::Repository, user::User, utils};
use crate::lib::cursor_connection::{CursorConnection, PaginationArguments};
use mongodb::{
  bson::{self, doc},
  error::Error as MongodbError,
  options::FindOneOptions,
};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
  #[serde(rename = "_id")]
  pub _id: bson::oid::ObjectId,
  pub avatar_url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub location: Option<String>,
  pub login: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub people: Option<CursorConnection<User>>,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website_url: Option<String>,
  #[serde(rename = "__typename")]
  pub typename: String,
}

pub async fn find_organization_by_login(
  db: &mongodb::Database,
  login: &String,
) -> Result<Option<Organization>, MongodbError> {
  let organization_collection = db.collection::<Organization>("organizations");

  let filter = doc! { "login": login };
  let options = FindOneOptions::builder().projection(doc! { "people": 0 }).build();
  let organization = organization_collection.find_one(filter, options).await?;

  Ok(organization)
}

pub async fn find_people_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, MongodbError> {
  let organization_collection = db.collection::<Organization>("organizations");

  let pipeline = pipeline_paginated_people(login, pagination_arguments);
  let mut cursor = organization_collection.aggregate(pipeline, None).await?;

  let mut people: Vec<User> = vec![];
  while let Some(result) = cursor.next().await {
    let user: User = bson::from_document(result?)?;
    people.push(user);
  }

  let people = utils::users_to_cursor_connection(people);

  Ok(Some(people))
}

pub async fn find_repositories_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let repositories_collection = db.collection::<Repository>("repositories");
  let organization = find_organization_by_login(db, login).await?;

  if let None = organization {
    return Ok(None);
  }

  let organization = organization.unwrap();

  let pipeline = pipeline_paginated_repositories(&organization._id, pagination_arguments);
  let mut cursor = repositories_collection.aggregate(pipeline, None).await?;

  let mut repositories: Vec<Repository> = vec![];
  while let Some(result) = cursor.next().await {
    let repo: Repository = bson::from_document(result?)?;
    repositories.push(repo);
  }

  let repositories = utils::repositories_to_cursor_connection(repositories);

  Ok(Some(repositories))
}

fn pipeline_paginated_people(login: &String, pagination_arguments: PaginationArguments) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let organization_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let project_people = vec![
    doc! { "$unwind": "$people" },
    doc! { "$project": { "_id": "$people._id" } },
  ];

  let paginate_people = match organization_id {
    Some(_id) => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$match": { "_id": { operator: _id } } },
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
    .chain(project_people)
    .chain(paginate_people)
    .chain(lookup_with_users)
    .collect()
}

fn pipeline_paginated_repositories(
  owner_id: &bson::oid::ObjectId,
  pagination_arguments: PaginationArguments,
) -> Vec<bson::Document> {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let repository_id = utils::to_object_id(cursor);
  let order = utils::to_order(&direction);
  let operator = utils::to_operator(&direction);

  let filter_by_owner_id = vec![doc! { "$match": { "owner._id": owner_id } }];

  let paginate_repositories = match repository_id {
    Some(_id) => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$match": { "_id": { operator: _id } } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
    None => vec![
      doc! { "$sort": { "_id": order } },
      doc! { "$limit": limit },
      doc! { "$sort": { "_id": 1 } },
    ],
  };

  vec![]
    .into_iter()
    .chain(filter_by_owner_id)
    .chain(paginate_repositories)
    .collect()
}
