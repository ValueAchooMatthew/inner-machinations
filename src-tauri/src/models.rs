use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_builder::QueryId;
use serde::{Deserialize, Serialize};

// Any instances of a character being typed as a string is done due to the fact the deserialized datatype coming from the
// type scripty back-end, despite being a single character is always of type string since typescript does not have a character data type

// REMEMBER ORDER OF FIELDS IN STRUCTS MATTER
#[derive(Queryable, Insertable, QueryableByName, QueryId, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::saved_workspaces)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct SavedWorkspace {
  pub id: i32,
  pub workspace_name: String,
  pub type_of_automata: TypeOfAutomata,
  pub date_of_last_update: NaiveDateTime,
  pub alphabet: String,
  pub should_show_string_traversal: bool,
  pub should_strict_check: bool,
  pub default_connection_character: String
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::saved_states)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SavedState {
  pub id: i32,
  pub workspace_id: i32,
  pub state_id: i32,
  pub x: i32,
  pub y: i32,
  pub is_start: bool,
  pub is_final: bool
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::saved_connections)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SavedConnection {
  pub id: i32,
  pub workspace_id: i32,
  pub connection_id: i32,
  pub from_state: i32,
  pub to_state: i32,
  pub connection_character: String,
  pub control_point_one_x: i32,
  pub control_point_one_y: i32,
  pub control_point_two_x: i32,
  pub control_point_two_y: i32
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::saved_regexes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SavedRegex {
  pub id: i32,
  pub regex_name: String,
  pub regex: String,
  pub date_of_last_update: NaiveDateTime
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Coordinate {
  pub x: i32,
  pub y: i32
}

// The automaton as the workbench draws it, which is how the frontend sends it and gets it back.
// The field names follow the frontend's

// A state's id stays the same when it moves
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StateData {
  pub id: i32,
  pub position: Coordinate,
  pub is_start: bool,
  pub is_final: bool
}

// A connection joins two states, given by their ids, and is drawn as a curve through two control points.
// Its character is a single character, or ϵ for an epsilon connection
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionData {
  pub id: i32,
  pub from: i32,
  pub to: i32,
  pub character: String,
  pub control_points: [Coordinate; 2]
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct AutomatonData {
  pub states: Vec<StateData>,
  pub connections: Vec<ConnectionData>
}

#[derive(Debug, Deserialize, Serialize, diesel_derive_enum::DbEnum)]
#[DbValueStyle = "UPPERCASE"]
pub enum TypeOfAutomata {
  DFA,
  NFA
}

#[derive(Debug, Serialize)]
pub struct WorkspaceData {
  pub automaton: AutomatonData,
  pub type_of_automata: TypeOfAutomata,
  pub date_of_last_update: String,
  pub alphabet: Vec<String>,
  pub should_strict_check: bool,
  pub should_show_string_traversal: bool,
  pub default_connection_character: String
}

// A saved regular expression, as typed, which needn't be one that can be built yet
#[derive(Debug, Serialize, PartialEq)]
pub struct RegexData {
  pub regex_name: String,
  pub regex: String,
  pub date_of_last_update: String
}
