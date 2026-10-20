extern crate diesel;

pub mod schema;
pub mod models;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::{env, fs};

use diesel::Connection as DieselConnection;
use diesel::sqlite::SqliteConnection;

use models::{BezierCurve, Connection, Coordinate, State};

// Set once at startup, to mydb.sqlite3 in the app data folder
static DATABASE_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn set_database_path(database_path: PathBuf) {
  DATABASE_PATH.set(database_path).ok();
}

pub fn establish_connection() -> SqliteConnection {
  let database_path = DATABASE_PATH
    .get()
    .expect("The database path must be set before connecting to the database");

  SqliteConnection::establish(&database_path.to_string_lossy())
    .unwrap_or_else(|_| panic!("Error connecting to database"))
}

// Earlier builds kept the database beside the executable. If the data folder has no database yet,
// copy that one across, leaving the original where it was
pub fn import_database_from_executable_directory(database_path: &Path) {

  if database_path.exists() {
    return;
  }

  let previous_database_path = env::current_exe()
    .ok()
    .and_then(|path_to_executable| Some(path_to_executable.parent()?.join("mydb.sqlite3")));

  if let Some(previous_database_path) = previous_database_path {
    if previous_database_path.is_file() {
      fs::copy(&previous_database_path, database_path).ok();
    }
  }

}

pub fn create_unique_state_coordinates(state_positions: &HashSet<String>) -> Coordinate {
  
  let mut x_position = 300;
  let mut y_position = 300;

  let mut hashed_position = x_position.to_string() + "," + y_position.to_string().as_str();

  while state_positions.contains(&hashed_position) {

    if x_position < 800 {
      x_position += 200;
    } else {
      x_position = 300;
      y_position += 200;
    }

    hashed_position = x_position.to_string() + "," + y_position.to_string().as_str();

  };
  return Coordinate {
    x: x_position,
    y: y_position
  };
}

pub fn create_connections_from_state_positions(state_positions: &HashMap<String, State>) -> Vec<Connection> {

  let mut connections = vec![];

  // quite slow but okay as most graphs used will be quite sparse
  for (current_state_key, current_state) in state_positions {

    for (connection_character, connected_state_keys) in current_state.get_all_connections() {
      for connected_state_key in connected_state_keys {
        let is_connected_to_self: bool = current_state_key == connected_state_key;

        let end_point: Coordinate = connected_state_key
          .try_into()
          .expect("Could not parse given key to coordinates");

        let current_position = current_state.get_position();

        let new_bezier_curve = BezierCurve {
          start_point: 
          current_position,

          control_point_one: 
            if is_connected_to_self { 
              Coordinate {
              x: current_position.x - 200,
              y: current_position.y + 200
              } 
            } else {
              current_position
            },

          control_point_two:
            if is_connected_to_self 
            { Coordinate {
              x: end_point.x - 200,
              y: end_point.y - 200
            } } else 
            {end_point},
          
          end_point
        };

        let new_connection = Connection {
          connection_character: connection_character
            .to_owned(),
          curve: new_bezier_curve,
          element: String::from("Connection")
        };

        connections.push(new_connection)
      }

    }

  };

  return connections;

}

pub fn remove_all_epsilon_transitions(state_positions: &mut HashMap<String, State>) {

  let mut make_final;
  let mut finished = false;

  while !finished {
    finished = true;
    let cloned_state_positions = state_positions.clone();
    for (_, state) in &mut *state_positions {

      make_final = false;

      let connections = state
        .get_all_connections_mut();

      if let Some(epsilon_state_keys) = connections.clone().get("ϵ") {
        if epsilon_state_keys.len() == 0 {
          connections.remove("ϵ");
          continue;
        }

        finished = false;
        for epsilon_state_key in epsilon_state_keys {
          let epsilon_state = cloned_state_positions
            .get(epsilon_state_key)
            .expect("Could not retrieve the requested state");

          if epsilon_state.is_final() {
            make_final = true;
          }

          let connections_from_epsilon_state = epsilon_state.get_all_connections();

          for (character, keys) in connections_from_epsilon_state {
            connections
              .entry(character.to_owned())
              .and_modify(|current_set| {
                for key in keys {
                  current_set.insert(key.to_owned());
                }
              })
              .or_insert(keys.to_owned());
          }
          connections
            .entry("ϵ".to_owned())
            .and_modify(|current_set| {
              current_set.remove(epsilon_state_key);
          });

          if make_final {
            state.make_final();
          }
          break;
        }
      }
    }
  }
}

// An input alphabet must consist of entirely unique characters and should be at most a single character long
// We want to preserve the order of the alphabet for ease of use thus hashsets are not an option, thus we will iterate
// Over everything and ensure it fits our requirements
pub fn sanitize_input_alphabet(alphabet: Vec<&str>) -> Vec<String> {

  let mut previously_seen_input_characters = HashSet::new();
  let sanitized_alphabet = alphabet.into_iter().filter(|input_character| {
    if !previously_seen_input_characters.contains(input_character) && input_character.len() == 1 {
      previously_seen_input_characters.insert(input_character.to_owned());
      return true;
    }
    return false;
  })
  .map(|input_characters| {
    return input_characters.to_owned();
  })
  .collect();

  return sanitized_alphabet;
}