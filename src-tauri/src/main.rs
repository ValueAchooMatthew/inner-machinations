// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
extern crate diesel_migrations;
use app::set_working_directory;
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub mod schema;
pub mod models;
pub mod testing_automata_funcs;
pub mod validation_automata_funcs;
pub mod saving_automata_funcs;
pub mod advanced_automata_funcs;
pub mod registration_funcs;
pub mod language_determination_funcs;
pub mod verification_funcs;
pub mod regular_expression_funcs;

use regular_expression_funcs::{build_parse_tree, test_string_regex, saving_regex_funcs::{
  create_regex_workspace, save_regex_workspace, does_regex_workspace_name_exist, get_users_regex_workspace_names, retrieve_regex_workspace_data,
  update_regex_workspace_name, delete_regex_workspace
}};
use registration_funcs::{is_correct_log_in, register_user, is_user_registered};
use advanced_automata_funcs::{minimize_dfa, convert_nfa_to_dfa};
use testing_automata_funcs::{test_string_dfa, test_string_nfa};
use saving_automata_funcs::{save_regular_automata_workspace, delete_regular_automata_workspace, retrieve_regular_automata_workspace_data, 
update_regular_automata_workspace_name,does_regular_automata_workspace_name_exist, create_regular_automata_workspace, 
update_regular_automata_workspace_alphabet, update_showing_string_traversal, update_default_connection_character, update_strict_checking, 
update_regular_automata_type, get_users_regular_automata_workspace_names};
use validation_automata_funcs::verify_valid_dfa;
use language_determination_funcs::determine_language_of_regular_automata;
use verification_funcs::{send_verification_email, is_user_verified, verify_user};
use app::establish_connection;

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) {

  // This will run the necessary migrations.
  connection
    .run_pending_migrations(MIGRATIONS)
    .expect("There was an error running the migrations");
}

fn main() {

  set_working_directory();

  let mut connection = establish_connection();
  run_migrations(&mut connection);

  tauri::Builder::default()
  .plugin(tauri_plugin_store::Builder::default().build())
  .invoke_handler(tauri::generate_handler![
    register_user, is_user_registered, is_correct_log_in,send_verification_email, verify_user, is_user_verified, test_string_dfa,
    test_string_nfa, verify_valid_dfa, save_regular_automata_workspace, delete_regular_automata_workspace, retrieve_regular_automata_workspace_data, 
    create_regular_automata_workspace, minimize_dfa, convert_nfa_to_dfa, determine_language_of_regular_automata, build_parse_tree, test_string_regex, 
    update_regular_automata_workspace_name, does_regular_automata_workspace_name_exist, update_regular_automata_workspace_alphabet, 
    update_default_connection_character, update_showing_string_traversal, update_strict_checking, update_regular_automata_type, create_regex_workspace, 
    get_users_regular_automata_workspace_names, save_regex_workspace, does_regex_workspace_name_exist, get_users_regex_workspace_names, 
    retrieve_regex_workspace_data, update_regex_workspace_name, delete_regex_workspace]
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}