use crate::model::utils::*;

#[cfg(test)]
mod describe_parse_cursor {
  use super::*;
  use base64;
  use pretty_assertions::assert_eq;

  #[test]
  fn should_parse_cursor_to_id() {
    let cursor = Some("MTIzNDU=".to_string());
    let id = parse_cursor(cursor);
    let expected_id = 12345;

    assert_eq!(id, Some(expected_id));
  }

  #[test]
  fn should_return_none_when_cursor_is_none() {
    let cursor = None;
    let id = parse_cursor(cursor);

    assert_eq!(id, None);
  }

  #[test]
  fn should_return_none_when_cursor_is_not_a_valid_cursor() {
    let cursor = Some("invalid_cursor".to_string());
    let id = parse_cursor(cursor);

    assert_eq!(id, None);
  }

  #[test]
  fn should_return_none_when_cursor_is_not_a_valid_id() {
    let cursor = Some(base64::encode("invalid_id"));
    let id = parse_cursor(cursor);

    assert_eq!(id, None);
  }
}
