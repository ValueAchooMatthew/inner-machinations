use chrono::NaiveDateTime;
use diesel::{query_builder::QueryId, Connection, Identifiable, Insertable, Queryable, QueryableByName, Selectable, SqliteConnection};
use serde::{Deserialize, Serialize};

pub fn establish_connection() -> SqliteConnection {
  SqliteConnection::establish(&"mydb.sqlite3")
    .unwrap_or_else(|_| panic!("Error connecting to database"))
}

// REMEMBER ORDER OF FIELDS IN STRUCTS MATTER
#[derive(Queryable, Selectable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
  pub id: i32,
  pub email: String,
  pub password: String,
  pub verified: bool,
  pub number_of_untitled_regular_automata_workspaces: i32,
  pub number_of_untitled_regex_workspaces: i32,
  pub code: Option<String>
}

#[derive(Queryable, Insertable, QueryableByName, QueryId, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::saved_regular_automata_workspaces)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct SavedRegularAutomataWorkspace {
  pub id: i32,
  pub user_id: i32,
  pub workspace_name: String,
  pub type_of_automata: TypeOfAutomata,
  pub date_of_last_update: NaiveDateTime,
  pub alphabet: String,
  pub should_show_string_traversal: bool,
  pub should_strict_check: bool,
  pub default_connection_character: String
}

#[derive(Queryable, Insertable, QueryableByName, QueryId, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::saved_regex_workspaces)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct SavedRegexWorkspace {
  pub id: i32,
  pub user_id: i32,
  pub regex_name: String,
  pub regex: String,
  pub date_of_last_update: NaiveDateTime,
}

#[derive(Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::saved_states)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct SavedState {
  pub id: i32,
  pub workspace_id: i32,
  pub position: String,
  pub is_start: bool,
  pub is_final: bool
}

#[derive(Queryable, Selectable, QueryableByName, Insertable)]
#[diesel(table_name = crate::schema::saved_regular_automata_connections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct SavedConnection {
  pub id: i32,
  pub workspace_id: i32,
  pub start_point: String,
  pub control_point_one: String,
  pub control_point_two: String,
  pub end_point: String,
  pub connection_character: String
}

#[derive(Debug, Deserialize, Serialize, diesel_derive_enum::DbEnum)]
#[DbValueStyle = "UPPERCASE"]
pub enum TypeOfAutomata {
  DFA,
  NFA
}