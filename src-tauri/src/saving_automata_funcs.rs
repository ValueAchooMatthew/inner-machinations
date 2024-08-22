use std::collections::HashMap;

use app::{encrypt_user_data, sanitize_input_alphabet, establish_connection, models::State};
use app::models::{Connection, SavedWorkspace, TypeOfAutomata, User, WorkspaceData};
use app::schema::{saved_connections, users, saved_states, saved_workspaces};

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, SqliteConnection};
use magic_crypt::new_magic_crypt;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

#[tauri::command]
pub fn create_workspace(email: String, workspace_name: String) {

  let mut conn = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let new_saved_automata = (
    saved_workspaces::user_id.eq(&user_id), 
    saved_workspaces::workspace_name.eq(&workspace_name),
    saved_workspaces::type_of_automata.eq(&TypeOfAutomata::DFA)
  );

  diesel::insert_into(saved_workspaces::table)
    .values(new_saved_automata)
    .execute(&mut conn)
    .expect("There was an error creating the workspace");

    println!("Creating new workspace {}", &workspace_name);
  
  saved_workspaces::table
    .filter(saved_workspaces::workspace_name.eq(workspace_name))
    .limit(1)
    .get_result::<SavedWorkspace>(&mut conn)
    .expect("There was an error retrieving the workspace");

}

#[tauri::command]
pub fn update_workspace_alphabet(workspace_name: String, email: String, alphabet: Vec<String>) -> Vec<String> {
  let mut conn: SqliteConnection = establish_connection();
  let user_id =  get_user_id(&email, &mut conn);
  let workspace = get_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve workspace");

  let sanitized_input_alphabet = sanitize_input_alphabet(alphabet); 
  
  diesel::update(&workspace)
    .set(saved_workspaces::alphabet.eq(sanitized_input_alphabet.join(",")))
    .execute(&mut conn)
    .expect("There was an error updating the workspace's alphabet");

  return sanitized_input_alphabet;

}

#[tauri::command]
pub fn save_workspace(workspace_name: String, 
  states: HashMap<String, State>, 
  email: String, 
  connections: Vec<Connection>) {

  let mut conn = establish_connection();
  let user_id = get_user_id(&email, &mut conn);
  let workspace: SavedWorkspace = get_workspace(&workspace_name, &user_id, &mut conn)
    .expect("There was an error retrieving the workspace");

  save_states_to_db(&workspace.id, &states, &mut conn)
    .expect("There was an error saving the states to the database");

  save_connections_to_db(&workspace.id, &connections, &mut conn)
    .expect("There was an error saving the connections to the database");

  set_current_time(&workspace.id, &mut conn)
    .expect("There was an error updating the last modified time of the workspace");

  println!("Saved!");

}

#[tauri::command]
pub fn delete_workspace(workspace_name: String, email: String) {

  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);
  let workspace: SavedWorkspace = get_workspace(&workspace_name, &user_id, &mut conn)
    .expect("There was an eror retrieving the workspace");

  diesel::delete(saved_states::table)
    .filter(saved_states::workspace_id.eq(workspace.id))
    .execute(&mut conn)
    .expect("There was an error deleting the old workspace's previous states");

  diesel::delete(saved_connections::table)
    .filter(saved_connections::workspace_id.eq(workspace.id))
    .execute(&mut conn)
    .expect("There was an error deleting the old workspace's previous connections");

  diesel::delete(saved_workspaces::table)
    .filter(saved_workspaces::user_id.eq(&user_id))
    .filter(saved_workspaces::workspace_name.eq(&workspace_name))
    .execute(&mut conn)
    .expect("There was an error deleting the workspace from the db");

  println!("Deleted workspace {workspace_name}")
}

#[tauri::command]
pub fn does_workspace_name_exist(workspace_name: String, email: String) -> bool {
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);
  return get_workspace(&workspace_name, &user_id, &mut conn).is_ok()
}

#[tauri::command]
pub fn update_workspace_name(original_workspace_name: String, email: String, new_workspace_name: String) {
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_workspace(&original_workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_workspaces::workspace_name.eq(new_workspace_name))
    .execute(&mut conn)
    .expect("Could not update the workspace name");
}

#[tauri::command]
pub fn update_automata_type(workspace_name: String, email: String, type_of_automata: TypeOfAutomata) {

  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_workspaces::type_of_automata.eq(type_of_automata))
    .execute(&mut conn)
    .expect("Could not update the automata type of the workspace");

}

