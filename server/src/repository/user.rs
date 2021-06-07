use mongodb::bson::{self, doc};

pub async fn find_user_by_login(
  db: &mongodb::Database,
  login: &String,
) -> Result<mongodb::Cursor, mongodb::error::Error> {
  let user_collection = db.collection("users");

  user_collection
    .aggregate(pipeline_find_user_by_login(&login), None)
    .await
}

fn pipeline_find_user_by_login(login: &String) -> Vec<bson::Document> {
  let stage_match_user_by_login = doc! {
    "$match": {
      "login": login
    }
  };

  vec![stage_match_user_by_login]
}
