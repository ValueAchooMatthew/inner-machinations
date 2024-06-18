// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
pub mod schema;
pub mod models;
pub mod testing_automata_funcs;
pub mod validation_automata_funcs;
pub mod saving_automata_funcs;
pub mod advanced_automata_funcs;
pub mod db;

use dotenv::dotenv;
use lettre::message::Mailbox;
use std::env;
use db::register_user;
use db::is_correct_log_in;
use advanced_automata_funcs::is_dfa_minimized;
use testing_automata_funcs::{test_string_dfa, test_string_nfa};
use saving_automata_funcs::{save_workspace, delete_workspace, retrieve_workspace_data, get_users_saved_workspaces};
use validation_automata_funcs::verify_valid_dfa;

// Fixed Opsec but should refactor key getting and setting into separate func in lib
fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    register_user, is_user_registered, is_correct_log_in,
    send_email, verify_user, is_user_verified, test_string_dfa,
    test_string_nfa, verify_valid_dfa, save_workspace, delete_workspace, retrieve_workspace_data, 
    get_users_saved_workspaces, is_dfa_minimized
  ]
)
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

use app::schema::users;
use diesel::query_dsl::methods::FilterDsl;
use magic_crypt::new_magic_crypt;
use app::{encrypt_user_data, establish_connection, generate_code, retrieve_registered_user, set_user_code};

#[tauri::command]
fn is_user_registered(email: &str) -> bool {

  dotenv().ok();
  let key = env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);

  let [encrypted_email, _] = encrypt_user_data(&cipher, email, "");
  retrieve_registered_user(&encrypted_email).is_some()
}

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
#[tauri::command]
fn send_email(email_address: &str) -> String {
  let code = generate_code();

  dotenv().ok();
  let key = env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);

  set_user_code(&cipher, &code, email_address);

  let email = Message::builder()
    .from("Matthew <info.innermachinations@gmail.com>".parse().unwrap())
    .to(email_address.parse::<Mailbox>().unwrap())
    .subject("Inner Machinations Verification")
    .header(ContentType::TEXT_PLAIN)
    .body("Please enter the following code to verify your email: ".to_owned() + &code)
    .unwrap();

  let creds = Credentials::new("matthewtamerfarah@gmail.com".to_owned(), "fkyr oetz ethu vqbx".to_owned());
  
  // Open a remote connection to gmail
  let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();
  
  // Send the email
  match mailer.send(&email) {
    Ok(_) => println!("Email sent successfully!"),
    Err(e) => panic!("Could not send email: {e:?}"),
  }
  code

}

use diesel::SqliteConnection;
use diesel::RunQueryDsl;
use models::User;
#[tauri::command]
fn is_user_verified(email_address: &str) -> bool {
  use crate::users::dsl::*;
  use crate::diesel::ExpressionMethods;

  dotenv().ok();
  let key = env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);

  let [encrypted_email, _] = encrypt_user_data(&cipher, email_address, "");
  let mut conn: SqliteConnection = establish_connection();
  let person: Result<User, diesel::result::Error> = users
    .filter(email.eq(encrypted_email))
    .filter(verified.eq(true))
    .get_result::<User>(&mut conn);

  match person {
    Ok(_person) => {
      true
    },
    Err(_) => false
  }
}

#[tauri::command]
fn verify_user(email_address: &str){
  use crate::users::dsl::*;
  use crate::diesel::ExpressionMethods;

  let key = env::var("ENCRYPTION_KEY").ok()
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);

  let [encrypted_email, _] = encrypt_user_data(&cipher, email_address, "");
  let mut conn: SqliteConnection = establish_connection();

  diesel::update(users)
    .filter(email.eq(encrypted_email))
    .set(verified.eq(true))
    .execute(&mut conn)
    .expect("There was an error assigning a code for the user");
}
