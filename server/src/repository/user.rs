use crate::cursor_connection::CursorConnection;
use crate::model::organization::Organization;
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
  organizations_limit: &u32,
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
    .aggregate(
      pipeline_paginate_organization(&login, None, organizations_limit),
      None,
    )
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

fn pipeline_paginate_organization(
  user_login: &String,
  organization_id: Option<&bson::oid::ObjectId>,
  organizations_limit: &u32,
) -> Vec<bson::Document> {
  let filter_by_login = vec![doc! { "$match": { "login": user_login } }];

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
      doc! { "$limit": organizations_limit },
    ],
    None => vec![doc! { "$limit": organizations_limit }],
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
