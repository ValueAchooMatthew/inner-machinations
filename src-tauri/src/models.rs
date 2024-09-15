use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::query_builder::QueryId;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

use crate::establish_connection;
use crate::schema::saved_states;
use crate::schema::saved_regular_automata_connections;
use crate::schema::users;
// Any instances of a character being typed as a string is done due to the fact the deserialized datatype coming from the
// type scripty back-end, despite being a single character is always of type string since typescript does not have a character data type

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

#[derive(Debug, Deserialize, Serialize, Clone, Eq)]
pub struct State { 
  position: Coordinate,
  states_connected_to: HashMap<String, HashSet<String>>,
  is_start: bool,
  is_final: bool,
  element: String
}

impl State {

  pub fn new(position: Coordinate, is_start: bool, is_final: bool) -> Self {
    return State {
      position,
      states_connected_to: HashMap::new(),
      is_start,
      is_final,
      element: String::from("State")
    };
  }

  pub fn add_connection(&mut self, connection_character: &str, state_key_to_connect: impl Into<String>) {
    
    if let Some(currently_connected_state_keys) = self.states_connected_to.get_mut(connection_character) {
      currently_connected_state_keys.insert(state_key_to_connect.into());
    } else {
      self.states_connected_to.insert(connection_character.to_owned(), HashSet::from([state_key_to_connect.into()]));
    }
  }
  
  pub fn get_all_connected_state_keys(&self) -> HashSet<&String> {

    let mut all_connected_state_keys = HashSet::new();

    for all_state_keys in self.states_connected_to.values() {
      for key in all_state_keys {
        all_connected_state_keys.insert(key);
      }
    }

    return all_connected_state_keys;

  }

  pub fn get_all_connections(&self) -> &HashMap<std::string::String, HashSet<std::string::String>> {
    return &self.states_connected_to; 
  }

  pub fn get_all_connections_mut(&mut self) -> &mut HashMap<std::string::String, HashSet<std::string::String>> {
    return &mut self.states_connected_to; 
  }
  
  pub fn is_final(&self) -> bool { 
    return self.is_final;
  }

  pub fn make_final(&mut self) {
    self.is_final = true;
  }

  pub fn is_start(&self) -> bool {
    return self.is_start;
  }

  pub fn make_start(&mut self) {
    self.is_start = true;
  }
  
  pub fn get_connections_by_character(&self, connection_character: &str) -> Option<&HashSet<String>> {
    return self.states_connected_to
      .get(connection_character);
  }

  pub fn get_position(&self) -> Coordinate {
    return self.position;
  }

  pub fn get_position_as_string(&self) -> String {
    return self.position.into();
  }

  pub fn set_all_connections(&mut self, connections: HashMap<String, HashSet<String>>) {
    self.states_connected_to = connections;
  }
  
  // Make make customer error type in future for this case
  pub fn remove_connection_by_character(&mut self, connection_character: &str, state_key_to_remove: impl Into<String>) -> Result<(), ()> {
    let connected_states_by_character = self.states_connected_to
      .get_mut(connection_character)
      .ok_or(())?;

    connected_states_by_character.remove(&state_key_to_remove.into());

    Ok(())

  }

  // This function is useful for the purpose of DFA minimization, as it's guarenteed that all valid DFA's can only possess a single
  // unique connected by a character for every state, however this should NEVER be used with NFA's as for set sizes greater than 1
  // It will return keys pseudo randomly and thus lead to undefinable behaviour 
  pub fn get_first_connected_state_key_by_character(&self, connection_character: &str) -> Option<String> {

    if let Some(state_keys_connected_by_character) = self.get_connections_by_character(connection_character) {
      
      let first_state_key_connected_by_character = state_keys_connected_by_character.iter().next();
      return first_state_key_connected_by_character.cloned();

    }

    return None;

  }

  pub fn remove_all_connections_by_character(&mut self, connection_character: &str) {
    self.states_connected_to.remove(connection_character);
  }


}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.position == other.position
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

