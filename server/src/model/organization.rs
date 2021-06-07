use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub _id: Option<bson::oid::ObjectId>,
  pub id: Option<String>,
  pub avatar_url: String,
  pub url: String,
  pub login: String,
}

impl Organization {
  pub fn deserialize(document: bson::Document) -> Result<Self, mongodb::bson::de::Error> {
    let data: Organization = bson::from_document(document)?;

    Ok(Organization {
      _id: None,
      id: data._id.map(|_id| _id.to_string()),
      ..data
    })
  }
}
