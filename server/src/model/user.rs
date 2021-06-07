use super::organization::Organization;
use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub _id: Option<bson::oid::ObjectId>,
  pub id: Option<String>,
  pub avatar_url: String,
  pub bio: String,
  pub email: String,
  pub login: String,
  pub name: String,
  pub website_url: String,
  pub organizations: Vec<Organization>,
}

impl User {
  pub fn deserialize(document: bson::Document) -> Result<Self, mongodb::bson::de::Error> {
    let data: User = bson::from_document(document)?;

    Ok(User {
      _id: None,
      id: data._id.map(|_id| _id.to_string()),
      ..data
    })
  }
}
