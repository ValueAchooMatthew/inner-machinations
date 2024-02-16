// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
pub mod schema;
pub mod models;

use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use app::{check_for_user, add_user_to_db};
#[tauri::command]
fn register_user(em: &str, pswd: &str){

  let magic_crypt = new_magic_crypt!("magickey", 256);
  let encrypted_email = magic_crypt.encrypt_str_to_base64(em);
  let encrypted_password = magic_crypt.encrypt_str_to_base64(pswd);
  match check_for_user(&encrypted_email) {
    Some(user) =>{
      println!("User found!");
      println!("ecrypted email: {} encrypted password: {}", user.email, user.password);
      println!("decrypted email: {} decrypted password: {}", magic_crypt.decrypt_base64_to_string(user.email).unwrap(), magic_crypt.decrypt_base64_to_string(user.password).unwrap());
    },
    None => {
      println!("No user found.");
      add_user_to_db(&encrypted_email, &encrypted_password);
    }
  }
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![register_user])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}