use super::utils;
use crate::http::cursor_connection::{CursorConnection, PaginationArguments};
use crate::model::organization::Organization;
use crate::model::user::User;
use mongodb::{
  bson::{self, doc},
  error::Error as MongodbError,
  options::FindOneOptions,
  Collection,
};
use tokio_stream::StreamExt;

pub async fn find_organization_by_login(
  db: &mongodb::Database,
  login: &String,
) -> Result<Option<Organization>, MongodbError> {
  let organization_collection: Collection<Organization> = db.collection_with_type("organizations");

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
  let organization_collection: Collection<Organization> = db.collection_with_type("organizations");

  let pipeline = pipeline_paginated_people(login, pagination_arguments);
  let mut cursor = organization_collection.aggregate(pipeline, None).await?;

  let mut people: Vec<User> = vec![];
  while let Some(result) = cursor.next().await {
    let org: User = bson::from_document(result?)?;
    people.push(org);
  }

  let people = utils::users_to_cursor_connection(people);

  // TODO is really needed return Result<Option<_>>?
  Ok(Some(people))
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
