// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate diesel;
mod schema;
mod db;


// use dotenvy::{dotenv, env, PgConnection};

// fn establish_connection() -> PgConnection {
//   dotenv().ok();
//   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//   PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
// }



#[tauri::command]
fn get_user_data(email: String, password: String) {
  println!("email: {email}, password: {password}");
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![get_user_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
