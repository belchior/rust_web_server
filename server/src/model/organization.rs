// TODO the mod cursor_connection should be outside of http module
use crate::http::cursor_connection::CursorConnection;
use crate::model::User;
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
}
