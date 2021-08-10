use crate::http::cursor_connection::CursorConnection;
use crate::model::organization::Organization;
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
  #[serde(rename = "_id")]
  pub _id: bson::oid::ObjectId,
  pub avatar_url: String,
  pub bio: String,
  pub email: String,
  pub login: String,
  pub name: String,
  pub website_url: String,
  #[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
  pub organizations: Option<CursorConnection<Organization>>,
}
