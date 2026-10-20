use std::collections::HashMap;

use tauri::State as Managed;

use crate::database::Database;
use crate::models::{Connection, State, TypeOfAutomata, WorkspaceData};
use crate::workspace_storage;

#[tauri::command]
pub fn create_workspace(database: Managed<'_, Database>, workspace_name: &str) {
  workspace_storage::create_workspace(&mut database.connection(), workspace_name)
    .expect("There was an error creating the workspace");
}

#[tauri::command]
pub fn update_workspace_alphabet(database: Managed<'_, Database>, workspace_name: &str, alphabet: Vec<&str>) -> Vec<String> {
  workspace_storage::update_workspace_alphabet(&mut database.connection(), workspace_name, alphabet)
    .expect("There was an error updating the workspace's alphabet")
}

#[tauri::command]
pub fn save_workspace(
  database: Managed<'_, Database>,
  workspace_name: &str,
  states: HashMap<String, State>,
  connections: Vec<Connection>
) {
  workspace_storage::save_workspace(&mut database.connection(), workspace_name, &states, &connections)
    .expect("There was an error saving the workspace");
}

#[tauri::command]
pub fn delete_workspace(database: Managed<'_, Database>, workspace_name: &str) {
  workspace_storage::delete_workspace(&mut database.connection(), workspace_name)
    .expect("There was an error deleting the workspace");
}

#[tauri::command]
pub fn does_workspace_name_exist(database: Managed<'_, Database>, workspace_name: &str) -> bool {
  workspace_storage::does_workspace_name_exist(&mut database.connection(), workspace_name)
    .expect("There was an error looking for the workspace")
}

#[tauri::command]
pub fn update_workspace_name(database: Managed<'_, Database>, original_workspace_name: &str, new_workspace_name: &str) {
  workspace_storage::update_workspace_name(&mut database.connection(), original_workspace_name, new_workspace_name)
    .expect("Could not update the workspace name");
}

#[tauri::command]
pub fn update_automata_type(database: Managed<'_, Database>, workspace_name: &str, type_of_automata: TypeOfAutomata) {
  workspace_storage::update_automata_type(&mut database.connection(), workspace_name, type_of_automata)
    .expect("Could not update the automata type of the workspace");
}

#[tauri::command]
pub fn update_strict_checking(database: Managed<'_, Database>, workspace_name: &str, should_strict_check: bool) {
  workspace_storage::update_strict_checking(&mut database.connection(), workspace_name, should_strict_check)
    .expect("Could not update the strict checking option for the workspace");
}

#[tauri::command]
pub fn update_showing_string_traversal(database: Managed<'_, Database>, workspace_name: &str, should_show_traversal: bool) {
  workspace_storage::update_showing_string_traversal(&mut database.connection(), workspace_name, should_show_traversal)
    .expect("Could not update string traversal option for the workspace");
}

#[tauri::command]
pub fn update_default_connection_character(
  database: Managed<'_, Database>,
  workspace_name: &str,
  default_connection_character: String
) {
  workspace_storage::update_default_connection_character(&mut database.connection(), workspace_name, &default_connection_character)
    .expect("Could not update the default connection character for the workspace");
}

#[tauri::command]
pub fn retrieve_workspace_data(database: Managed<'_, Database>, workspace_name: &str) -> WorkspaceData {
  workspace_storage::retrieve_workspace_data(&mut database.connection(), workspace_name)
    .expect("There was an error retrieving the workspace")
}

#[tauri::command]
pub fn get_saved_workspaces(database: Managed<'_, Database>) -> Vec<String> {
  workspace_storage::get_saved_workspaces(&mut database.connection())
    .expect("There was an error retrieving the saved workspaces")
}
