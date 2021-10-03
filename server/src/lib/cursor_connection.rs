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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Edges<T> {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
  // TODO implement hasNextPage and hasPreviousPage
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Direction {
  Forward,
  Backward,
}

#[derive(Deserialize, Debug)]
pub struct PaginationArguments {
  first: Option<u64>,
  #[serde(default, deserialize_with = "optional_string")]
  after: Option<String>,
  last: Option<u64>,
  #[serde(default, deserialize_with = "optional_string")]
  before: Option<String>,
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

  pub fn parse_args(self) -> Result<(Direction, u64, Option<String>), Error> {
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
  Ok(result.to_owned())
}

type ReferenceFrom<T> = fn(item: &T) -> String;

fn reference_to_cursor(reference: String) -> String {
  base64::encode(reference)
}

fn optional_string<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
  Deserialize::deserialize(d).map(|value: Option<_>| match value {
    None => None,
    Some("" | "null") => None,
    Some(value) => Some(value.to_owned()),
  })
}

#[cfg(test)]
mod cursor_connection_spec {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn should_produce_an_cursor_connection_instance_with_edges() {
    let items = vec!["test 00".to_owned(), "test 01".to_owned()];
    let reference_from = |item: &String| item.clone();
    let cursor_connection = CursorConnection::new(items, reference_from);
    let expected_cursor_connection = CursorConnection {
      page_info: PageInfo {
        start_cursor: Some("dGVzdCAwMA==".to_owned()),
        end_cursor: Some("dGVzdCAwMQ==".to_owned()),
      },
      edges: vec![
        Edges {
          cursor: "dGVzdCAwMA==".to_owned(),
          node: "test 00".to_owned(),
        },
        Edges {
          cursor: "dGVzdCAwMQ==".to_owned(),
          node: "test 01".to_owned(),
        },
      ],
    };

    assert_eq!(cursor_connection, expected_cursor_connection);
  }

  #[test]
  fn should_produce_an_cursor_connection_instance_without_edges() {
    let items: Vec<String> = vec![];
    let reference_from = |item: &String| item.clone();
    let cursor_connection = CursorConnection::new(items, reference_from);
    let expected_cursor_connection = CursorConnection {
      page_info: PageInfo {
        start_cursor: None,
        end_cursor: None,
      },
      edges: vec![],
    };

    assert_eq!(cursor_connection, expected_cursor_connection);
  }

  #[test]
  fn testing_cursor_to_reference() {
    let reference = cursor_to_reference("cmVmXzEyMzQ=".to_owned());
    let expected_reference = "ref_1234".to_owned();

    assert_eq!(reference, Ok(expected_reference));
  }
}

#[cfg(test)]
mod pagination_arguments_spec {
  use super::*;
  use pretty_assertions::assert_eq;

  /// PaginationArguments::is_valid

  #[test]
  fn should_be_valid_not_to_pass_attributes() {
    let arg = PaginationArguments {
      first: None,
      after: None,
      last: None,
      before: None,
    };

    assert!(PaginationArguments::is_valid(&arg) == true);
  }

  #[test]
  fn should_be_valid_pass_only_first() {
    let arg = PaginationArguments {
      first: Some(2),
      after: None,
      last: None,
      before: None,
    };

    assert!(PaginationArguments::is_valid(&arg) == true);
  }

  #[test]
  fn should_be_valid_pass_only_after() {
    let arg = PaginationArguments {
      first: None,
      after: Some("tff3g3fD=".to_owned()),
      last: None,
      before: None,
    };

    assert!(PaginationArguments::is_valid(&arg) == true);
  }

  #[test]
  fn should_be_valid_pass_only_last() {
    let arg = PaginationArguments {
      first: None,
      after: None,
      last: Some(20),
      before: None,
    };

    assert!(PaginationArguments::is_valid(&arg) == true);
  }

  #[test]
  fn should_be_valid_pass_only_before() {
    let arg = PaginationArguments {
      first: None,
      after: None,
      last: None,
      before: Some("tff3g3fD=".to_owned()),
    };

    assert!(PaginationArguments::is_valid(&arg) == true);
  }

  #[test]
  fn should_be_invalid_pass_first_and_last_attributes() {
    let arg = PaginationArguments {
      first: None,
      after: Some("tff3g3fD=".to_owned()),
      last: None,
      before: Some("tff3g3fD=".to_owned()),
    };

    assert!(PaginationArguments::is_valid(&arg) == false);
  }

  #[test]
  fn should_be_invalid_pass_after_and_before_attributes() {
    let arg = PaginationArguments {
      first: Some(2),
      after: None,
      last: Some(3),
      before: None,
    };

    assert!(PaginationArguments::is_valid(&arg) == false);
  }

  /// PaginationArguments::parse_args

  #[test]
  fn should_parse_with_forward_and_default_value_when_there_is_no_attributes() {
    let arg = PaginationArguments {
      first: None,
      after: None,
      last: None,
      before: None,
    };
    let expected_values = Ok((Direction::Forward, 15, None));

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_with_error_when_there_is_first_and_last_attributes() {
    let arg = PaginationArguments {
      first: Some(3),
      after: None,
      last: Some(4),
      before: None,
    };
    let expected_values = Err(Error::InvalidPaginationArguments);

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_with_error_when_there_is_after_and_before_attributes() {
    let arg = PaginationArguments {
      first: None,
      after: Some("tff3g3fD=".to_owned()),
      last: None,
      before: Some("tff3g3fD=".to_owned()),
    };
    let expected_values = Err(Error::InvalidPaginationArguments);

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_when_there_is_first_and_after_attributes() {
    let arg = PaginationArguments {
      first: Some(10),
      after: Some("tff3g3fD=".to_owned()),
      last: None,
      before: None,
    };
    let expected_values = Ok((Direction::Forward, 10, Some("tff3g3fD=".to_owned())));

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_when_there_is_last_and_before_attributes() {
    let arg = PaginationArguments {
      first: None,
      after: None,
      last: Some(10),
      before: Some("tff3g3fD=".to_owned()),
    };
    let expected_values = Ok((Direction::Backward, 10, Some("tff3g3fD=".to_owned())));

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_when_there_is_only_first_attribute() {
    let arg = PaginationArguments {
      first: Some(10),
      after: None,
      last: None,
      before: None,
    };
    let expected_values = Ok((Direction::Forward, 10, None));

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_when_there_is_only_after_attribute() {
    let arg = PaginationArguments {
      first: None,
      after: Some("tff3g3fD=".to_owned()),
      last: None,
      before: None,
    };
    let expected_values = Ok((Direction::Forward, 15, Some("tff3g3fD=".to_owned())));

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_when_there_is_only_last_attribute() {
    let arg = PaginationArguments {
      first: None,
      after: None,
      last: Some(6),
      before: None,
    };
    let expected_values = Ok((Direction::Backward, 6, None));

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_when_there_is_only_before_attribute() {
    let arg = PaginationArguments {
      first: None,
      after: None,
      last: None,
      before: Some("tff3g3fD=".to_owned()),
    };
    let expected_values = Ok((Direction::Backward, 15, Some("tff3g3fD=".to_owned())));

    assert_eq!(arg.parse_args(), expected_values);
  }
}
