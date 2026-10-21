use tauri::State as Managed;

use crate::database::Database;
use crate::error::{validate_regex_name, CommandResult, Error};
use crate::models::RegexData;
use crate::regex_storage;

#[tauri::command]
pub fn create_regex(database: Managed<'_, Database>, regex_name: &str) -> CommandResult<()> {
  validate_regex_name(regex_name)?;
  regex_storage::create_regex(&mut database.connection(), regex_name)
    .map_err(|error| Error::from_regex_naming(error, "create the regular expression", regex_name))
}

#[tauri::command]
pub fn get_saved_regexes(database: Managed<'_, Database>) -> CommandResult<Vec<RegexData>> {
  regex_storage::get_saved_regexes(&mut database.connection())
    .map_err(|error| Error::new(format!("Couldn't list the saved regular expressions: {error}")))
}

#[tauri::command]
pub fn retrieve_regex(database: Managed<'_, Database>, regex_name: &str) -> CommandResult<RegexData> {
  regex_storage::retrieve_regex(&mut database.connection(), regex_name)
    .map_err(|error| Error::from_regex_storage(error, "open the regular expression", regex_name))
}

// Saves the regular expression as typed, whether or not it can be built
#[tauri::command]
pub fn save_regex(database: Managed<'_, Database>, regex_name: &str, regex: &str) -> CommandResult<()> {
  regex_storage::save_regex(&mut database.connection(), regex_name, regex)
    .map_err(|error| Error::from_regex_storage(error, "save the regular expression", regex_name))
}

#[tauri::command]
pub fn update_regex_name(
  database: Managed<'_, Database>,
  original_regex_name: &str,
  new_regex_name: &str
) -> CommandResult<()> {
  validate_regex_name(new_regex_name)?;
  regex_storage::update_regex_name(&mut database.connection(), original_regex_name, new_regex_name)
    .map_err(|error| {
      // The original name is the one that can be missing, and the new name the one that can be taken
      let name = if error == diesel::result::Error::NotFound { original_regex_name } else { new_regex_name };
      Error::from_regex_naming(error, "rename the regular expression", name)
    })
}

#[tauri::command]
pub fn delete_regex(database: Managed<'_, Database>, regex_name: &str) -> CommandResult<()> {
  regex_storage::delete_regex(&mut database.connection(), regex_name)
    .map_err(|error| Error::from_regex_storage(error, "delete the regular expression", regex_name))
}

#[tauri::command]
pub fn does_regex_name_exist(database: Managed<'_, Database>, regex_name: &str) -> CommandResult<bool> {
  regex_storage::does_regex_name_exist(&mut database.connection(), regex_name)
    .map_err(|error| Error::from_regex_storage(error, "look for the regular expression", regex_name))
}
