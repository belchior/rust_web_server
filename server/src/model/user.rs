use crate::{
  lib::cursor_connection::{CursorConnection, PaginationArguments},
  model::{self, organization::Organization, repository::Repository, Pipeline},
};
use mongodb::{
  bson::{doc, oid::ObjectId},
  error::Error as ModelError,
  options::FindOneOptions,
};
use serde::{Deserialize, Serialize};

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
  pub organizations: Option<Vec<Organization>>,
  pub url: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website_url: Option<String>,
  #[serde(rename = "__typename")]
  pub typename: String,
}

pub async fn find_user_by_login(db: &mongodb::Database, login: &String) -> Result<Option<User>, ModelError> {
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
) -> Result<Vec<Organization>, ModelError> {
  let user_collection = db.collection::<User>("users");
  let pipeline = pipeline_paginated_organization(&login, pagination_arguments);
  let cursor = user_collection.aggregate(pipeline, None).await?;
  let items = model::utils::collect_into_model(cursor).await;

  Ok(items)
}

pub async fn find_starred_repositories_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<Repository>, ModelError> {
  let user_collection = db.collection::<User>("users");
  let pipeline = pipeline_paginated_starred_repositories(login, pagination_arguments);
  let cursor = user_collection.aggregate(pipeline, None).await?;
  let items = model::utils::collect_into_model(cursor).await;

  Ok(items)
}

pub async fn find_followers_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<User>, ModelError> {
  let user_collection = db.collection::<User>("users");
  let pipeline = pipeline_paginated_followers(login, pagination_arguments);
  let cursor = user_collection.aggregate(pipeline, None).await?;
  let items = model::utils::collect_into_model(cursor).await;

  Ok(items)
}

pub async fn find_following_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<User>, ModelError> {
  let user_collection = db.collection::<User>("users");
  let pipeline = pipeline_paginated_following(login, pagination_arguments);
  let cursor = user_collection.aggregate(pipeline, None).await?;
  let items = model::utils::collect_into_model(cursor).await;

  Ok(items)
}

pub async fn users_to_cursor_connection(
  db: &mongodb::Database,
  org_login: &String,
  result: Result<Vec<User>, ModelError>,
) -> Result<CursorConnection<User>, ModelError> {
  let result = result?;
  let (has_previous_page, has_next_page) = if result.len() > 0 {
    let coll_name = "organizations";
    let field_name = "people";
    let first_item_id = result.first().unwrap()._id;
    let last_item_id = result.first().unwrap()._id;

    model::utils::pages_previous_and_next(db, org_login, &first_item_id, &last_item_id, coll_name, field_name).await
  } else {
    (false, false)
  };

  let reference_from = |item: &User| item._id.to_hex();
  let items = CursorConnection::new(result, has_previous_page, has_next_page, reference_from);

  Ok(items)
}

fn pipeline_paginated_organization(login: &String, pagination_arguments: PaginationArguments) -> Pipeline {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let organization_id = model::utils::to_object_id(cursor);
  let order = model::utils::to_order(&direction);
  let operator = model::utils::to_operator(&direction);

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

fn pipeline_paginated_starred_repositories(login: &String, pagination_arguments: PaginationArguments) -> Pipeline {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let repository_id = model::utils::to_object_id(cursor);
  let order = model::utils::to_order(&direction);
  let operator = model::utils::to_operator(&direction);

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

fn pipeline_paginated_followers(login: &String, pagination_arguments: PaginationArguments) -> Pipeline {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let user_id = model::utils::to_object_id(cursor);
  let order = model::utils::to_order(&direction);
  let operator = model::utils::to_operator(&direction);

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

fn pipeline_paginated_following(login: &String, pagination_arguments: PaginationArguments) -> Pipeline {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let user_id = model::utils::to_object_id(cursor);
  let order = model::utils::to_order(&direction);
  let operator = model::utils::to_operator(&direction);

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
