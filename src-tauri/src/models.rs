use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_builder::QueryId;

// Any instances of a character being typed as a string is done due to the fact the deserialized datatype coming from the
// type scripty back-end, despite being a single character is always of type string since typescript does not have a character data type

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub password: String,
  pub verified: bool,
  pub code: Option<String>
}

#[derive(Queryable, Insertable, QueryableByName, QueryId, Selectable)]
#[diesel(table_name = crate::schema::saved_workspaces)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct SavedWorkspace {
  pub id: i32,
  pub user_id: i32,
  pub workspace_name: String,
  pub type_of_automata: TypeOfAutomata,
  pub date_of_last_update: NaiveDateTime
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
#[diesel(table_name = crate::schema::saved_connections)]
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

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct State { 
  pub position: Coordinate,
  pub states_connected_to: HashMap<String, Vec<String>>,
  pub is_start: bool,
  pub is_final: bool,
  pub element: String
}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.position == other.position
  }
}

impl Into<String> for State {
  fn into(self) -> String {
    self.position.into()
  }
} 

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connection {
    pub curve: BezierCurve,
    pub connection_character: String,
    pub element: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BezierCurve {
  pub start_point: Coordinate,
  pub control_point_one: Coordinate,
  pub control_point_two: Coordinate,
  pub end_point: Coordinate
}

impl Into<String> for Coordinate {
  fn into(self) -> String {
    let mut built_string = self.x.to_string();
    built_string.push(',');
    // Using reference here since push_str takes in &str as param
    built_string.push_str(&self.y.to_string());
    return built_string;
  }
}

impl TryFrom<String> for Coordinate {

  type Error = ();

  fn try_from(value: String) -> Result<Coordinate, ()> {

    let processed_strings: Vec<&str> = value.split(",").collect();

    if processed_strings.len() != 2 {
      return Err(());
    }

    let coordinate = Coordinate {
      x: processed_strings
        .get(0)
        .unwrap()
        .parse::<i32>()
        .unwrap(),

      y: processed_strings
        .get(1)
        .unwrap()
        .parse::<i32>()
        .unwrap()
    };
    return Ok(coordinate);
  }
}

impl TryFrom<&String> for Coordinate {

  type Error = ();

  fn try_from(value: &String) -> Result<Coordinate, ()> {

    let processed_strings: Vec<&str> = value.split(",").collect();

    if processed_strings.len() != 2 {
      return Err(());
    }

    let coordinate = Coordinate {
      x: processed_strings
        .get(0)
        .unwrap()
        .parse::<i32>()
        .unwrap(),

      y: processed_strings
        .get(1)
        .unwrap()
        .parse::<i32>()
        .unwrap()
    };

    return Ok(coordinate);
  }
}

#[derive(Debug, Deserialize, Serialize, diesel_derive_enum::DbEnum)]
#[DbValueStyle = "UPPERCASE"]
pub enum TypeOfAutomata {
  DFA,
  NFA
}