impl User {

  // Changes both the number stored in the object itself and the database
  // Get and update methods tied together as I do not expect for retrieval of this info
  // To ever occur without updating
  pub fn get_and_update_number_of_untitled_regular_automata_workspaces(&mut self) -> i32 {

    self.number_of_untitled_regular_automata_workspaces += 1;
    let mut conn = establish_connection();

    diesel::update(users::table
      .filter(users::id.eq(self.id)))
      .set(users::number_of_untitled_regular_automata_workspaces.eq(self.number_of_untitled_regular_automata_workspaces))
      .execute(&mut conn)
      .expect("There was an error updating the number of untitled regular automata workspaces in the database");

    self.number_of_untitled_regular_automata_workspaces

  }

  pub fn get_and_update_number_of_untitled_regex_workspaces(&mut self) -> i32 {

    self.number_of_untitled_regex_workspaces += 1;
    let mut conn = establish_connection();

    diesel::update(users::table
      .filter(users::id.eq(self.id)))
      .set(users::number_of_untitled_regex_workspaces.eq(self.number_of_untitled_regex_workspaces))
      .execute(&mut conn)
      .expect("There was an error updating the number of untitled regex workspaces in the database");

    self.number_of_untitled_regex_workspaces
  }

}


#[derive(Debug, Deserialize, Serialize)]
pub struct RegularAutomataWorkspaceData {
  start_state_index: Option<usize>,
  start_state_position: Option<String>,
  state_positions: HashMap<String, State>,
  list_of_states: Vec<State>,
  list_of_regular_automata_connections: Vec<Connection>,
  type_of_automata: TypeOfAutomata,
  date_of_last_update: String,
  alphabet: Vec<String>,
  should_strict_check: bool,
  should_show_string_traversal: bool,
  default_connection_character: String
}

impl RegularAutomataWorkspaceData {

  pub fn new(workspace: SavedRegularAutomataWorkspace) -> Self {

    let mut conn = establish_connection();

    let list_of_states = Self::get_list_of_states_from_saved_workspace(&workspace, &mut conn);
    let (start_state_index, start_state_position) = Self::get_start_state_information(&list_of_states);
    let list_of_regular_automata_connections = Self::get_list_of_regular_automata_connections_from_saved_workspace(&workspace, &mut conn);
    let state_positions = Self::get_state_positions_from_list_of_states(&list_of_states);
    let alphabet = Self::parse_alphabet(&workspace);

    let automata_type = workspace.type_of_automata;
    let date_of_last_update = workspace.date_of_last_update.format("%Y-%m-%d %H:%M:%S").to_string();
    let should_strict_check = workspace.should_strict_check;
    let should_show_string_traversal = workspace.should_show_string_traversal;
    let default_connection_character = workspace.default_connection_character;
    
    return RegularAutomataWorkspaceData {
      start_state_index,
      start_state_position,
      state_positions,
      list_of_states,
      list_of_regular_automata_connections,
      type_of_automata: automata_type,
      date_of_last_update,
      alphabet,
      should_strict_check,
      should_show_string_traversal,
      default_connection_character
    }

  }

  pub fn get_start_state_position(&self) -> &Option<String> {
    &self.start_state_position
  }

  pub fn get_state_positions(&self) -> &HashMap<String, State> {
    &self.state_positions
  }

  fn get_state_positions_from_list_of_states(list_of_states: &Vec<State>) -> HashMap<String, State> {

    let mut state_positions = HashMap::new();


    for state in list_of_states {

      let state_coords_as_string: String = state.position
        .into();

      state_positions.insert(state_coords_as_string.to_owned(), state.to_owned());

    }

    return state_positions;

  }

