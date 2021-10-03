use super::cursor_connection::*;
use pretty_assertions::assert_eq;

// CursorConnection::new

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

// PaginationArguments::is_valid

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

// PaginationArguments::parse_args

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
