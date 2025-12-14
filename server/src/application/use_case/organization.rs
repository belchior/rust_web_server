use crate::{
  application::error::AppError,
  infrastructure::database::{
    self,
    cursor_connection::{CursorConnection, PaginationArguments},
    organization::Organization,
    repository::Repository,
    user::User,
  },
};

pub async fn find_organization(db: &database::DBConnection, login: &String) -> Result<Option<Organization>, AppError> {
  let result = database::organization::find_organization_by_login(db, login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(some) => Ok(some),
  }
}

pub async fn find_people(
  db: &database::DBConnection,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, AppError> {
  let result = database::organization::find_organization_by_login(db, login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(None) => Ok(None),
    Ok(Some(_)) => {
      let result = database::organization::find_people_by_login(db, login, pagination_arguments).await;
      let cursor = database::organization::organizations_users_to_cursor_connection(db, login, result).await;
      match cursor {
        Err(err) => Err(AppError::Database(err)),
        Ok(cursor) => Ok(Some(cursor)),
      }
    }
  }
}

pub async fn find_repositories(
  db: &database::DBConnection,
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, AppError> {
  let result = database::organization::find_organization_by_login(db, login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(None) => Ok(None),
    Ok(Some(owner)) => {
      let result = database::repository::find_repositories_by_owner_login(db, &owner.login, pagination_arguments).await;
      let cursor = database::repository::repositories_to_cursor_connection(db, &owner.login, result).await;
      match cursor {
        Err(err) => Err(AppError::Database(err)),
        Ok(cursor) => Ok(Some(cursor)),
      }
    }
  }
}
