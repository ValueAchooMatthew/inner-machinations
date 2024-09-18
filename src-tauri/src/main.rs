// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
extern crate diesel_migrations;

mod schema;
mod user_flow_funcs;
mod regular_expression_funcs;
mod regular_automata_funcs;
mod tests;
mod miscellaneous;

use miscellaneous::{database_models_and_utilities::establish_connection, environment::{run_migrations, set_working_directory}};
use regular_expression_funcs::{regular_expression_linguistics::test_string_regex, regular_expression_parsing::build_parse_tree, 
regular_expression_saving::{create_regex_workspace, retrieve_regex_workspace_data, does_regex_workspace_name_exist, 
delete_regex_workspace, save_regex_workspace, update_regex_workspace_name, get_users_regex_workspace_names}};
use regular_automata_funcs::{regular_automata_extra_features::{convert_nfa_to_dfa, minimize_dfa}, 
regular_automata_saving::{create_regular_automata_workspace, delete_regular_automata_workspace, does_regular_automata_workspace_name_exist, 
get_users_regular_automata_workspace_names, retrieve_regular_automata_workspace_data, save_regular_automata_workspace, update_default_connection_character, 
update_regular_automata_type, update_regular_automata_workspace_alphabet, update_regular_automata_workspace_name, update_showing_string_traversal, 
update_strict_checking}};
use regular_automata_funcs::{regular_automata_linguistics::{test_string_dfa, test_string_nfa, determine_language_of_regular_automata}, 
regular_automata_validation::verify_valid_dfa};
use user_flow_funcs::{registration::{is_correct_log_in, is_user_registered, register_user}, 
verification::{is_user_verified, send_verification_email, verify_user}};

fn main() {

  set_working_directory();

  let mut connection = establish_connection();
  run_migrations(&mut connection);

  tauri::Builder::default()
  .plugin(tauri_plugin_store::Builder::default().build())
  .invoke_handler(tauri::generate_handler![
    register_user, is_user_registered, is_correct_log_in, send_verification_email, verify_user, is_user_verified, test_string_dfa,
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