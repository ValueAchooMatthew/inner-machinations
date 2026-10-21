// Workspaces in the database. Each function is given the connection to use, so the commands can share
// the app's connection and the tests can each use their own database
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error::NotFound;

use crate::models::{
  AutomatonData, ConnectionData, Coordinate, SavedConnection, SavedState, SavedWorkspace, StateData, TypeOfAutomata,
  WorkspaceData
};
use crate::sanitize_input_alphabet;
use crate::schema::{saved_connections, saved_states, saved_workspaces};

#[cfg(test)]
mod tests;

pub fn create_workspace(connection: &mut SqliteConnection, workspace_name: &str) -> QueryResult<()> {

  diesel::insert_into(saved_workspaces::table)
    .values((
      saved_workspaces::workspace_name.eq(workspace_name),
      saved_workspaces::type_of_automata.eq(TypeOfAutomata::DFA)
    ))
    .execute(connection)?;

  Ok(())

}

pub fn does_workspace_name_exist(connection: &mut SqliteConnection, workspace_name: &str) -> QueryResult<bool> {
  Ok(get_workspace(connection, workspace_name).optional()?.is_some())
}

// The names of every saved workspace, oldest first
pub fn get_saved_workspaces(connection: &mut SqliteConnection) -> QueryResult<Vec<String>> {
  saved_workspaces::table
    .select(saved_workspaces::workspace_name)
    .order(saved_workspaces::id)
    .load(connection)
}

pub fn retrieve_workspace_data(connection: &mut SqliteConnection, workspace_name: &str) -> QueryResult<WorkspaceData> {

  let workspace = get_workspace(connection, workspace_name)?;

  let states: Vec<SavedState> = saved_states::table
    .filter(saved_states::workspace_id.eq(workspace.id))
    .order(saved_states::state_id)
    .load(connection)?;

  let connections: Vec<SavedConnection> = saved_connections::table
    .filter(saved_connections::workspace_id.eq(workspace.id))
    .order(saved_connections::connection_id)
    .load(connection)?;

  Ok(WorkspaceData {
    automaton: AutomatonData {
      states: states
        .into_iter()
        .map(|state| StateData {
          id: state.state_id,
          position: Coordinate { x: state.x, y: state.y },
          is_start: state.is_start,
          is_final: state.is_final
        })
        .collect(),
      connections: connections
        .into_iter()
        .map(|saved_connection| ConnectionData {
          id: saved_connection.connection_id,
          from: saved_connection.from_state,
          to: saved_connection.to_state,
          character: saved_connection.connection_character,
          control_points: [
            Coordinate { x: saved_connection.control_point_one_x, y: saved_connection.control_point_one_y },
            Coordinate { x: saved_connection.control_point_two_x, y: saved_connection.control_point_two_y }
          ]
        })
        .collect()
    },
    type_of_automata: workspace.type_of_automata,
    date_of_last_update: workspace.date_of_last_update.format("%Y-%m-%d %H:%M:%S").to_string(),
    alphabet: workspace.alphabet.split(',').filter(|character| !character.is_empty()).map(str::to_owned).collect(),
    should_strict_check: workspace.should_strict_check,
    should_show_string_traversal: workspace.should_show_string_traversal,
    default_connection_character: workspace.default_connection_character
  })

}

// Replaces the workspace's automaton and alphabet, all at once or not at all, and returns the alphabet as
// saved: its distinct single characters. A connection to a state the automaton doesn't have, or two states
// or connections with the same id, is an error and saves nothing
pub fn save_workspace(
  connection: &mut SqliteConnection,
  workspace_name: &str,
  automaton: &AutomatonData,
  alphabet: Vec<&str>
) -> QueryResult<Vec<String>> {

  connection.transaction(|connection| {
    let workspace = get_workspace(connection, workspace_name)?;
    let sanitized_input_alphabet = sanitize_input_alphabet(alphabet);

    // Connections first, since they refer to the states
    diesel::delete(saved_connections::table.filter(saved_connections::workspace_id.eq(workspace.id)))
      .execute(connection)?;
    diesel::delete(saved_states::table.filter(saved_states::workspace_id.eq(workspace.id)))
      .execute(connection)?;

    save_states(connection, workspace.id, &automaton.states)?;
    save_connections(connection, workspace.id, &automaton.connections)?;

    diesel::update(saved_workspaces::table.filter(saved_workspaces::id.eq(workspace.id)))
      .set((
        saved_workspaces::alphabet.eq(sanitized_input_alphabet.join(",")),
        saved_workspaces::date_of_last_update.eq::<NaiveDateTime>(chrono::offset::Local::now().naive_local())
      ))
      .execute(connection)?;

    Ok(sanitized_input_alphabet)
  })

}

// Deletes the workspace along with its states and connections, all at once or not at all
pub fn delete_workspace(connection: &mut SqliteConnection, workspace_name: &str) -> QueryResult<()> {

  connection.transaction(|connection| {
    let workspace = get_workspace(connection, workspace_name)?;

    diesel::delete(saved_connections::table.filter(saved_connections::workspace_id.eq(workspace.id)))
      .execute(connection)?;

    diesel::delete(saved_states::table.filter(saved_states::workspace_id.eq(workspace.id)))
      .execute(connection)?;

    diesel::delete(saved_workspaces::table.filter(saved_workspaces::id.eq(workspace.id)))
      .execute(connection)?;

    Ok(())
  })

}

