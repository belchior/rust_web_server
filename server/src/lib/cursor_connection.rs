use base64;
use serde::{Deserialize, Deserializer, Serialize};

///
///   forward pagination argument
///   first = 3
///   after = CURSOR -> 01
///
///   previousPage          nextPage
///        |                   |
///   |----|----|----|----|----|----|----|----|----|----
///   | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09
///   |----|----|----|----|----|----|----|----|----|----
///        |    |_____________|
///      CURSOR        3
///
///   backward pagination argument
///   last   = 3
///   before = CURSOR -> 08
///
///                  previousPage          nextPage
///                       |                   |
///   |----|----|----|----|----|----|----|----|----|----
///   | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09
///   |----|----|----|----|----|----|----|----|----|----
///                            |_____________||
///                                   3     CURSOR
///

pub type ReferenceFrom<T> = fn(item: &T) -> String;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Edges<T> {
  pub cursor: String,
  pub node: T,
}
impl<T> Edges<T> {
  fn into_edges(items: Vec<T>, reference_from: ReferenceFrom<T>) -> Vec<Self> {
    items
      .into_iter()
      .map(|item| Self {
        cursor: reference_to_cursor(reference_from(&item)),
        node: item,
      })
      .collect()
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
  pub has_previous_page: bool,
  pub has_next_page: bool,
  pub start_cursor: Option<String>,
  pub end_cursor: Option<String>,
}
impl PageInfo {
  fn new<T>(items: &Vec<T>, has_next_page: bool, has_previous_page: bool, reference_from: ReferenceFrom<T>) -> Self {
    if items.len() == 0 {
      return Self {
        has_previous_page: false,
        has_next_page: false,
        start_cursor: None,
        end_cursor: None,
      };
    }

    let first_item = items.first().unwrap();
    let last_item = items.last().unwrap();

    Self {
      has_previous_page,
      has_next_page,
      start_cursor: Some(reference_to_cursor(reference_from(first_item))),
      end_cursor: Some(reference_to_cursor(reference_from(last_item))),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CursorConnection<T> {
  pub page_info: PageInfo,
  pub edges: Vec<Edges<T>>,
}
impl<T> CursorConnection<T> {
  pub fn new(items: Vec<T>, has_next_page: bool, has_previous_page: bool, reference_from: ReferenceFrom<T>) -> Self {
    Self {
      page_info: PageInfo::new(&items, has_next_page, has_previous_page, reference_from),
      edges: Edges::into_edges(items, reference_from),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
  Forward,
  Backward,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationArguments {
  pub first: Option<u32>,
  #[serde(default, deserialize_with = "optional_string")]
  pub after: Option<String>,
  pub last: Option<u32>,
  #[serde(default, deserialize_with = "optional_string")]
  pub before: Option<String>,
}
impl PaginationArguments {
  pub fn is_valid(arg: &Self) -> bool {
    match &arg {
      Self {
        first: Some(_),
        last: Some(_),
        ..
      } => false,

      Self {
        after: Some(_),
        before: Some(_),
        ..
      } => false,

      _ => true,
    }
  }

  pub fn parse_args(self) -> Result<(Direction, u32, Option<String>), Error> {
    use Direction::*;
    let default_limit = 15;

    match &self {
      Self {
        first: None,
        after: None,
        last: None,
        before: None,
      } => Ok((Forward, default_limit, None)),

      Self {
        first: Some(_),
        last: Some(_),
        ..
      } => Err(Error::InvalidPaginationArguments),

      Self {
        after: Some(_),
        before: Some(_),
        ..
      } => Err(Error::InvalidPaginationArguments),

      Self {
        first: Some(limit),
        after: Some(cursor),
        ..
      } => Ok((Forward, limit.clone(), Some(cursor.clone()))),

      Self {
        last: Some(limit),
        before: Some(cursor),
        ..
      } => Ok((Backward, limit.clone(), Some(cursor.clone()))),

      Self { first: Some(limit), .. } => Ok((Forward, limit.clone(), None)),

      Self {
        after: Some(cursor), ..
      } => Ok((Forward, default_limit, Some(cursor.clone()))),

      Self { last: Some(limit), .. } => Ok((Backward, limit.clone(), None)),

      Self {
        before: Some(cursor), ..
      } => Ok((Backward, default_limit, Some(cursor.clone()))),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum Error {
  DecodeError(base64::DecodeError),
  Utf8Error(std::str::Utf8Error),
  InvalidPaginationArguments,
}
impl From<base64::DecodeError> for Error {
  fn from(error: base64::DecodeError) -> Self {
    Error::DecodeError(error)
  }
}
impl From<std::str::Utf8Error> for Error {
  fn from(error: std::str::Utf8Error) -> Self {
    Error::Utf8Error(error)
  }
}

pub fn cursor_to_reference(cursor: String) -> Result<String, Error> {
  let result = base64::decode(cursor)?;
  let result = std::str::from_utf8(&result)?;
  Ok(result.to_string())
}

fn reference_to_cursor(reference: String) -> String {
  base64::encode(reference)
}

fn optional_string<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
  Deserialize::deserialize(d).map(|value: Option<_>| match value {
    None => None,
    Some("" | "null") => None,
    Some(value) => Some(value.to_string()),
  })
}
