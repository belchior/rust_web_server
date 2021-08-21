use super::utils;
use crate::http::cursor_connection::{CursorConnection, PaginationArguments};
use crate::model::organization::Organization;
use crate::model::repository::Repository;
use crate::model::user::User;
use mongodb::{
  bson::{self, doc},
  options::FindOneOptions,
  Collection,
};
use tokio_stream::StreamExt;

pub async fn find_user_by_login(
  db: &mongodb::Database,
  login: &String,
  organization_limit: &u32,
) -> Result<Option<User>, mongodb::error::Error> {
  let user_collection: Collection<User> = db.collection_with_type("users");

  let filter = doc! { "login": login };
  let options = FindOneOptions::builder()
    .projection(doc! { "organizations": 0 })
    .build();

  let user = user_collection.find_one(filter, options).await?;

  if user.is_none() {
    return Ok(None);
  }

  let mut cursor = user_collection
    .aggregate(pipeline_paginate_organization(&login, None, organization_limit), None)
    .await?;

  let mut organizations: Vec<Organization> = vec![];
  while let Some(result) = cursor.next().await {
    let org: Organization = bson::from_document(result?)?;
    organizations.push(org);
  }

  let reference_from = |item: &Organization| item._id.to_hex();
  let organizations = CursorConnection::new(organizations, reference_from);
  let user = user.map(|user| User {
    organizations: Some(organizations),
    ..user
  });

  Ok(user)
}

pub async fn find_starred_repositories_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, mongodb::error::Error> {
  let user_collection: Collection<User> = db.collection_with_type("users");

  let mut cursor = user_collection
    .aggregate(
      pipeline_paginated_starred_repositories(login, pagination_arguments),
      None,
    )
    .await?;

  let mut repositories: Vec<Repository> = vec![];

  while let Some(result) = cursor.next().await {
    let repo: Repository = bson::from_document(result?)?;
    repositories.push(repo);
  }

  let repositories = utils::repository_to_cursor_connection(repositories);

  Ok(Some(repositories))
}

pub async fn find_followers_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, mongodb::error::Error> {
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

fn pipeline_paginate_organization(
  login: &String,
  organization_id: Option<&bson::oid::ObjectId>,
  organization_limit: &u32,
) -> Vec<bson::Document> {
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

  let paginate = match organization_id {
    Some(_id) => vec![
      doc! { "$match": { "organizations._id": { "$gt": _id } } },
      doc! { "$limit": organization_limit },
    ],
    None => vec![doc! { "$limit": organization_limit }],
  };

  let project = vec![
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
    .chain(paginate)
    .chain(project)
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
