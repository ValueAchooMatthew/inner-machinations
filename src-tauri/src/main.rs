// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
pub mod schema;
pub mod models;

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![handle_registration_event, is_user_registered])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

use magic_crypt::new_magic_crypt;
use app::{add_user_to_db, decrypt_user_data, encrypt_user_data, retrieve_registered_user};
#[tauri::command]
fn handle_registration_event(email: &str, password: &str) -> () {

  let cipher = new_magic_crypt!("magickey", 256);
  let [encrypted_email, encrypted_password] = encrypt_user_data(&cipher, email, password);

  // is_user_registered encrypts the data itself (so it can be called from the front end) so the unencrypted data should
  // be passed into it
  if is_user_registered(&email, &password) {
    let user = retrieve_registered_user(&encrypted_email).unwrap();
    println!("user: {:?}", &user);
    let [decrypted_email, decrypted_password] = decrypt_user_data(&cipher, user);
    println!("The user with email: {} and password: {} has already been registered", decrypted_email, decrypted_password);
    // Technically don't need to check this, since I will call is_user_registered from the front end but provides additional
    // error protection to prevent panics in the case user registration isnt checked beforehand. 

  }else{
    println!("The user has not been registered");
    add_user_to_db(&encrypted_email, &encrypted_password)
    // TODO: Add response and redirect to dashboard
  }

}

#[tauri::command]
fn is_user_registered(email: &str, password: &str) -> bool{
  let cipher = new_magic_crypt!("magickey", 256);
  let [email, _] = encrypt_user_data(&cipher, email, password);
  match retrieve_registered_user(&email){
    Some(_) => true,
    None => false
  }
}