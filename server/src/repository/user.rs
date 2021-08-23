use super::utils;
use crate::http::cursor_connection::{CursorConnection, PaginationArguments};
use crate::model::{Organization, Repository, User};
use mongodb::{
  bson::{self, doc},
  error::Error as MongodbError,
  options::FindOneOptions,
  Collection,
};
use tokio_stream::StreamExt;

pub async fn find_user_by_login(db: &mongodb::Database, login: &String) -> Result<Option<User>, MongodbError> {
  let user_collection: Collection<User> = db.collection_with_type("users");

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
  let user_collection: Collection<User> = db.collection_with_type("users");

  let pipeline = pipeline_paginated_organization(&login, pagination_arguments);
  let mut cursor = user_collection.aggregate(pipeline, None).await?;

  let mut organizations: Vec<Organization> = vec![];
  while let Some(result) = cursor.next().await {
    let org: Organization = bson::from_document(result?)?;
    organizations.push(org);
  }

  let organizations = utils::organizations_to_cursor_connection(organizations);

  Ok(Some(organizations))
}

pub async fn find_starred_repositories_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let user_collection: Collection<User> = db.collection_with_type("users");

  let pipeline = pipeline_paginated_starred_repositories(login, pagination_arguments);
  let mut cursor = user_collection.aggregate(pipeline, None).await?;

  let mut repositories: Vec<Repository> = vec![];
  while let Some(result) = cursor.next().await {
    let repo: Repository = bson::from_document(result?)?;
    repositories.push(repo);
  }

  let repositories = utils::repositories_to_cursor_connection(repositories);

  Ok(Some(repositories))
}

pub async fn find_followers_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, MongodbError> {
  let user_collection: Collection<User> = db.collection_with_type("users");
  let pipeline = pipeline_paginated_followers(login, pagination_arguments);
  let mut cursor = user_collection.aggregate(pipeline, None).await?;

  let mut followers: Vec<User> = vec![];
  while let Some(result) = cursor.next().await {
    let follower: User = bson::from_document(result?)?;
    followers.push(follower);
  }

  let followers = utils::users_to_cursor_connection(followers);

  Ok(Some(followers))
}

pub async fn find_following_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, MongodbError> {
  let user_collection: Collection<User> = db.collection_with_type("users");
  let pipeline = pipeline_paginated_following(login, pagination_arguments);
  let mut cursor = user_collection.aggregate(pipeline, None).await?;

  let mut following: Vec<User> = vec![];
  while let Some(result) = cursor.next().await {
    let follower: User = bson::from_document(result?)?;
    following.push(follower);
  }

  let following = utils::users_to_cursor_connection(following);

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