#[tauri::command]
pub fn update_strict_checking(workspace_name: String, email: String, should_strict_check: bool) {
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_workspaces::should_strict_check.eq(should_strict_check))
    .execute(&mut conn)
    .expect("Could not update the strict checking option for the workspace");
}

#[tauri::command]
pub fn update_showing_string_traversal(workspace_name: String, email: String, should_show_traversal: bool) {
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_workspaces::should_show_string_traversal.eq(should_show_traversal))
    .execute(&mut conn)
    .expect("Could not update string traversal option for the workspace");

}

#[tauri::command]
pub fn update_default_connection_character(workspace_name: String, email: String, default_connection_character: String) {
  // Should sanitize that string passed in is single character
  // Also should consider switching to using char datatype. using string for consistency with ts frontend

  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let workspace = get_workspace(&workspace_name, &user_id, &mut conn)
    .expect("Could not retrieve the requested workspace");

  diesel::update(&workspace)
    .set(saved_workspaces::default_connection_character.eq(default_connection_character))
    .execute(&mut conn)
    .expect("Could not update the default connection character for the workspace");

}

#[tauri::command]
pub fn retrieve_workspace_data(workspace_name: String, email: String) -> WorkspaceData {
    
  let mut conn: SqliteConnection = establish_connection();
  let user_id = get_user_id(&email, &mut conn);
  let workspace = get_workspace(&workspace_name, &user_id, &mut conn)
    .expect("There was an error retrieving the workspace");

  let workspace_data = WorkspaceData::new(workspace);

  return workspace_data;

}

#[tauri::command]
pub fn get_users_saved_workspaces(email: String) -> Vec<String> {

  let mut conn = establish_connection();
  let user_id = get_user_id(&email, &mut conn);

  let retrieved_workspaces: Vec<SavedWorkspace> = saved_workspaces::table
    .filter(saved_workspaces::user_id.eq(&user_id))
    .get_results(&mut conn)
    .expect("There was an error retrieving the user's saved workspaces");

  let workspace_names: Vec<String> = retrieved_workspaces
    .iter()
    .map(|workspace| workspace.workspace_name.to_owned())
    .collect();

  return workspace_names.to_owned();

}

fn get_workspace(workspace_name: &String, user_id: &i32, conn: &mut SqliteConnection) -> Result<SavedWorkspace, diesel::result::Error> {
  
  saved_workspaces::table
    .filter(saved_workspaces::user_id.eq(user_id))
    .filter(saved_workspaces::workspace_name.eq(&workspace_name))
    .limit(1)
    .get_result::<SavedWorkspace>(conn)

}

fn save_states_to_db(workspace_id: &i32, states: &HashMap<String, State>, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
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

fn get_user_id(email: &String, conn: &mut SqliteConnection) -> i32 {

  let key = std::env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");

  let cipher = new_magic_crypt!(&key, 256);
  let [encrypted_user_email, _ ] = encrypt_user_data(&cipher, &email, "");

  let user: User = users::table
    .filter(users::email.eq(&encrypted_user_email))
    .limit(1)
    .get_result::<User>(conn)
    .expect("There was an error finding the user's profile");

  user.id
}

fn save_connections_to_db(workspace_id: &i32, connections: &Vec<Connection>, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {

  // First delete all existing connections relating to the current automata
  diesel::delete(saved_connections::table)
    .filter(saved_connections::workspace_id.eq(workspace_id))
    .execute(conn)?;

  let mut connections_to_be_inserted = vec![];

  for connection in connections{
    let connection_to_be_inserted = (
      saved_connections::workspace_id.eq(workspace_id),
      saved_connections::connection_character.eq(&connection.connection_character),
      saved_connections::start_point.eq::<String>(connection.curve.start_point.into()),
      saved_connections::control_point_one.eq::<String>(connection.curve.control_point_one.into()),
      saved_connections::control_point_two.eq::<String>(connection.curve.control_point_two.into()),
      saved_connections::end_point.eq::<String>(connection.curve.end_point.into())
    );
    connections_to_be_inserted.push(connection_to_be_inserted);
  }

  // Second step is inserting connections into connections table

  diesel::insert_into(saved_connections::table)
    .values(connections_to_be_inserted)
    .execute(conn)?;

  return Ok(());

}



fn set_current_time(workspace_id: &i32, conn: &mut SqliteConnection) -> Result<(), diesel::result::Error> {

  // Sets time of last update to current time
  diesel::update(saved_workspaces::table
    .filter(saved_workspaces::id.eq(workspace_id)))
    .set(saved_workspaces::date_of_last_update.eq::<NaiveDateTime>(chrono::offset::Local::now().naive_local()))
    .execute(conn)?;

  Ok(())
}
