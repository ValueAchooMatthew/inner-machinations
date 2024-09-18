use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, SqliteConnection};
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::miscellaneous::common_models::State;
use crate::miscellaneous::database_models_and_utilities::establish_connection;
use crate::miscellaneous::database_models_and_utilities::SavedRegularAutomataWorkspace;
use crate::miscellaneous::database_models_and_utilities::TypeOfAutomata;
use crate::miscellaneous::miscellaneous_utilities::sanitize_input_alphabet;
use crate::schema::saved_regular_automata_connections;
use crate::schema::saved_regular_automata_workspaces;
use crate::schema::saved_states;
use crate::user_flow_funcs::user_models::get_user;
use crate::user_flow_funcs::user_models::get_user_id;

use super::regular_automata_models::RegularAutomataWorkspaceData;
use super::regular_automata_models::RegularAutomatonConnection;

#[tauri::command(rename_all = "snake_case")]
pub fn create_regular_automata_workspace(email: &str) -> String {

  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  let number_of_untitled_projects = get_user(user_id, &mut conn)
    .get_and_update_number_of_untitled_regular_automata_workspaces();

  let mut new_workspace_name = format!("Untitled Project #{number_of_untitled_projects}");

  if number_of_untitled_projects == 1 {
    new_workspace_name = String::from("Untitled Project");
  }

  // To handle edge case where user has already named a project to be some variant of Untitled Project or Untitled Project # we cycle through
  // workspace numbers until one does not yet exist in the database

  while does_regular_automata_workspace_name_exist(&new_workspace_name, email) {
    let number_of_untitled_projects =  get_user(user_id, &mut conn)
      .get_and_update_number_of_untitled_regular_automata_workspaces();
    new_workspace_name = format!("Untitled Project #{number_of_untitled_projects}");
  }

  let new_saved_automata = (
    saved_regular_automata_workspaces::user_id.eq(&user_id), 
    saved_regular_automata_workspaces::workspace_name.eq(&new_workspace_name),
    saved_regular_automata_workspaces::type_of_automata.eq(&TypeOfAutomata::DFA)
  );

  diesel::insert_into(saved_regular_automata_workspaces::table)
    .values(new_saved_automata)
    .execute(&mut conn)
    .expect("There was an error creating the workspace");

  println!("Creating new workspace {}", new_workspace_name);

  new_workspace_name
  
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_regular_automata_workspace_alphabet(workspace_name: &str, email: &str, alphabet: Vec<&str>) -> Vec<String> {
  let mut conn: SqliteConnection = establish_connection();
  let user_id =  get_user_id(&email, &mut conn);
  let workspace = get_regular_automata_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve workspace");

  let sanitized_input_alphabet = sanitize_input_alphabet(alphabet); 
  
  diesel::update(&workspace)
    .set(saved_regular_automata_workspaces::alphabet.eq(sanitized_input_alphabet.join(",")))
    .execute(&mut conn)
    .expect("There was an error updating the workspace's alphabet");

  return sanitized_input_alphabet;
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_regular_automata_workspace(
  workspace_name: &str,
  states: HashMap<String, State>,
  email: &str,
  connections: Vec<RegularAutomatonConnection>
) {

  let mut conn = establish_connection();
  let user_id = get_user_id(&email, &mut conn);
  let workspace: SavedRegularAutomataWorkspace = get_regular_automata_workspace(&workspace_name, &user_id, &mut conn)
    .expect("There was an error retrieving the workspace");

  save_states(&workspace.id, &states, &mut conn)
    .expect("There was an error saving the states to the database");

  save_regular_automata_connections(&workspace.id, &connections, &mut conn)
    .expect("There was an error saving the connections to the database");

  update_last_updated_workspace_time(&workspace.id, &mut conn)
    .expect("There was an error updating the last modified time of the workspace");

  println!("Saved!");
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_regular_automata_workspace(workspace_name: &str, email: &str) {

  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);
  let workspace: SavedRegularAutomataWorkspace = get_regular_automata_workspace(&workspace_name, &user_id, &mut conn)
    .expect("There was an eror retrieving the workspace");

  diesel::delete(saved_states::table)
    .filter(saved_states::workspace_id.eq(workspace.id))
    .execute(&mut conn)
    .expect("There was an error deleting the old workspace's previous states");

  diesel::delete(saved_regular_automata_connections::table)
    .filter(saved_regular_automata_connections::workspace_id.eq(workspace.id))
    .execute(&mut conn)
    .expect("There was an error deleting the old workspace's previous connections");

  diesel::delete(saved_regular_automata_workspaces::table)
    .filter(saved_regular_automata_workspaces::user_id.eq(&user_id))
    .filter(saved_regular_automata_workspaces::workspace_name.eq(&workspace_name))
    .execute(&mut conn)
    .expect("There was an error deleting the workspace from the db");

  println!("Deleted workspace {workspace_name}")
}

#[tauri::command(rename_all = "snake_case")]
pub fn does_regular_automata_workspace_name_exist(workspace_name: &str, email: &str) -> bool {
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);
  return get_regular_automata_workspace(&workspace_name, &user_id, &mut conn).is_ok()
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_regular_automata_workspace_name(original_workspace_name: &str, email: &str, new_workspace_name: &str) {
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_regular_automata_workspace(&original_workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_regular_automata_workspaces::workspace_name.eq(new_workspace_name))
    .execute(&mut conn)
    .expect("Could not update the workspace name");
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_regular_automata_type(workspace_name: &str, email: &str, type_of_automata: TypeOfAutomata) {

  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_regular_automata_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_regular_automata_workspaces::type_of_automata.eq(type_of_automata))
    .execute(&mut conn)
    .expect("Could not update the automata type of the workspace");
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_strict_checking(workspace_name: &str, email: &str, should_strict_check: bool) {
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_regular_automata_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_regular_automata_workspaces::should_strict_check.eq(should_strict_check))
    .execute(&mut conn)
    .expect("Could not update the strict checking option for the workspace");
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_showing_string_traversal(workspace_name: &str, email: &str, should_show_traversal: bool) {
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_regular_automata_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_regular_automata_workspaces::should_show_string_traversal.eq(should_show_traversal))
    .execute(&mut conn)
    .expect("Could not update string traversal option for the workspace");
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_default_connection_character(workspace_name: &str, email: &str, default_connection_character: String) {
  // Should sanitize that string passed in is single character
  // Also should consider switching to using char datatype. using string for consistency with ts frontend

  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_regular_automata_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_regular_automata_workspaces::default_connection_character.eq(default_connection_character))
    .execute(&mut conn)
    .expect("Could not update the default connection character for the workspace");
}

#[tauri::command(rename_all = "snake_case")]
pub fn retrieve_regular_automata_workspace_data(workspace_name: &str, email: &str) -> RegularAutomataWorkspaceData {
    
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);
  let workspace = get_regular_automata_workspace(&workspace_name, &user_id, &mut conn)
    .expect("There was an error retrieving the workspace");

  RegularAutomataWorkspaceData::new(workspace)
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_users_regular_automata_workspace_names(email: &str) -> Vec<String> {

  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  let retrieved_regular_automata_workspaces: Vec<SavedRegularAutomataWorkspace> = saved_regular_automata_workspaces::table
    .filter(saved_regular_automata_workspaces::user_id.eq(&user_id))
    .get_results(&mut conn)
    .expect("There was an error retrieving the user's saved regular automata workspaces");

   retrieved_regular_automata_workspaces
    .iter()
    .map(|workspace| workspace.workspace_name.to_owned())
    .collect()
}

fn get_regular_automata_workspace(workspace_name: &str, user_id: &i32, conn: &mut SqliteConnection) -> Result<SavedRegularAutomataWorkspace, diesel::result::Error> {
  
  saved_regular_automata_workspaces::table
    .filter(saved_regular_automata_workspaces::user_id.eq(user_id))
    .filter(saved_regular_automata_workspaces::workspace_name.eq(&workspace_name))
    .limit(1)
    .get_result::<SavedRegularAutomataWorkspace>(conn)
}

fn save_states(workspace_id: &i32, states: &HashMap<String, State>, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
  // First step is to remove all existing states corresponding to the workspace
  diesel::delete(saved_states::table)
    .filter(saved_states::workspace_id.eq(&workspace_id))
    .execute(conn)?;
  
  // Second step is to add the states from the hashmap and associate each one to the existing state id
  // A state can have 0 inclusive connected states, so we will make one entry for the state itself with no connections,
  // and one for each state it is conencted to

  let mut states_to_be_inserted =  vec![];

  for (state_pos_key, state) in states {
    // Inserting state in the case a state has no connections
    states_to_be_inserted.push((
      saved_states::workspace_id.eq(workspace_id),
      saved_states::position.eq(state_pos_key),
      saved_states::is_start.eq(state.is_start()),
      saved_states::is_final.eq(state.is_final()),
    ));
  }

  diesel::insert_into(saved_states::table)
    .values(&states_to_be_inserted)
    .execute(conn)?;

  Ok(())

}

fn save_regular_automata_connections(workspace_id: &i32, connections: &Vec<RegularAutomatonConnection>, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {

  // First delete all existing connections relating to the current automata
  diesel::delete(saved_regular_automata_connections::table)
    .filter(saved_regular_automata_connections::workspace_id.eq(workspace_id))
    .execute(conn)?;

  let mut connections_to_be_inserted = vec![];

  for connection in connections{
    let connection_to_be_inserted = (
      saved_regular_automata_connections::workspace_id.eq(workspace_id),
      saved_regular_automata_connections::connection_character.eq(&connection.connection_character),
      saved_regular_automata_connections::start_point.eq::<String>(connection.curve.start_point.into()),
      saved_regular_automata_connections::control_point_one.eq::<String>(connection.curve.control_point_one.into()),
      saved_regular_automata_connections::control_point_two.eq::<String>(connection.curve.control_point_two.into()),
      saved_regular_automata_connections::end_point.eq::<String>(connection.curve.end_point.into())
    );
    connections_to_be_inserted.push(connection_to_be_inserted);
  }

  // Second step is inserting connections into connections table

  diesel::insert_into(saved_regular_automata_connections::table)
    .values(connections_to_be_inserted)
    .execute(conn)?;

  return Ok(());
}

fn update_last_updated_workspace_time(workspace_id: &i32, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {

  // Sets time of last update to current time
  diesel::update(saved_regular_automata_workspaces::table
    .filter(saved_regular_automata_workspaces::id.eq(workspace_id)))
    .set(saved_regular_automata_workspaces::date_of_last_update.eq::<NaiveDateTime>(chrono::offset::Local::now().naive_local()))
    .execute(conn)?;

  Ok(())
}