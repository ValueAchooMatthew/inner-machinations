use std::{collections::{HashMap, HashSet}, fmt::Debug};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Deserialize, Serialize, Clone, Eq)]
pub struct State { 
  position: Coordinate,
  states_connected_to: HashMap<String, HashSet<String>>,
  is_start: bool,
  is_final: bool,
  element: String
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

impl State {

  pub fn new<T: TryInto<Coordinate> + Debug>(position: T, is_start: bool, is_final: bool) -> Self 
  where <T as TryInto<Coordinate>>::Error: Debug {

    let position = position.try_into().unwrap();

    return State {
      position: position,
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
  
  // Make make custom error type in future for this case
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
        .trim()
        .parse::<i32>()
        .unwrap(),

      y: processed_strings
        .get(1)
        .unwrap()
        .trim()
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
        .trim()
        .parse::<i32>()
        .unwrap(),

      y: processed_strings
        .get(1)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap()
    };

    return Ok(coordinate);
  }
}

impl TryFrom<&str> for Coordinate {

  type Error = ();

  fn try_from(value: &str) -> Result<Coordinate, ()> {

    let processed_strings: Vec<&str> = value.split(",").collect();

    if processed_strings.len() != 2 {
      return Err(());
    }

    let coordinate = Coordinate {
      x: processed_strings
        .get(0)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap(),
      y: processed_strings
        .get(1)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap()
    };
    return Ok(coordinate);
  }
}
