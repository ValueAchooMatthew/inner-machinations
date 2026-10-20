extern crate diesel;

pub mod schema;
pub mod models;
pub mod automaton;

use std::collections::{BTreeMap, HashMap, HashSet};
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

// Creates a connection for every transition. A previous connection with the same endpoints and character
// keeps its curve, so curves the user has shaped survive
pub fn create_connections_from_state_positions(
  state_positions: &HashMap<String, State>,
  previous_connections: &[Connection]
) -> Vec<Connection> {

  let mut connections = vec![];

  // How many connections join each pair of states so far, in either direction, so the next one can be drawn apart from them
  let mut connection_counts: HashMap<(&String, &String), usize> = HashMap::new();

  // Sorted so the same states always produce the same connections
  let sorted_state_positions: BTreeMap<&String, &State> = state_positions.iter().collect();

  for (current_state_key, current_state) in sorted_state_positions {

    let sorted_connections: BTreeMap<&String, &HashSet<String>> = current_state
      .get_all_connections()
      .iter()
      .collect();

    for (connection_character, connected_state_keys) in sorted_connections {

      let mut connected_state_keys: Vec<&String> = connected_state_keys.iter().collect();
      connected_state_keys.sort();

      for connected_state_key in connected_state_keys {

        let Some(connected_state) = state_positions.get(connected_state_key) else {
          continue;
        };

        let start_point = current_state.get_position();
        let end_point = connected_state.get_position();

        let previous_connection = previous_connections.iter().find(|connection| {
          connection.connection_character == *connection_character
            && connection.curve.start_point == start_point
            && connection.curve.end_point == end_point
        });

        let state_pair = if current_state_key <= connected_state_key {
          (current_state_key, connected_state_key)
        } else {
          (connected_state_key, current_state_key)
        };
        let connection_count = connection_counts.entry(state_pair).or_insert(0);

        let curve = match previous_connection {
          Some(previous_connection) => previous_connection.curve.clone(),
          None => create_default_curve(start_point, end_point, current_state_key == connected_state_key, *connection_count)
        };
        *connection_count += 1;

        connections.push(Connection {
          connection_character: connection_character.to_owned(),
          curve,
          element: String::from("Connection")
        });
      }

    }

  };

  return connections;

}

// The first connection between two states is straight, and each one after it bends further away, alternating sides,
// so their characters are drawn apart. A state's first loop to itself goes to the left, as one the user draws does,
// and later loops go up, down and right, getting bigger after that
fn create_default_curve(start_point: Coordinate, end_point: Coordinate, is_connected_to_self: bool, index: usize) -> BezierCurve {

  if is_connected_to_self {
    let size = 200 + 100 * (index / 4) as i32;
    let ((first_x, first_y), (second_x, second_y)) = match index % 4 {
      0 => ((-size, size), (-size, -size)),
      1 => ((-size, -size), (size, -size)),
      2 => ((size, size), (-size, size)),
      _ => ((size, -size), (size, size))
    };
    return BezierCurve {
      start_point,
      control_point_one: Coordinate { x: start_point.x + first_x, y: start_point.y + first_y },
      control_point_two: Coordinate { x: end_point.x + second_x, y: end_point.y + second_y },
      end_point
    };
  }

  // The bend is measured from the same side whichever way the connection goes
  let (first_point, second_point) = if (start_point.x, start_point.y) <= (end_point.x, end_point.y) {
    (start_point, end_point)
  } else {
    (end_point, start_point)
  };
  let x_distance = (second_point.x - first_point.x) as f64;
  let y_distance = (second_point.y - first_point.y) as f64;
  let length = x_distance.hypot(y_distance);

  if index == 0 || length == 0.0 {
    return BezierCurve {
      start_point,
      control_point_one: start_point,
      control_point_two: end_point,
      end_point
    };
  }

  // 100 to one side, then 100 to the other, then 200 to the first side, and so on
  let bend = 100.0 * ((index + 1) / 2) as f64 * if index % 2 == 1 { 1.0 } else { -1.0 };
  let point_along_curve = |fraction: f64| Coordinate {
    x: (start_point.x as f64 + (end_point.x - start_point.x) as f64 * fraction - y_distance / length * bend).round() as i32,
    y: (start_point.y as f64 + (end_point.y - start_point.y) as f64 * fraction + x_distance / length * bend).round() as i32
  };

  BezierCurve {
    start_point,
    control_point_one: point_along_curve(1.0 / 3.0),
    control_point_two: point_along_curve(2.0 / 3.0),
    end_point
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
