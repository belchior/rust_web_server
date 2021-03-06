use super::cursor_connection::*;
use pretty_assertions::assert_eq;

#[test]
fn testing_cursor_to_reference() {
  let reference = cursor_to_reference("cmVmXzEyMzQ=".to_string());
  let expected_reference = "ref_1234".to_string();

  assert_eq!(reference, Ok(expected_reference));
}

#[cfg(test)]
mod describe_cursor_connection_new {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn should_produce_an_cursor_connection_instance_with_edges() {
    let items = vec!["test 00".to_string(), "test 01".to_string()];
    let reference_from = |item: &String| item.clone();
    let cursor_connection = CursorConnection::new(items, false, false, reference_from);
    let expected_cursor_connection = CursorConnection {
      page_info: PageInfo {
        has_next_page: false,
        has_previous_page: false,
        start_cursor: Some("dGVzdCAwMA==".to_string()),
        end_cursor: Some("dGVzdCAwMQ==".to_string()),
      },
      edges: vec![
        Edges {
          cursor: "dGVzdCAwMA==".to_string(),
          node: "test 00".to_string(),
        },
        Edges {
          cursor: "dGVzdCAwMQ==".to_string(),
          node: "test 01".to_string(),
        },
      ],
    };

    assert_eq!(cursor_connection, expected_cursor_connection);
  }

  #[test]
  fn should_produce_an_cursor_connection_instance_without_edges() {
    let items: Vec<String> = vec![];
    let reference_from = |item: &String| item.clone();
    let cursor_connection = CursorConnection::new(items, false, false, reference_from);
    let expected_cursor_connection = CursorConnection {
      page_info: PageInfo {
        has_next_page: false,
        has_previous_page: false,
        start_cursor: None,
        end_cursor: None,
      },
      edges: vec![],
    };

    assert_eq!(cursor_connection, expected_cursor_connection);
  }
}

#[cfg(test)]
mod describe_pagination_arguments_is_valid {
  use super::*;

  #[test]
  fn should_be_valid_not_pass_attributes() {
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
      after: Some("tff3g3fD=".to_string()),
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
      before: Some("tff3g3fD=".to_string()),
    };

    assert!(PaginationArguments::is_valid(&arg) == true);
  }

  #[test]
  fn should_be_invalid_pass_first_and_last_attributes() {
    let arg = PaginationArguments {
      first: None,
      after: Some("tff3g3fD=".to_string()),
      last: None,
      before: Some("tff3g3fD=".to_string()),
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
}

#[cfg(test)]
mod describe_pagination_arguments_parse_args {
  use super::*;
  use pretty_assertions::assert_eq;

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
      after: Some("tff3g3fD=".to_string()),
      last: None,
      before: Some("tff3g3fD=".to_string()),
    };
    let expected_values = Err(Error::InvalidPaginationArguments);

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_when_there_is_first_and_after_attributes() {
    let arg = PaginationArguments {
      first: Some(10),
      after: Some("tff3g3fD=".to_string()),
      last: None,
      before: None,
    };
    let expected_values = Ok((Direction::Forward, 10, Some("tff3g3fD=".to_string())));

    assert_eq!(arg.parse_args(), expected_values);
  }

  #[test]
  fn should_parse_when_there_is_last_and_before_attributes() {
    let arg = PaginationArguments {
      first: None,
      after: None,
      last: Some(10),
      before: Some("tff3g3fD=".to_string()),
    };
    let expected_values = Ok((Direction::Backward, 10, Some("tff3g3fD=".to_string())));

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
      after: Some("tff3g3fD=".to_string()),
      last: None,
      before: None,
    };
    let expected_values = Ok((Direction::Forward, 15, Some("tff3g3fD=".to_string())));

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
      before: Some("tff3g3fD=".to_string()),
    };
    let expected_values = Ok((Direction::Backward, 15, Some("tff3g3fD=".to_string())));

    assert_eq!(arg.parse_args(), expected_values);
  }
}
