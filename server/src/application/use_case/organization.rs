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

pub async fn find_organization(login: &String) -> Result<Option<Organization>, AppError> {
  let result = database::organization::find_organization_by_login(login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(some) => Ok(some),
  }
}

pub async fn find_people(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, AppError> {
  let result = database::organization::find_organization_by_login(login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(None) => Ok(None),
    Ok(Some(_)) => {
      let result = database::organization::find_people_by_login(login, pagination_arguments).await;
      let cursor = database::organization::organizations_users_to_cursor_connection(login, result).await;
      match cursor {
        Err(err) => Err(AppError::Database(err)),
        Ok(cursor) => Ok(Some(cursor)),
      }
    }
  }
}

pub async fn find_repositories(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, AppError> {
  let result = database::organization::find_organization_by_login(login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(None) => Ok(None),
    Ok(Some(owner)) => {
      let result = database::repository::find_repositories_by_owner_login(&owner.login, pagination_arguments).await;
      let cursor = database::repository::repositories_to_cursor_connection(&owner.login, result).await;
      match cursor {
        Err(err) => Err(AppError::Database(err)),
        Ok(cursor) => Ok(Some(cursor)),
      }
    }
  }
}
