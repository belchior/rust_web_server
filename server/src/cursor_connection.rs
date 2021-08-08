use base64;
use serde::{Deserialize, Serialize};

/*
  forward pagination argument
  first = 3
  after = CURSOR -> 01

  previousPage          nextPage
       |                   |
  |----|----|----|----|----|----|----|----|----|----
  | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09
  |----|----|----|----|----|----|----|----|----|----
       |    |_____________|
     CURSOR        3


  backward pagination argument
  last   = 3
  before = CURSOR -> 08

                 previousPage          nextPage
                      |                   |
  |----|----|----|----|----|----|----|----|----|----
  | 00 | 01 | 02 | 03 | 04 | 05 | 06 | 07 | 08 | 09
  |----|----|----|----|----|----|----|----|----|----
                           |_____________||
                                  3     CURSOR
*/

type ReferenceFrom<T> = fn(item: &T) -> String;

fn reference_to_cursor(reference: String) -> String {
  base64::encode(reference)
}

#[derive(Serialize, Deserialize, Debug)]
struct Edges<T> {
  cursor: String,
  node: T,
}
impl<T> Edges<T> {
  fn items_to_edges(items: Vec<T>, reference_from: ReferenceFrom<T>) -> Vec<Self> {
    items
      .into_iter()
      .map(|item| Self {
        cursor: reference_to_cursor(reference_from(&item)),
        node: item,
      })
      .collect()
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
  // @TODO implement hasNextPage and hasPreviousPage
  start_cursor: Option<String>,
  end_cursor: Option<String>,
}
impl PageInfo {
  fn new<T>(items: &Vec<T>, reference_from: ReferenceFrom<T>) -> Self {
    if items.len() == 0 {
      return Self {
        start_cursor: None,
        end_cursor: None,
      };
    }

    let first_item = items.first().unwrap();
    let last_item = items.last().unwrap();

    Self {
      start_cursor: Some(reference_to_cursor(reference_from(first_item))),
      end_cursor: Some(reference_to_cursor(reference_from(last_item))),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CursorConnection<T> {
  page_info: PageInfo,
  edges: Vec<Edges<T>>,
}
impl<T> CursorConnection<T> {
  pub fn new(items: Vec<T>, reference_from: ReferenceFrom<T>) -> Self {
    Self {
      page_info: PageInfo::new(&items, reference_from),
      edges: Edges::items_to_edges(items, reference_from),
    }
  }
}

#[derive(Debug)]
pub enum Direction {
  Forward,
  Backward,
}

#[derive(Deserialize, Debug)]
pub struct PaginationArguments {
  pub first: Option<i64>,
  pub after: Option<String>,
  pub last: Option<i64>,
  pub before: Option<String>,
}
impl PaginationArguments {
  pub fn parse_args(self) -> Result<(Direction, i64, Option<String>), Error> {
    use Direction::*;

    match &self {
      PaginationArguments {
        first: Some(limit),
        after: Some(cursor),
        ..
      } => Ok((Forward, limit.clone(), Some(cursor.clone()))),

      PaginationArguments {
        first: Some(limit),
        after: None,
        ..
      } => Ok((Forward, limit.clone(), None)),

      PaginationArguments {
        last: Some(limit),
        before: Some(cursor),
        ..
      } => Ok((Backward, limit.clone(), Some(cursor.clone()))),

      PaginationArguments {
        last: Some(limit),
        after: None,
        ..
      } => Ok((Backward, limit.clone(), None)),

      _ => Err(Error::InvalidPaginationArguments),
    }
  }
}

#[derive(Debug)]
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
  Ok(result.to_owned())
}