  fn get_list_of_states_from_saved_workspace(saved_workspace: &SavedRegularAutomataWorkspace, conn: &mut SqliteConnection) -> Vec<State> {
    // First get the states and connections from the database

    let mut list_of_states = Vec::new();

    let retrieved_states: Vec<SavedState> = saved_states::table
      .filter(saved_states::workspace_id.eq(&saved_workspace.id))
      .get_results::<SavedState>(conn)
      .expect("There was an issue getting the workspace's states");

    for retrieved_state in retrieved_states {
      let parsed_state = Self::parse_saved_state_to_regular_state(retrieved_state, saved_workspace, conn);
      list_of_states.push(parsed_state);
    }

    return list_of_states;

  }

  fn parse_saved_state_to_regular_state(state: SavedState, workspace: &SavedRegularAutomataWorkspace, conn: &mut SqliteConnection) -> State {
    
    let states_connected_to_given_state: Vec<SavedConnection> = saved_regular_automata_connections::table
      .filter(saved_regular_automata_connections::workspace_id.eq(&workspace.id))
      .filter(saved_regular_automata_connections::start_point.eq(&state.position))
      .get_results::<SavedConnection>(conn)
      .expect("There was an issue getting the workspace's states");

    let parsed_state_position = state.position.try_into()
      .expect("The string should be castable into Coordinate form");
  
    let mut parsed_state = State::new(parsed_state_position, state.is_start, state.is_final);
  
    for connected_state in states_connected_to_given_state {
      parsed_state.add_connection(&connected_state.connection_character, connected_state.end_point);
    }
    parsed_state
  }

  fn get_list_of_regular_automata_connections_from_saved_workspace(workspace: &SavedRegularAutomataWorkspace, conn: &mut SqliteConnection) -> Vec<Connection> {

    let mut list_of_regular_automata_connections = Vec::new();

    let retrieved_connections: Vec<SavedConnection> = saved_regular_automata_connections::table
      .filter(saved_regular_automata_connections::workspace_id.eq(&workspace.id))
      .get_results::<SavedConnection>(conn)
      .expect("There was an issue getting the workspace's states");

    for retrieved_connection in retrieved_connections {
      let parsed_connection = Self::parse_saved_connection_to_regular_connection(retrieved_connection);
      list_of_regular_automata_connections.push(parsed_connection);
    }

    list_of_regular_automata_connections

  }

  fn parse_saved_connection_to_regular_connection(connection: SavedConnection) -> Connection {

    let parsed_curve = BezierCurve {
      start_point: connection.start_point.try_into().expect("Could not parse string to coordinates"),
      control_point_one: connection.control_point_one.try_into().expect("Could not parse string to coordinates"),
      control_point_two: connection.control_point_two.try_into().expect("Could not parse string to coordinates"),
      end_point: connection.end_point.try_into().expect("Could not parse string to coordinates")
    };

    let parsed_connection = Connection {
      curve: parsed_curve,
      connection_character: connection.connection_character,
      element: String::from("Connection")
    };

    parsed_connection
  }
  
  // Return type corresponds to start state index and start state key respectively
  // Optional as saved workspace may not have start state
  fn get_start_state_information(list_of_states: &Vec<State>) -> (Option<usize>, Option<String>) {
    for (index, state_reference) in list_of_states.iter().enumerate() {
      if state_reference.is_start() {
        return (Some(index), Some(state_reference.get_position_as_string()));
      }
    }
    return (None, None);
  }

  fn parse_alphabet(saved_workspace: &SavedRegularAutomataWorkspace) -> Vec<String> {

    return saved_workspace.alphabet
      .split(',')
      .map(|s| s.to_string())
      .collect();
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegexWorkspaceData {
  regex_name: String,
  regex: String,
  date_of_last_update: String,
}

impl RegexWorkspaceData {
  pub fn new(workspace: SavedRegexWorkspace) -> Self {

    RegexWorkspaceData {
      regex_name: workspace.regex_name,
      regex: workspace.regex,
      date_of_last_update: workspace.date_of_last_update.format("%Y-%m-%d %H:%M:%S").to_string()
    }

  }
}
