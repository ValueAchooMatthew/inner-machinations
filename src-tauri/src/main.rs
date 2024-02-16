// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
pub mod schema;
pub mod models;
use crate::diesel::*;

#[tauri::command]
fn print_users(){
  use crate::models::User;
  use crate::schema::users::dsl::*;

  let connection = &mut establish_connection();
  let results = users
      .limit(5)
      .select(User::as_select())
      .load(connection)
      .expect("Error loading users");

  println!("Displaying {} users", results.len());
  for user in results {
      println!("{}", user.email);
      println!("-----------\n");
      println!("{}", user.password);
  }

}

use app::establish_connection;
use diesel::{RunQueryDsl, SelectableHelper};
use self::models::Register;
#[tauri::command]
fn register_user(email: &str, password: &str) -> () {
  let mut conn = establish_connection();
  use crate::schema::users;
  let new_user = Register { email, password };
  diesel::insert_into(users::table)
      .values(&new_user)
      .execute(&mut conn)
      .expect("whoopsies, there was an error registering the user!");
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![register_user, print_users])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}
