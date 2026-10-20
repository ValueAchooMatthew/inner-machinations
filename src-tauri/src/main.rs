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
pub mod language_determination_funcs;
pub mod regular_expression_funcs;

use regular_expression_funcs::{build_parse_tree, test_string_regex};
use advanced_automata_funcs::{minimize_dfa, convert_nfa_to_dfa};
use testing_automata_funcs::{test_string_dfa, test_string_nfa};
use saving_automata_funcs::{save_workspace, delete_workspace, retrieve_workspace_data, get_saved_workspaces, 
update_workspace_name, does_workspace_name_exist, create_workspace, update_workspace_alphabet, 
update_showing_string_traversal, update_default_connection_character, update_strict_checking, update_automata_type};
use validation_automata_funcs::verify_valid_dfa;
use language_determination_funcs::determine_language_of_automata;
use app::establish_connection;

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) {

  // This will run the necessary migrations.
  connection
    .run_pending_migrations(MIGRATIONS)
    .expect("There was an error running the migrations");
}

fn main() {

  // WebKitGTK's DMA-BUF renderer makes the window quit with "Error 71 (Protocol error)
  // dispatching to Wayland display" on NVIDIA under Wayland, so fall back to its other
  // renderer unless the user set this themselves. Runs before any other thread starts.
  #[cfg(target_os = "linux")]
  {
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
      std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
  }

  set_working_directory();

  let mut connection = establish_connection();
  run_migrations(&mut connection);

  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    test_string_dfa, test_string_nfa, verify_valid_dfa, save_workspace, delete_workspace, retrieve_workspace_data, 
    get_saved_workspaces, minimize_dfa, convert_nfa_to_dfa, determine_language_of_automata, build_parse_tree, 
    test_string_regex, update_workspace_name, does_workspace_name_exist, create_workspace, update_workspace_alphabet, 
    update_default_connection_character, update_showing_string_traversal, update_strict_checking, update_automata_type]
  )
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
#[cfg(test)]
mod tests {
  use super::MIGRATIONS;
  use diesel::sql_types::{Integer, Text};
  use diesel::{sql_query, Connection, QueryableByName, RunQueryDsl, SqliteConnection};
  use diesel_migrations::MigrationHarness;

  #[derive(QueryableByName, Debug, PartialEq)]
  struct Workspace {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    workspace_name: String
  }

  #[derive(QueryableByName, Debug, PartialEq)]
  struct Count {
    #[diesel(sql_type = Integer)]
    count: i32
  }

  // Foreign keys are on, as they are in the app, which made the first version of the migration
  // removing accounts fail on any database that had saved states
  #[test]
  fn test_migrations_keep_workspaces_from_databases_with_accounts() {
    let mut connection = SqliteConnection::establish(":memory:").unwrap();
    sql_query("PRAGMA foreign_keys = ON").execute(&mut connection).unwrap();

    connection.run_next_migration(MIGRATIONS).unwrap();
    for statement in [
      "INSERT INTO users (id, email, password) VALUES (1, 'first', 'x'), (2, 'second', 'y')",
      "INSERT INTO saved_workspaces (id, user_id, workspace_name, type_of_automata)
        VALUES (1, 1, 'Untitled Project', 'DFA'), (2, 2, 'Untitled Project', 'NFA'), (3, 2, 'Other', 'DFA')",
      "INSERT INTO saved_states (workspace_id, position) VALUES (2, '100,100'), (3, '200,200')",
      "INSERT INTO saved_connections (workspace_id, start_point, control_point_one, control_point_two, end_point, connection_character)
        VALUES (3, '200,200', '200,200', '200,200', '200,200', 'a')"
    ] {
      sql_query(statement).execute(&mut connection).unwrap();
    }

    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let workspaces: Vec<Workspace> = sql_query("SELECT id, workspace_name FROM saved_workspaces ORDER BY id")
      .load(&mut connection)
      .unwrap();
    assert_eq!(workspaces, vec![
      Workspace { id: 1, workspace_name: "Untitled Project".to_owned() },
      Workspace { id: 2, workspace_name: "Untitled Project (2)".to_owned() },
      Workspace { id: 3, workspace_name: "Other".to_owned() }
    ]);

    let mut count = |query: &str| sql_query(query).load::<Count>(&mut connection).unwrap()[0].count;
    assert_eq!(count("SELECT count(*) AS count FROM saved_states"), 2);
    assert_eq!(count("SELECT count(*) AS count FROM saved_connections"), 1);
    assert_eq!(count("SELECT count(*) AS count FROM sqlite_master WHERE name = 'users'"), 0);
    assert_eq!(count("SELECT count(*) AS count FROM pragma_foreign_key_list('saved_states') WHERE \"table\" = 'saved_workspaces'"), 1);

    // States still can't point at a workspace that doesn't exist
    assert!(sql_query("INSERT INTO saved_states (workspace_id, position) VALUES (99, '0,0')").execute(&mut connection).is_err());
  }
}
