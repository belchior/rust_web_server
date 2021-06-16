use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
  pub color: String,
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct License {
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
  pub _id: bson::oid::ObjectId,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
  #[serde(rename = "_id")]
  pub _id: bson::oid::ObjectId,
  pub primary_language: Option<Language>,
  pub name: String,
  pub description: Option<String>,
  // @TODO the correct type of this field should be u32
  pub fork_count: f64,
  pub license_info: Option<License>,
  pub owner: Owner,
}
