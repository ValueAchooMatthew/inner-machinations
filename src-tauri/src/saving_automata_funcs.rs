use tauri::State as Managed;

use crate::database::Database;
use crate::error::{validate_connection_character, validate_workspace_name, CommandResult, Error};
use crate::models::{AutomatonData, TypeOfAutomata, WorkspaceData};
use crate::workspace_storage;

#[tauri::command]
pub fn create_workspace(database: Managed<'_, Database>, workspace_name: &str) -> CommandResult<()> {
  validate_workspace_name(workspace_name)?;
  workspace_storage::create_workspace(&mut database.connection(), workspace_name)
    .map_err(|error| Error::from_naming(error, "create the workspace", workspace_name))
}

#[tauri::command]
pub fn update_workspace_alphabet(
  database: Managed<'_, Database>,
  workspace_name: &str,
  alphabet: Vec<&str>
) -> CommandResult<Vec<String>> {
  workspace_storage::update_workspace_alphabet(&mut database.connection(), workspace_name, alphabet)
    .map_err(|error| Error::from_storage(error, "save the alphabet", workspace_name))
}

// Saves the automaton and the alphabet, and returns the alphabet as saved
#[tauri::command]
pub fn save_workspace(
  database: Managed<'_, Database>,
  workspace_name: &str,
  automaton: AutomatonData,
  alphabet: Vec<&str>
) -> CommandResult<Vec<String>> {
  workspace_storage::save_workspace(&mut database.connection(), workspace_name, &automaton, alphabet)
    .map_err(|error| Error::from_storage(error, "save the workspace", workspace_name))
}

#[tauri::command]
pub fn delete_workspace(database: Managed<'_, Database>, workspace_name: &str) -> CommandResult<()> {
  workspace_storage::delete_workspace(&mut database.connection(), workspace_name)
    .map_err(|error| Error::from_storage(error, "delete the workspace", workspace_name))
}

#[tauri::command]
pub fn does_workspace_name_exist(database: Managed<'_, Database>, workspace_name: &str) -> CommandResult<bool> {
  workspace_storage::does_workspace_name_exist(&mut database.connection(), workspace_name)
    .map_err(|error| Error::from_storage(error, "look for the workspace", workspace_name))
}

#[tauri::command]
pub fn update_workspace_name(
  database: Managed<'_, Database>,
  original_workspace_name: &str,
  new_workspace_name: &str
) -> CommandResult<()> {
  validate_workspace_name(new_workspace_name)?;
  workspace_storage::update_workspace_name(&mut database.connection(), original_workspace_name, new_workspace_name)
    .map_err(|error| {
      // The original name is the one that can be missing, and the new name the one that can be taken
      let name = if error == diesel::result::Error::NotFound { original_workspace_name } else { new_workspace_name };
      Error::from_naming(error, "rename the workspace", name)
    })
}

#[tauri::command]
pub fn update_automata_type(
  database: Managed<'_, Database>,
  workspace_name: &str,
  type_of_automata: TypeOfAutomata
) -> CommandResult<()> {
  workspace_storage::update_automata_type(&mut database.connection(), workspace_name, type_of_automata)
    .map_err(|error| Error::from_storage(error, "save the type of automaton", workspace_name))
}

#[tauri::command]
pub fn update_strict_checking(
  database: Managed<'_, Database>,
  workspace_name: &str,
  should_strict_check: bool
) -> CommandResult<()> {
  workspace_storage::update_strict_checking(&mut database.connection(), workspace_name, should_strict_check)
    .map_err(|error| Error::from_storage(error, "save the strict checking option", workspace_name))
}

#[tauri::command]
pub fn update_showing_string_traversal(
  database: Managed<'_, Database>,
  workspace_name: &str,
  should_show_traversal: bool
) -> CommandResult<()> {
  workspace_storage::update_showing_string_traversal(&mut database.connection(), workspace_name, should_show_traversal)
    .map_err(|error| Error::from_storage(error, "save the string traversal option", workspace_name))
}

#[tauri::command]
pub fn update_default_connection_character(
  database: Managed<'_, Database>,
  workspace_name: &str,
  default_connection_character: String
) -> CommandResult<()> {
  validate_connection_character(&default_connection_character)?;
  workspace_storage::update_default_connection_character(&mut database.connection(), workspace_name, &default_connection_character)
    .map_err(|error| Error::from_storage(error, "save the default connection character", workspace_name))
}

#[tauri::command]
pub fn retrieve_workspace_data(database: Managed<'_, Database>, workspace_name: &str) -> CommandResult<WorkspaceData> {
  workspace_storage::retrieve_workspace_data(&mut database.connection(), workspace_name)
    .map_err(|error| Error::from_storage(error, "open the workspace", workspace_name))
}

#[tauri::command]
pub fn get_saved_workspaces(database: Managed<'_, Database>) -> CommandResult<Vec<String>> {
  workspace_storage::get_saved_workspaces(&mut database.connection())
    .map_err(|error| Error::new(format!("Couldn't list the saved workspaces: {error}")))
}
