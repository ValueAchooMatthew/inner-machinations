// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
extern crate diesel_migrations;
use app::get_encryption_key;
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

use std::env;
use registration_funcs::{is_correct_log_in, register_user, is_user_registered};
use advanced_automata_funcs::{minimize_dfa, convert_nfa_to_dfa};
use testing_automata_funcs::{test_string_dfa, test_string_nfa};
use saving_automata_funcs::{save_workspace, delete_workspace, retrieve_workspace_data, get_users_saved_workspaces};
use validation_automata_funcs::verify_valid_dfa;
use language_determination_funcs::determine_language_of_dfa;
use verification_funcs::{send_verification_email, is_user_verified, verify_user};
use app::{establish_connection, generate_code};


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
  .invoke_handler(tauri::generate_handler![
    register_user, is_user_registered, is_correct_log_in,
    send_verification_email, verify_user, is_user_verified, test_string_dfa,
    test_string_nfa, verify_valid_dfa, save_workspace, delete_workspace, retrieve_workspace_data, 
    get_users_saved_workspaces, minimize_dfa, convert_nfa_to_dfa, determine_language_of_dfa
  ]
)
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}