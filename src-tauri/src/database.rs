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
    assert!(sql_query("INSERT INTO saved_states (workspace_id, state_id, x, y) VALUES (99, 1, 0, 0)").execute(&mut connection).is_err());
  }

  #[derive(QueryableByName, Debug, PartialEq)]
  struct SavedState {
    #[diesel(sql_type = Integer)]
    workspace_id: i32,
    #[diesel(sql_type = Integer)]
    state_id: i32,
    #[diesel(sql_type = Integer)]
    x: i32,
    #[diesel(sql_type = Integer)]
    y: i32
  }

  #[derive(QueryableByName, Debug, PartialEq)]
  struct SavedConnection {
    #[diesel(sql_type = Integer)]
    connection_id: i32,
    #[diesel(sql_type = Integer)]
    from_state: i32,
    #[diesel(sql_type = Integer)]
    to_state: i32,
    #[diesel(sql_type = Integer)]
    control_point_one_x: i32,
    #[diesel(sql_type = Integer)]
    control_point_two_y: i32
  }

  // States found by position become states found by id, with what can't be read left out
  #[test]
  fn test_migrations_give_states_ids() {
    let mut connection = SqliteConnection::establish(":memory:").unwrap();
    sql_query("PRAGMA foreign_keys = ON").execute(&mut connection).unwrap();

    connection.run_next_migration(MIGRATIONS).unwrap();
    connection.run_next_migration(MIGRATIONS).unwrap();
    for statement in [
      "INSERT INTO saved_workspaces (id, workspace_name, type_of_automata) VALUES (1, 'first', 'DFA'), (2, 'second', 'NFA')",
      "INSERT INTO saved_states (id, workspace_id, position, is_start, is_final)
        VALUES (10, 1, '200,200', 1, 0), (11, 1, '500,200', 0, 1), (12, 1, 'nonsense', 0, 0), (20, 2, '-100,300', 1, 1)",
      "INSERT INTO saved_connections (id, workspace_id, start_point, control_point_one, control_point_two, end_point, connection_character)
        VALUES (30, 1, '200,200', '250,150', '450,150', '500,200', 'a'),
               (31, 1, '200,200', '200,200', '900,900', '900,900', 'a'),
               (32, 1, '500,200', '500,200', '200,200', '200,200', 'b'),
               (33, 2, '-100,300', '-300,500', '-300,100', '-100,300', 'c')"
    ] {
      sql_query(statement).execute(&mut connection).unwrap();
    }

    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let states: Vec<SavedState> = sql_query("SELECT workspace_id, state_id, x, y FROM saved_states ORDER BY id")
      .load(&mut connection)
      .unwrap();
    assert_eq!(states, vec![
      SavedState { workspace_id: 1, state_id: 1, x: 200, y: 200 },
      SavedState { workspace_id: 1, state_id: 2, x: 500, y: 200 },
      SavedState { workspace_id: 2, state_id: 1, x: -100, y: 300 }
    ]);

    let connections: Vec<SavedConnection> = sql_query(
      "SELECT connection_id, from_state, to_state, control_point_one_x, control_point_two_y FROM saved_connections ORDER BY id"
    )
      .load(&mut connection)
      .unwrap();
    assert_eq!(connections, vec![
      SavedConnection { connection_id: 1, from_state: 1, to_state: 2, control_point_one_x: 250, control_point_two_y: 150 },
      SavedConnection { connection_id: 2, from_state: 2, to_state: 1, control_point_one_x: 500, control_point_two_y: 200 },
      SavedConnection { connection_id: 1, from_state: 1, to_state: 1, control_point_one_x: -300, control_point_two_y: 100 }
    ]);

    // A connection can only join states that exist
    assert!(sql_query(
      "INSERT INTO saved_connections (workspace_id, connection_id, from_state, to_state, connection_character,
        control_point_one_x, control_point_one_y, control_point_two_x, control_point_two_y) VALUES (1, 9, 1, 7, 'a', 0, 0, 0, 0)"
    ).execute(&mut connection).is_err());
  }
}
