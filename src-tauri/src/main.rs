// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use firebase_rs::*;



// fn retrieveData(name: String) -> String{
//   let firebase = Firebase::auth("https://myfirebase.firebaseio.com", "AUTH_KEY").unwrap();
//   return format!("Hello, {}", name);
// }

fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
