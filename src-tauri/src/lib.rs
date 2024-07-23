extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use std::cell::RefCell;
use std::{collections::HashMap, fs};
use std::io::Write;
use std::env;

use diesel::Connection as DieselConnection;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
  SqliteConnection::establish(&"mydb.sqlite3")
    .unwrap_or_else(|_| panic!("Error connecting to database"))
}

use models::{BezierCurve, Connection, Coordinate, SmartState, State, User};

use magic_crypt::{MagicCrypt256, MagicCryptTrait};
pub fn encrypt_user_data(cipher: &MagicCrypt256, email: &str, password: &str) -> [String; 2] {
  let encrypted_email = cipher.encrypt_str_to_base64(email);
  let encrypted_password = cipher.encrypt_str_to_base64(password);
  [encrypted_email, encrypted_password]
}

pub fn decrypt_user_data(cipher: &MagicCrypt256, user: User) -> [String; 2]{
  let decrypted_email = cipher.decrypt_base64_to_string(user.email).unwrap();
  let decrypted_password = cipher.decrypt_base64_to_string(user.password).unwrap();
  [decrypted_email, decrypted_password]
}

use rand::{distributions::Alphanumeric, Rng}; // 0.8
pub fn generate_code() -> String {
  let code: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(6)
    .map(char::from)
    .collect();
  code
}

pub fn get_encryption_key() -> String {

  let mut env_file = dotenv::dotenv();

  if env_file.is_err() {
    create_new_env_file();
    env_file = dotenv::dotenv();
  }
  
  env_file.ok();

  env::var("ENCRYPTION_KEY")
    .expect("There was an error retrieving the encryption key")

}

fn create_new_env_file() {
  let mut new_env_file = fs::File
  ::create_new(".env")
  .unwrap();

let random_string: String = rand::thread_rng()
  .sample_iter(&Alphanumeric)
  .take(12)
  .map(char::from)
  .collect();

let env_variable = String::from("ENCRYPTION_KEY=") + format!("{random_string}").as_str();
new_env_file.write(env_variable.as_bytes())
  .expect("There was an error writing to the env file");
}

pub fn set_working_directory() {

  let path_to_executable = env::current_exe()
    .expect("There was an error retrieving the path to the executable");

  let exectable_directory = path_to_executable.parent();

  env::set_current_dir(exectable_directory.unwrap())
    .expect("There was an error setting the working directory");

}

pub fn create_unique_state_coordinates(state_positions: &HashMap<String, RefCell<State>>) -> Coordinate {
  
  let mut x_position = 300;
  let mut y_position = 300;

  let mut hashed_position = x_position.to_string() + "," + y_position.to_string().as_str();

  while state_positions.contains_key(&hashed_position) {

    if x_position < 800 {
      x_position += 200;
    } else {
      x_position = 100;
      y_position += 200;
    }

    hashed_position = x_position.to_string() + "," + y_position.to_string().as_str();

  };
  return Coordinate {
    x: x_position,
    y: y_position
  };
}

pub fn create_connections_from_state_positions(state_positions: &HashMap<String, RefCell<State>>) -> Vec<Connection> {

  let mut connections = vec![];

  // quite slow but okay as most graphs used will be quite sparse
  for (current_state_key, current_state) in state_positions {

    let current_state = current_state.borrow();

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