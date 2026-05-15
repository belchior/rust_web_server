use crate::infrastructure::database::{self, QueryParam};
use serde::{Deserialize, Serialize};
use sql_query_builder as sql;
use std::collections::HashMap;
use tokio_postgres::{Error as ClientError, Row};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct License {
  pub license_key: String,
  pub name: String,
}

impl From<Row> for License {
  fn from(row: Row) -> Self {
    Self {
      license_key: row.get("license_key"),
      name: row.get("license_name"),
    }
  }
}

pub async fn find_licenses_by_repository_ids(
  repository_ids: &[i64],
) -> Result<HashMap<i64, Vec<License>>, ClientError> {
  let db = database::get_connection().await.get().await.unwrap();
  let query = query_licenses_by_repository_ids();

  let params: Vec<QueryParam> = vec![&repository_ids];

  let licenses =
    db.query(&query, &params[..])
      .await?
      .into_iter()
      .fold(HashMap::<i64, Vec<License>>::new(), |mut hash, row| {
        let repository_id: i64 = row.get("repository_id");
        let list = if let Some(list) = hash.get_mut(&repository_id) {
          list
        } else {
          hash.insert(repository_id, vec![]);
          hash.get_mut(&repository_id).unwrap()
        };
        list.push(License::from(row));

        hash
      });

  Ok(licenses)
}

fn query_licenses_by_repository_ids() -> String {
  sql::Select::new()
    .select("repository_id, l.*")
    .from("licenses l")
    .inner_join("repositories_licenses rl using(license_key)")
    .where_clause("repository_id = any($1)")
    .as_string()
}
