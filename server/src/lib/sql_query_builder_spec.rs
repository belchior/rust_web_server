use crate::lib::sql_query_builder::SelectBuilder;
use pretty_assertions::assert_eq;

#[test]
fn testing_select_from_people() {
  let query = SelectBuilder::new()
    .select("*")
    .from("people")
    .inner_join("users_organizations uo", "uo.organization_login = o.login")
    .inner_join("users u", "u.login = uo.user_login")
    .where_clause("login < $2")
    .and("id > $id")
    .order_by("joined_at asc")
    .limit("3")
    .as_string();

  let expected_query = "\
    SELECT * \
    FROM people \
    INNER JOIN users_organizations uo ON (uo.organization_login = o.login) \
    INNER JOIN users u ON (u.login = uo.user_login) \
    WHERE login < $2 AND id > $id \
    ORDER BY joined_at asc \
    LIMIT 3";

  assert_eq!(query, expected_query);
}

#[test]
fn testing_statement_with() {
  let query = SelectBuilder::new()
    .with(
      "people",
      SelectBuilder::new().select("*").from("users").where_clause("id = $1"),
    )
    .select("*")
    .from("people")
    .order_by("id DESC")
    .as_string();

  let expected_query = "\
    WITH people AS ( SELECT * FROM users WHERE id = $1 ) \
    SELECT * FROM people ORDER BY id DESC\
  ";

  assert_eq!(query, expected_query);
}
