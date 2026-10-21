use chrono::NaiveDateTime;
use diesel::result::{DatabaseErrorKind, Error};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::database::in_memory_connection;
use crate::regex_storage::{
  create_regex, delete_regex, does_regex_name_exist, get_saved_regexes, retrieve_regex, save_regex, update_regex_name
};
use crate::schema::saved_regexes;
use crate::workspace_storage::create_workspace;

fn names(connection: &mut SqliteConnection) -> Vec<String> {
  get_saved_regexes(connection).unwrap().into_iter().map(|saved| saved.regex_name).collect()
}

fn backdate(connection: &mut SqliteConnection, regex_name: &str) {
  diesel::update(saved_regexes::table.filter(saved_regexes::regex_name.eq(regex_name)))
    .set(saved_regexes::date_of_last_update.eq(NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()))
    .execute(connection)
    .unwrap();
}

#[test]
fn test_new_regex_is_empty() {
  let connection = &mut in_memory_connection();
  create_regex(connection, "new regex").unwrap();

  assert!(does_regex_name_exist(connection, "new regex").unwrap());
  assert_eq!(names(connection), vec!["new regex"]);
  let saved = retrieve_regex(connection, "new regex").unwrap();
  assert_eq!(saved.regex_name, "new regex");
  assert_eq!(saved.regex, "");
}

#[test]
fn test_saved_regex_comes_back_the_same() {
  let connection = &mut in_memory_connection();
  create_regex(connection, "round trip").unwrap();

  // Saved as typed, even when it isn't finished
  for regex in ["a(b+c)*", "ϵ | é*", "(a+"] {
    save_regex(connection, "round trip", regex).unwrap();
    assert_eq!(retrieve_regex(connection, "round trip").unwrap().regex, regex);
  }
}

#[test]
fn test_saving_updates_the_date() {
  let connection = &mut in_memory_connection();
  create_regex(connection, "dated").unwrap();
  backdate(connection, "dated");
  assert_eq!(retrieve_regex(connection, "dated").unwrap().date_of_last_update, "2024-01-01 00:00:00");

  save_regex(connection, "dated", "ab").unwrap();

  let date_of_last_update = retrieve_regex(connection, "dated").unwrap().date_of_last_update;
  assert!(date_of_last_update.as_str() > "2024-01-01 00:00:00");
  assert!(NaiveDateTime::parse_from_str(&date_of_last_update, "%Y-%m-%d %H:%M:%S").is_ok());
}

#[test]
fn test_regexes_are_listed_oldest_first_with_what_they_are() {
  let connection = &mut in_memory_connection();
  for regex_name in ["b", "c", "a"] {
    create_regex(connection, regex_name).unwrap();
  }
  save_regex(connection, "c", "c*").unwrap();

  let saved = get_saved_regexes(connection).unwrap();
  assert_eq!(names(connection), vec!["b", "c", "a"]);
  assert_eq!(saved[1].regex, "c*");
  assert_eq!(saved[1], retrieve_regex(connection, "c").unwrap());
}

#[test]
fn test_regexes_are_sent_in_the_frontend_s_shape() {
  let connection = &mut in_memory_connection();
  create_regex(connection, "shape").unwrap();
  save_regex(connection, "shape", "a*").unwrap();
  backdate(connection, "shape");

  assert_eq!(serde_json::to_value(retrieve_regex(connection, "shape").unwrap()).unwrap(), serde_json::json!({
    "regex_name": "shape", "regex": "a*", "date_of_last_update": "2024-01-01 00:00:00"
  }));
}

#[test]
fn test_renaming_keeps_the_regex() {
  let connection = &mut in_memory_connection();
  create_regex(connection, "old name").unwrap();
  save_regex(connection, "old name", "a+b").unwrap();

  update_regex_name(connection, "old name", "new name").unwrap();

  assert!(!does_regex_name_exist(connection, "old name").unwrap());
  assert_eq!(retrieve_regex(connection, "new name").unwrap().regex, "a+b");
}

#[test]
fn test_deleting_removes_only_that_regex() {
  let connection = &mut in_memory_connection();
  create_regex(connection, "to delete").unwrap();
  create_regex(connection, "to keep").unwrap();

  delete_regex(connection, "to delete").unwrap();

  assert!(!does_regex_name_exist(connection, "to delete").unwrap());
  assert_eq!(names(connection), vec!["to keep"]);
}

#[test]
fn test_missing_regexes_are_errors() {
  let connection = &mut in_memory_connection();

  assert_eq!(retrieve_regex(connection, "missing").unwrap_err(), Error::NotFound);
  assert_eq!(save_regex(connection, "missing", "a").unwrap_err(), Error::NotFound);
  assert_eq!(delete_regex(connection, "missing").unwrap_err(), Error::NotFound);
  assert_eq!(update_regex_name(connection, "missing", "other").unwrap_err(), Error::NotFound);
  assert!(!does_regex_name_exist(connection, "missing").unwrap());
}

#[test]
fn test_names_must_be_unique() {
  let connection = &mut in_memory_connection();
  create_regex(connection, "taken").unwrap();
  create_regex(connection, "other").unwrap();

  assert!(matches!(
    create_regex(connection, "taken").unwrap_err(),
    Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
  ));
  assert!(matches!(
    update_regex_name(connection, "other", "taken").unwrap_err(),
    Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
  ));
  assert_eq!(names(connection), vec!["taken", "other"]);
}

// They're listed apart on the dashboard, so a regex and a workspace can share a name
#[test]
fn test_regexes_and_workspaces_are_named_separately() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "shared name").unwrap();

  create_regex(connection, "shared name").unwrap();

  assert_eq!(names(connection), vec!["shared name"]);
  assert_eq!(crate::workspace_storage::get_saved_workspaces(connection).unwrap(), vec!["shared name"]);
}
