use std::error::Error;
use std::path::Path;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::{env, fs};

use diesel::connection::SimpleConnection;
use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

// The app's connection to its database, which commands take turns with
pub struct Database(Mutex<SqliteConnection>);

impl Database {

  // Opens the database at the path, creating it if needed, and brings its tables up to date
  pub fn open(database_path: &Path) -> Result<Self, Box<dyn Error + Send + Sync>> {
    let mut connection = SqliteConnection::establish(&database_path.to_string_lossy())?;
    prepare(&mut connection)?;
    Ok(Database(Mutex::new(connection)))
  }

  pub fn connection(&self) -> MutexGuard<'_, SqliteConnection> {
    // A command that panicked while holding the connection doesn't leave it unusable for the rest
    self.0.lock().unwrap_or_else(PoisonError::into_inner)
  }

}

fn prepare(connection: &mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync>> {
  // The bundled SQLite enforces foreign keys already, but don't rely on how it was built
  connection.batch_execute("PRAGMA foreign_keys = ON")?;
  connection.run_pending_migrations(MIGRATIONS)?;
  Ok(())
}

// An empty database in memory with every migration run, for tests
#[cfg(test)]
pub fn in_memory_connection() -> SqliteConnection {
  let mut connection = SqliteConnection::establish(":memory:").unwrap();
  prepare(&mut connection).unwrap();
  connection
}

// Earlier builds kept the database beside the executable. If the data folder has no database yet,
// copy that one across, leaving the original where it was
pub fn import_database_from_executable_directory(database_path: &Path) {

  if database_path.exists() {
    return;
  }

  let previous_database_path = env::current_exe()
    .ok()
    .and_then(|path_to_executable| Some(path_to_executable.parent()?.join("mydb.sqlite3")));

  if let Some(previous_database_path) = previous_database_path {
    if previous_database_path.is_file() {
      fs::copy(&previous_database_path, database_path).ok();
    }
  }

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
