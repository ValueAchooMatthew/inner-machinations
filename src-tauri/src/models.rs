use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_builder::QueryId;

// Any instances of a character being typed as a string is done due to the fact the deserialized datatype coming from the
// type scripty back-end, despite being a single character is always of type string since typescript does not have a character data type

#[derive(Queryable, Selectable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
use std::collections::HashSet;
use std::hash::Hash;
#[derive(Debug, Deserialize, Serialize, Clone, Eq)]
pub struct State { 
  position: Coordinate,
  states_connected_to: HashMap<String, HashSet<String>>,
  is_start: bool,
  is_final: bool,
  element: String
}

pub trait SmartState {
  fn new(position: Coordinate, is_start: bool, is_final: bool) -> Self;
  fn add_connection(&mut self, connection_character: &str, state_key_to_connect: impl Into<String>);
  fn get_connections_by_character(&self, connection_character: &str) -> Option<&HashSet<String>>;
  fn get_all_connections(&self) -> &HashMap<String, HashSet<String>>;
  fn get_all_connections_mut(&mut self) -> &mut HashMap<String, HashSet<String>>;
  fn set_all_connections(&mut self, connections: HashMap<String, HashSet<String>>);
  // Add better error typing in future
  fn remove_connection_by_character(&mut self, connection_character: &str, state_key_to_remove: impl Into<String>) -> Result<(), ()>;
  fn get_all_connected_state_keys(&self) -> HashSet<&String>;
  fn is_final(&self) -> bool;
  fn make_final(&mut self);
  fn is_start(&self) -> bool;
  fn make_start(&mut self);
  fn get_position(&self) -> Coordinate;
  fn get_position_as_string(&self) -> String;
  fn remove_all_connections_by_character(&mut self, connection_character: &str);
}

impl SmartState for State {

  fn new(position: Coordinate, is_start: bool, is_final: bool) -> Self {
    return State {
      position,
      states_connected_to: HashMap::new(),
      is_start,
      is_final,
      element: String::from("State")
    };
  }

  fn add_connection(&mut self, connection_character: &str, state_key_to_connect: impl Into<String>) {
    
    if let Some(currently_connected_state_keys) = self.states_connected_to.get_mut(connection_character) {
      currently_connected_state_keys.insert(state_key_to_connect.into());
    } else {
      self.states_connected_to.insert(connection_character.to_owned(), HashSet::from([state_key_to_connect.into()]));
    }
  }
  
  fn get_all_connected_state_keys(&self) -> HashSet<&String> {

    let mut all_connected_state_keys = HashSet::new();

    for all_state_keys in self.states_connected_to.values() {
      for key in all_state_keys {
        all_connected_state_keys.insert(key);
      }
    }

    return all_connected_state_keys;

  }

  fn get_all_connections(&self) -> &HashMap<std::string::String, HashSet<std::string::String>> {
    return &self.states_connected_to; 
  }

  fn get_all_connections_mut(&mut self) -> &mut HashMap<std::string::String, HashSet<std::string::String>> {
    return &mut self.states_connected_to; 
  }
  
  fn is_final(&self) -> bool { 
    return self.is_final;
  }

  fn make_final(&mut self) {
    self.is_final = true;
  }

  fn is_start(&self) -> bool {
    return self.is_start;
  }

  fn make_start(&mut self) {
    self.is_start = true;
  }
  
  fn get_connections_by_character(&self, connection_character: &str) -> Option<&HashSet<String>> {
    return self.states_connected_to
      .get(connection_character);
  }

  fn get_position(&self) -> Coordinate {
    return self.position;
  }

  fn get_position_as_string(&self) -> String {
    return self.position.into();
  }

  fn set_all_connections(&mut self, connections: HashMap<String, HashSet<String>>) {
    self.states_connected_to = connections;
  }
  
  // Make make customer error type in future for this case
  fn remove_connection_by_character(&mut self, connection_character: &str, state_key_to_remove: impl Into<String>) -> Result<(), ()> {
    let connected_states_by_character = self.states_connected_to
      .get_mut(connection_character)
      .ok_or(())?;

    connected_states_by_character.remove(&state_key_to_remove.into());

    Ok(())

  }

  fn remove_all_connections_by_character(&mut self, connection_character: &str) {
    self.states_connected_to.remove(connection_character);
  }

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

// We're able to do this as we never have two states assigned to the same position,
// thus there is a unique position for every state that exists
impl Hash for State {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.position.hash(state);
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