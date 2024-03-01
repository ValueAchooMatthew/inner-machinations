// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate diesel;
pub mod schema;
pub mod models;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

// Fixed Opsec but should refactor key getting and setting into separate func in lib
fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![register_user, is_user_registered, is_correct_log_in, send_email, verify_user, is_user_verified, get_links])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

use app::schema::users;
use diesel::query_dsl::methods::FilterDsl;
use magic_crypt::new_magic_crypt;
use app::{add_user_to_db, encrypt_user_data, establish_connection, generate_code, retrieve_registered_user, set_user_code};
#[tauri::command]
fn register_user(email: &str, password: &str) -> () {

  dotenv().ok();
  let key = env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);
  let [encrypted_email, encrypted_password] = encrypt_user_data(&cipher, email, password);
  add_user_to_db(&encrypted_email, &encrypted_password)

}

#[tauri::command]
fn is_correct_log_in(email_address: &str, pwrd: &str) -> bool{
  use crate::users::dsl::*;
  use crate::diesel::ExpressionMethods;

  dotenv().ok();
  let key = env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);
  let [encrypted_email, encrypted_password] = encrypt_user_data(&cipher, email_address, pwrd);
  
  let mut conn: MysqlConnection = establish_connection();
  let person: Result<User, diesel::result::Error> = users.filter(email.eq(encrypted_email))
    .filter(password.eq(encrypted_password))
    .get_result::<User>(&mut conn);


  // Eventually, I will need to implement either a JWT or cookie system to persist user log-in but for now this is okay as a
  // minimum viable product 
  match person.ok(){
    Some(_) => true,
    None => false
  }
}


// TODO: Fix way in which encryption is done
#[tauri::command]
fn is_user_registered(email: &str) -> bool {

  dotenv().ok();
  let key = env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);

  let [encrypted_email, _] = encrypt_user_data(&cipher, email, "");
  match retrieve_registered_user(&encrypted_email){
    Some(_) => true,
    None => false
  }
}

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
// TODO: Add sha-256 encryption to emails
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
    .to(email_address.parse().unwrap())
    .subject("Inner Machinations Verification")
    .header(ContentType::TEXT_PLAIN)
    .body(String::from("Please enter the following code to verify your email: ".to_owned() + &code))
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
  return code;

}

use diesel::MysqlConnection;
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
  let mut conn: MysqlConnection = establish_connection();
  let person: Result<User, diesel::result::Error> = users.filter(email.eq(encrypted_email))
    .get_result::<User>(&mut conn);
  
  let person = person.ok().unwrap();

  return person.verified;

}

#[tauri::command]
fn verify_user(email_address: &str) -> (){
  use crate::users::dsl::*;
  use crate::diesel::ExpressionMethods;

  let key = env::var("ENCRYPTION_KEY").ok()
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);

  let [encrypted_email, _] = encrypt_user_data(&cipher, email_address, "");
  let mut conn: MysqlConnection = establish_connection();

  diesel::update(users)
    .filter(email.eq(encrypted_email))
    .set(verified.eq(true))
    .execute(&mut conn)
    .expect("There was an error assigning a code for the user");
}

// use crate::models::Node;
use crate::models::Node;
#[tauri::command]
fn get_links(state_connections: HashMap<String, Node>, start_state_coordinates: String) -> (){
  println!("{}", start_state_coordinates);

  let start_node: &Node = state_connections.get(&start_state_coordinates).unwrap();

  println!("{:?}", start_node);

}
