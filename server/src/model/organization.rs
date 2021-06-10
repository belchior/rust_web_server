use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
  #[serde(rename = "_id")]
  pub _id: bson::oid::ObjectId,
  pub avatar_url: String,
  pub url: String,
  pub login: String,
}
