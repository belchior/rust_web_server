use crate::{
  application::AppError,
  infrastructure::database::{
    self,
    cursor_connection::{CursorConnection, PaginationArguments},
    organization::Organization,
    repository::Repository,
    user::User,
  },
};

pub async fn find_user(login: &String) -> Result<Option<User>, AppError> {
  let result = database::user::find_user_by_login(login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(some) => Ok(some),
  }
}

pub async fn find_organizations(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Organization>>, AppError> {
  let result = database::user::find_user_by_login(login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(None) => Ok(None),
    Ok(Some(_)) => {
      let result = database::user::find_organizations_by_user_login(login, pagination_arguments).await;
      let cursor = database::user::users_organizations_to_cursor_connection(login, result).await;
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
  let result = database::user::find_user_by_login(login).await;

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

pub async fn find_starred_repositories(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<Repository>>, AppError> {
  let result = database::user::find_user_by_login(login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(None) => Ok(None),
    Ok(Some(owner)) => {
      let result = database::user::find_starred_repositories_by_user_login(login, pagination_arguments).await;
      let cursor = database::repository::repositories_to_cursor_connection(&owner.login, result).await;
      match cursor {
        Err(err) => Err(AppError::Database(err)),
        Ok(cursor) => Ok(Some(cursor)),
      }
    }
  }
}

pub async fn find_followers(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, AppError> {
  let result = database::user::find_user_by_login(login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(None) => Ok(None),
    Ok(Some(_)) => {
      let result = database::user::find_followers_by_login(login, pagination_arguments).await;
      let cursor = database::user::followers_to_cursor_connection(login, result).await;
      match cursor {
        Err(err) => Err(AppError::Database(err)),
        Ok(cursor) => Ok(Some(cursor)),
      }
    }
  }
}

pub async fn find_following(
  login: &String,
  pagination_arguments: PaginationArguments,
) -> Result<Option<CursorConnection<User>>, AppError> {
  let result = database::user::find_user_by_login(login).await;

  match result {
    Err(err) => Err(AppError::Database(err)),
    Ok(None) => Ok(None),
    Ok(Some(_)) => {
      let result = database::user::find_following_by_login(login, pagination_arguments).await;
      let cursor = database::user::following_to_cursor_connection(login, result).await;
      match cursor {
        Err(err) => Err(AppError::Database(err)),
        Ok(cursor) => Ok(Some(cursor)),
      }
    }
  }
}
