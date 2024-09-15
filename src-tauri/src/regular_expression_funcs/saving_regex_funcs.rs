use app::get_user;
use app::models::RegexWorkspaceData;
use app::{establish_connection, get_user_id, models::SavedRegexWorkspace};
use chrono::NaiveDateTime;
use diesel::query_dsl::methods::{FilterDsl, LimitDsl};
use diesel::{ExpressionMethods, RunQueryDsl, SqliteConnection};
use crate::schema::saved_regex_workspaces;

#[tauri::command(rename_all = "snake_case")]
// Returns name of newly created workspace for use in frontend
pub fn create_regex_workspace(email: &str) -> String {
  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  let number_of_untitled_projects = get_user(user_id, &mut conn)
    .get_and_update_number_of_untitled_regex_workspaces();

  let mut new_workspace_name = format!("Untitled Project #{number_of_untitled_projects}");
  
  if number_of_untitled_projects == 1 {
    new_workspace_name = String::from("Untitled Project");
  }

  // To handle edge case where user has already named a project to be some variant of Untitled Project or Untitled Project # we cycle through
  // workspace numbers until one does not yet exist in the database

  while does_regex_workspace_name_exist(email, &new_workspace_name) {
    let number_of_untitled_projects =  get_user(user_id, &mut conn)
      .get_and_update_number_of_untitled_regex_workspaces();
    new_workspace_name = format!("Untitled Project #{number_of_untitled_projects}");
  }

  let new_saved_regex = (
    saved_regex_workspaces::user_id.eq(user_id),
    saved_regex_workspaces::regex_name.eq(&new_workspace_name),
    saved_regex_workspaces::date_of_last_update.eq::<NaiveDateTime>(chrono::offset::Local::now().naive_local())
  );

  diesel::insert_into(saved_regex_workspaces::table)
    .values(new_saved_regex)
    .execute(&mut conn)
    .expect("There was an error creating the regex");

  println!("Creating new saved regex {}", new_workspace_name);

  new_workspace_name
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_regex_workspace(regex: &str, regex_name: &str, email: &str) {
  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  diesel::update(saved_regex_workspaces::table
    .filter(saved_regex_workspaces::regex_name.eq(regex_name))
    .filter(saved_regex_workspaces::user_id.eq(&user_id))
  )
  .set((saved_regex_workspaces::regex.eq(regex),
  saved_regex_workspaces::date_of_last_update.eq::<NaiveDateTime>(chrono::offset::Local::now().naive_local())))
  .execute(&mut conn)
  .expect("Could not update the requested regex");
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_regex_workspace_name(email: &str, original_regex_name: &str, new_regex_name: &str) {
  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  diesel::update(saved_regex_workspaces::table
    .filter(saved_regex_workspaces::user_id.eq(&user_id))
    .filter(saved_regex_workspaces::regex_name.eq(original_regex_name))
  )
  .set(saved_regex_workspaces::regex_name.eq(new_regex_name))
  .execute(&mut conn)
  .expect("Could not change the regex name");
}

#[tauri::command(rename_all = "snake_case")]
pub fn does_regex_workspace_name_exist(email: &str, regex_name: &str) -> bool {
  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  return get_saved_regex(&user_id, regex_name, &mut conn).is_ok();
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_users_regex_workspace_names(email: &str) -> Vec<String> {
  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  let saved_regex_workspaces: Vec<SavedRegexWorkspace> = saved_regex_workspaces::table
    .filter(saved_regex_workspaces::user_id.eq(user_id))
    .get_results(&mut conn)
    .expect("There was an error retrieving the user's saved regex workspaces");

  saved_regex_workspaces
    .iter()
    .map(|workspace| {
      workspace.regex_name.to_owned()
    })
    .collect()
}

#[tauri::command(rename_all = "snake_case")]
pub fn retrieve_regex_workspace_data(email: &str, regex_name: &str) -> RegexWorkspaceData {
  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  let saved_regex_workspace = saved_regex_workspaces::table
    .filter(saved_regex_workspaces::user_id.eq(&user_id))
    .filter(saved_regex_workspaces::regex_name.eq(regex_name))
    .limit(1)
    .get_result(&mut conn)
    .expect("There was an error retrieving the requested regex workspace data");

  RegexWorkspaceData::new(saved_regex_workspace)
}

#[tauri::command(rename_all = "snake_case")]
pub fn delete_regex_workspace(email: &str, workspace_name: &str) {
  let mut conn = establish_connection();
  let user_id = get_user_id(email, &mut conn);

  let regex_workspace_to_delete = get_saved_regex(&user_id,workspace_name, &mut conn);

  diesel::delete(&regex_workspace_to_delete.unwrap())
    .execute(&mut conn)
    .expect("There was a problem deleting the requested regex workspace");
}

fn get_saved_regex(user_id: &i32, regex_name: &str, conn: &mut SqliteConnection) -> Result<SavedRegexWorkspace, diesel::result::Error> {

  saved_regex_workspaces::table
    .filter(saved_regex_workspaces::regex_name.eq(regex_name))
    .filter(saved_regex_workspaces::user_id.eq(&user_id))
    .get_result::<SavedRegexWorkspace>(conn)
}
