use crate::{
  application::AppError,
  infrastructure::database::{self, organization::Organization, user::User},
};
use futures::join;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Profile {
  User(User),
  Organization(Organization),
}

pub async fn find_profile(db: &database::DBConnection, login: &String) -> Result<Option<Profile>, AppError> {
  let (user, organization) = join!(
    database::user::find_user_by_login(db, login),
    database::organization::find_organization_by_login(db, login)
  );

  let result = match (user, organization) {
    (Err(err), _) | (_, Err(err)) => Err(AppError::Database(err)),
    (Ok(Some(user)), _) => Ok(Some(Profile::User(user))),
    (_, Ok(Some(org))) => Ok(Some(Profile::Organization(org))),
    _ => Ok(None),
  };

  result
}
