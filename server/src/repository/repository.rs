use crate::cursor_connection::{
  cursor_to_reference, CursorConnection, Direction, PaginationArguments,
};
use crate::model::repository::Repository;
use crate::model::user::User;
use mongodb::{
  bson::{self, doc, Document},
  error::Error as MongodbError,
  Collection,
};
use tokio_stream::StreamExt;

fn cursor_to_object_id(cursor: Option<String>) -> Option<bson::oid::ObjectId> {
  if cursor.is_none() {
    return None;
  }

  let reference = cursor_to_reference(cursor.unwrap());
  if reference.is_err() {
    return None;
  }

  let id = bson::oid::ObjectId::with_string(&reference.unwrap());
  if id.is_err() {
    return None;
  }

  let id = id.unwrap();
  Some(id)
}

fn to_cursor_connection(repositories: Vec<Repository>) -> CursorConnection<Repository> {
  let reference_from = |item: &Repository| item._id.to_hex();
  CursorConnection::new(repositories, reference_from)
}

async fn find_user(db: &mongodb::Database, login: &String) -> Result<Option<User>, MongodbError> {
  let user_collection: Collection<User> = db.collection_with_type("users");
  user_collection
    .find_one(doc! { "login": login }, None)
    .await
}

async fn find_repositories(
  db: &mongodb::Database,
  user: &User,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let repo_collection = db.collection("repositories");

  let (direction, limit, cursor) = pagination_arguments.parse_args().unwrap();

  let repo_id = cursor_to_object_id(cursor);
  let stage_match = match repo_id {
    None => doc! {
      "$match": {
        "owner._id": &user._id,
      }
    },
    Some(repo_id) => {
      let operator = match direction {
        Direction::Forward => "$gt",
        _ => "$lt",
      };
      doc! {
        "$match": {
          "owner._id": &user._id,
          "_id": { operator: repo_id }
        }
      }
    }
  };

  let order = match direction {
    Direction::Forward => 1,
    _ => -1,
  };
  let stage_sort = doc! { "$sort": { "_id": order } };

  let pipeline = vec![
    stage_match,
    stage_sort,
    doc! { "$limit": limit },
    doc! { "$sort": { "_id": 1 } },
  ];

  let cursor = repo_collection.aggregate(pipeline, None).await?;

  let result = cursor
    .collect::<Vec<Result<Document, MongodbError>>>()
    .await;

  let repositories = result
    .into_iter()
    .map(|document| bson::from_document(document.unwrap()).unwrap())
    .collect::<Vec<Repository>>();

  let repositories = to_cursor_connection(repositories);

  Ok(Some(repositories))
}

pub async fn find_repositories_by_login(
  db: &mongodb::Database,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, MongodbError> {
  let result = find_user(db, login).await?;

  match result {
    Some(user) => find_repositories(db, &user, pagination_arguments).await,
    None => Ok(None),
  }
}
