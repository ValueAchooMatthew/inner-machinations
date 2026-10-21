extern crate diesel;
extern crate diesel_migrations;

pub mod schema;
pub mod models;
pub mod database;
pub mod error;
pub mod workspace_storage;
pub mod automaton;
pub mod testing_automata_funcs;
pub mod validation_automata_funcs;
pub mod saving_automata_funcs;
pub mod advanced_automata_funcs;
pub mod language_determination_funcs;
pub mod regular_expression_funcs;

#[cfg(test)]
mod test_support;

use std::collections::HashSet;
use std::fs;

use tauri::Manager;

use database::{import_database_from_executable_directory, Database};
use models::Coordinate;

use regular_expression_funcs::{build_parse_tree, test_string_regex};
use advanced_automata_funcs::{minimize_dfa, convert_nfa_to_dfa};
use testing_automata_funcs::test_string;
use saving_automata_funcs::{save_workspace, delete_workspace, retrieve_workspace_data, get_saved_workspaces, 
update_workspace_name, does_workspace_name_exist, create_workspace, update_workspace_alphabet, 
update_showing_string_traversal, update_default_connection_character, update_strict_checking, update_automata_type};
use validation_automata_funcs::verify_valid_dfa;
use language_determination_funcs::determine_language_of_automata;

pub fn run() {

  // WebKitGTK's DMA-BUF renderer makes the window quit with "Error 71 (Protocol error)
  // dispatching to Wayland display" on NVIDIA under Wayland, so fall back to its other
  // renderer unless the user set this themselves. Runs before any other thread starts.
  #[cfg(target_os = "linux")]
  {
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
      std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
  }

  tauri::Builder::default()
  .setup(|app| {
    // The database lives in the app data folder, which is writable wherever the app is installed
    let data_directory = app.path().app_data_dir()?;
    fs::create_dir_all(&data_directory)?;

    let database_path = data_directory.join("mydb.sqlite3");
    import_database_from_executable_directory(&database_path);
    let database = Database::open(&database_path).map_err(|error| error as Box<dyn std::error::Error>)?;
    app.manage(database);
    Ok(())
  })
  .invoke_handler(tauri::generate_handler![
    test_string, verify_valid_dfa, save_workspace, delete_workspace, retrieve_workspace_data, 
    get_saved_workspaces, minimize_dfa, convert_nfa_to_dfa, determine_language_of_automata, build_parse_tree, 
    test_string_regex, update_workspace_name, does_workspace_name_exist, create_workspace, update_workspace_alphabet, 
    update_default_connection_character, update_showing_string_traversal, update_strict_checking, update_automata_type]
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}


// The first free spot on a grid that starts at (300, 300), runs four spots across, and continues down
pub fn create_unique_state_coordinates(taken_positions: &HashSet<Coordinate>) -> Coordinate {

  let mut position = Coordinate { x: 300, y: 300 };

  while taken_positions.contains(&position) {
    if position.x < 800 {
      position.x += 200;
    } else {
      position = Coordinate { x: 300, y: position.y + 200 };
    }
  }

  position
}

// An input alphabet is a list of distinct single characters, kept in the order given. Epsilon is not an input
// character, and a comma can't be one either, since the database separates the alphabet's characters with commas
pub fn sanitize_input_alphabet(alphabet: Vec<&str>) -> Vec<String> {

  let mut previously_seen_input_characters = HashSet::new();

  alphabet
    .into_iter()
    .filter(|input_character| {
      input_character.chars().count() == 1
        && *input_character != automaton::EPSILON
        && *input_character != ","
        && previously_seen_input_characters.insert(*input_character)
    })
    .map(str::to_owned)
    .collect()
}
