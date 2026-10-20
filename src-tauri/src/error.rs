use std::fmt;

use diesel::result::{DatabaseErrorKind, Error as DatabaseError};
use serde::{Serialize, Serializer};

// Why a command failed, sent to the frontend as a message it can show the user
#[derive(Debug, PartialEq)]
pub struct Error(String);

pub type CommandResult<T> = Result<T, Error>;

impl Error {

  pub fn new(message: impl Into<String>) -> Self {
    Error(message.into())
  }

  // For a database operation on the named workspace, which failed while the app was trying to do `action`
  pub fn from_storage(error: DatabaseError, action: &str, workspace_name: &str) -> Self {
    match error {
      DatabaseError::NotFound => Error(format!("There is no workspace called \"{workspace_name}\"")),
      DatabaseError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
        Error(format!("There is already a workspace called \"{workspace_name}\""))
      },
      other => Error(format!("Couldn't {action}: {other}"))
    }
  }

}

impl fmt::Display for Error {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.0)
  }
}

impl Serialize for Error {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&self.0)
  }
}

// Workspace names can't be blank
pub fn validate_workspace_name(workspace_name: &str) -> CommandResult<()> {
  if workspace_name.trim().is_empty() {
    return Err(Error::new("A workspace needs a name"));
  }
  Ok(())
}

// A connection character is exactly one character, and epsilon has its own kind of connection
pub fn validate_connection_character(connection_character: &str) -> CommandResult<()> {
  if connection_character.chars().count() != 1 || connection_character == crate::automaton::EPSILON {
    return Err(Error::new("The default connection character must be a single character, other than ϵ"));
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::{validate_connection_character, validate_workspace_name, Error};
  use diesel::result::{DatabaseErrorKind, Error as DatabaseError};

  #[test]
  fn test_storage_errors_are_explained() {
    assert_eq!(
      Error::from_storage(DatabaseError::NotFound, "save the workspace", "First"),
      Error::new("There is no workspace called \"First\"")
    );
    let unique = DatabaseError::DatabaseError(DatabaseErrorKind::UniqueViolation, Box::new("UNIQUE constraint failed".to_owned()));
    assert_eq!(
      Error::from_storage(unique, "rename the workspace", "Second"),
      Error::new("There is already a workspace called \"Second\"")
    );
    assert_eq!(
      Error::from_storage(DatabaseError::BrokenTransactionManager, "save the workspace", "First").to_string(),
      format!("Couldn't save the workspace: {}", DatabaseError::BrokenTransactionManager)
    );
  }

  #[test]
  fn test_errors_are_sent_as_their_message() {
    assert_eq!(serde_json::to_value(Error::new("Something went wrong")).unwrap(), "Something went wrong");
  }

  #[test]
  fn test_workspace_names_cant_be_blank() {
    assert!(validate_workspace_name("Automaton").is_ok());
    assert!(validate_workspace_name(" a ").is_ok());
    assert!(validate_workspace_name("").is_err());
    assert!(validate_workspace_name("   ").is_err());
  }

  #[test]
  fn test_connection_characters_are_single_characters() {
    assert!(validate_connection_character("a").is_ok());
    assert!(validate_connection_character("é").is_ok());
    assert!(validate_connection_character("").is_err());
    assert!(validate_connection_character("ab").is_err());
    assert!(validate_connection_character("ϵ").is_err());
  }
}
