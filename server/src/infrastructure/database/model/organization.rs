use crate::infrastructure::database::{
  self,
  cursor_connection::{CursorConnection, PaginationArguments},
  model::{self, user::User},
};
use mongodb::{
  bson::{doc, oid::ObjectId, serde_helpers::serialize_object_id_as_hex_string},
  error::Error as ModelError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
  #[serde(rename = "_id")]
  #[serde(serialize_with = "serialize_object_id_as_hex_string")]
  pub _id: ObjectId,
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

pub async fn find_organization_by_login(login: &String) -> Result<Option<Organization>, ModelError> {
  let db = database::get_connection().await;
  let organization_collection = db.collection::<Organization>("organizations");

  let filter = doc! { "login": login };
  let organization = organization_collection
    .find_one(filter)
    .projection(doc! { "people": 0 })
    .await?;

  Ok(organization)
}

pub async fn find_people_by_login(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Vec<User>, ModelError> {
  let db = database::get_connection().await;
  let organization_collection = db.collection::<Organization>("organizations");
  let pipeline = pipeline_paginated_people(login, pagination_arguments);
  let cursor = organization_collection.aggregate(pipeline).await?;
  let result = model::utils::collect_into_model(cursor).await;

  Ok(result)
}

pub async fn organizations_users_to_cursor_connection(
  org_login: &String,
  result: Result<Vec<User>, ModelError>,
) -> Result<CursorConnection<User>, ModelError> {
  let db = database::get_connection().await;
  let result = result?;
  let (has_previous_page, has_next_page) = if result.len() > 0 {
    let coll_name = "organizations";
    let field_name = "people";
    let first_item_id = result.first().unwrap()._id;
    let last_item_id = result.first().unwrap()._id;

    model::utils::pages_previous_and_next(&db, org_login, &first_item_id, &last_item_id, coll_name, field_name).await
  } else {
    (false, false)
  };

  let reference_from = |item: &User| item._id.to_hex();
  let items = CursorConnection::new(result, has_previous_page, has_next_page, reference_from);

  Ok(items)
}

fn pipeline_paginated_people(login: &String, pagination_arguments: PaginationArguments) -> model::Pipeline {
  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();
  let user_id = model::utils::to_object_id(cursor);
  let order = model::utils::to_order(&direction);
  let operator = model::utils::to_operator(&direction);

  let filter_by_login = vec![doc! { "$match": { "login": login } }];

  let keep_only_people = vec![
    doc! { "$project": { "_id": 0, "people": 1 } },
    doc! { "$unwind": "$people" },
  ];

  let lookup_with_users = vec![
    doc! { "$lookup": {
      "from": "users",
      "localField": "people._id",
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
    .chain(keep_only_people)
    .chain(lookup_with_users)
    .chain(filter_by_user_id)
    .chain(paginate_items)
    .collect()
}