pub fn update_workspace_name(
  connection: &mut SqliteConnection,
  original_workspace_name: &str,
  new_workspace_name: &str
) -> QueryResult<()> {

  let updated_rows = diesel::update(workspace_named(original_workspace_name))
    .set(saved_workspaces::workspace_name.eq(new_workspace_name))
    .execute(connection)?;

  expect_one_row(updated_rows)

}

// Saves the alphabet's distinct single characters, and returns them
pub fn update_workspace_alphabet(
  connection: &mut SqliteConnection,
  workspace_name: &str,
  alphabet: Vec<&str>
) -> QueryResult<Vec<String>> {

  let sanitized_input_alphabet = sanitize_input_alphabet(alphabet);

  let updated_rows = diesel::update(workspace_named(workspace_name))
    .set(saved_workspaces::alphabet.eq(sanitized_input_alphabet.join(",")))
    .execute(connection)?;

  expect_one_row(updated_rows)?;
  Ok(sanitized_input_alphabet)

}

pub fn update_automata_type(
  connection: &mut SqliteConnection,
  workspace_name: &str,
  type_of_automata: TypeOfAutomata
) -> QueryResult<()> {

  let updated_rows = diesel::update(workspace_named(workspace_name))
    .set(saved_workspaces::type_of_automata.eq(type_of_automata))
    .execute(connection)?;

  expect_one_row(updated_rows)

}

pub fn update_strict_checking(
  connection: &mut SqliteConnection,
  workspace_name: &str,
  should_strict_check: bool
) -> QueryResult<()> {

  let updated_rows = diesel::update(workspace_named(workspace_name))
    .set(saved_workspaces::should_strict_check.eq(should_strict_check))
    .execute(connection)?;

  expect_one_row(updated_rows)

}

pub fn update_showing_string_traversal(
  connection: &mut SqliteConnection,
  workspace_name: &str,
  should_show_traversal: bool
) -> QueryResult<()> {

  let updated_rows = diesel::update(workspace_named(workspace_name))
    .set(saved_workspaces::should_show_string_traversal.eq(should_show_traversal))
    .execute(connection)?;

  expect_one_row(updated_rows)

}

pub fn update_default_connection_character(
  connection: &mut SqliteConnection,
  workspace_name: &str,
  default_connection_character: &str
) -> QueryResult<()> {

  let updated_rows = diesel::update(workspace_named(workspace_name))
    .set(saved_workspaces::default_connection_character.eq(default_connection_character))
    .execute(connection)?;

  expect_one_row(updated_rows)

}

fn workspace_named(workspace_name: &str) -> diesel::dsl::Filter<saved_workspaces::table, diesel::dsl::Eq<saved_workspaces::workspace_name, &str>> {
  saved_workspaces::table.filter(saved_workspaces::workspace_name.eq(workspace_name))
}

fn get_workspace(connection: &mut SqliteConnection, workspace_name: &str) -> QueryResult<SavedWorkspace> {
  workspace_named(workspace_name).get_result(connection)
}

// An update that changed no rows was given the name of a workspace that doesn't exist
fn expect_one_row(updated_rows: usize) -> QueryResult<()> {
  if updated_rows == 0 {
    return Err(NotFound);
  }
  Ok(())
}

fn save_states(connection: &mut SqliteConnection, workspace_id: i32, states: &[StateData]) -> QueryResult<()> {

  let states_to_be_inserted: Vec<_> = states
    .iter()
    .map(|state| (
      saved_states::workspace_id.eq(workspace_id),
      saved_states::state_id.eq(state.id),
      saved_states::x.eq(state.position.x),
      saved_states::y.eq(state.position.y),
      saved_states::is_start.eq(state.is_start),
      saved_states::is_final.eq(state.is_final)
    ))
    .collect();

  diesel::insert_into(saved_states::table)
    .values(&states_to_be_inserted)
    .execute(connection)?;

  Ok(())

}

fn save_connections(connection: &mut SqliteConnection, workspace_id: i32, connections: &[ConnectionData]) -> QueryResult<()> {

  let connections_to_be_inserted: Vec<_> = connections
    .iter()
    .map(|saved_connection| (
      saved_connections::workspace_id.eq(workspace_id),
      saved_connections::connection_id.eq(saved_connection.id),
      saved_connections::from_state.eq(saved_connection.from),
      saved_connections::to_state.eq(saved_connection.to),
      saved_connections::connection_character.eq(&saved_connection.character),
      saved_connections::control_point_one_x.eq(saved_connection.control_points[0].x),
      saved_connections::control_point_one_y.eq(saved_connection.control_points[0].y),
      saved_connections::control_point_two_x.eq(saved_connection.control_points[1].x),
      saved_connections::control_point_two_y.eq(saved_connection.control_points[1].y)
    ))
    .collect();

  diesel::insert_into(saved_connections::table)
    .values(connections_to_be_inserted)
    .execute(connection)?;

  Ok(())

}
