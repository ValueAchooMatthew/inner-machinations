extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;
pub mod testing_automata_funcs;
pub mod advanced_automata_funcs;
mod language_determination_funcs;

use std::fs;
use std::io::Write;
use std::env;

use diesel::{prelude::*, Connection as DieselConnection};
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
  SqliteConnection::establish(&"mydb.sqlite3")
    .unwrap_or_else(|_| panic!("Error connecting to database"))
}

use models::User;
pub fn retrieve_registered_user(em: &str) -> Option<User> {
  use crate::schema::users::dsl::*;

  let mut connection =  establish_connection();
  let mut result: Vec<User> = users
    .filter(email.eq(em))
    .limit(1)
    .load(&mut connection)
    .expect("whoops, there was an error checking for the user!");
  
  result.pop()
}

use diesel::RunQueryDsl;
use crate::schema::users;
pub fn add_user_to_db(email: &str, password: &str) {
  let mut conn: SqliteConnection = establish_connection();
  let new_user = (users::email.eq(email), users::password.eq(password));
  diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&mut conn)
    .expect("whoopsies, there was an error registering the user!");
}

use magic_crypt::{MagicCrypt256, MagicCryptTrait};
pub fn encrypt_user_data(cipher: &MagicCrypt256, email: &str, password: &str) -> [String; 2] {
  let encrypted_email = cipher.encrypt_str_to_base64(email);
  let encrypted_password = cipher.encrypt_str_to_base64(password);
  [encrypted_email, encrypted_password]
}

pub fn decrypt_user_data(cipher: &MagicCrypt256, user: User) -> [String; 2]{
  let decrypted_email = cipher.decrypt_base64_to_string(user.email).unwrap();
  let decrypted_password = cipher.decrypt_base64_to_string(user.password).unwrap();
  [decrypted_email, decrypted_password]
}

use rand::{distributions::Alphanumeric, Rng}; // 0.8
pub fn generate_code() -> String {
  let code: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(6)
    .map(char::from)
    .collect();
  code
}

pub fn set_user_code(cipher: &MagicCrypt256, generated_code: &str, email_address: &str){
  use crate::users::dsl::*;

  let [encrypted_email, _] = encrypt_user_data(cipher, email_address, "");

  let mut conn: SqliteConnection = establish_connection();
  diesel::update(users)
    .filter(email.eq(encrypted_email))
    .set(code.eq(generated_code))
    .execute(&mut conn)
    .expect("There was an error assigning a code for the user");
}

pub fn get_encryption_key() -> String {

  let mut env_file = dotenv::dotenv();

  if env_file.is_err() {
    let mut new_env_file = fs::File
    ::create_new(".env")
    .unwrap();

    let random_string: String = rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(12)
      .map(char::from)
      .collect();

    let env_variable = String::from("ENCRYPTION_KEY=") + format!("{random_string}").as_str();
    new_env_file.write(env_variable.as_bytes())
      .expect("There was an error writing to the env file");

    env_file = dotenv::dotenv();
  }
  
  env_file.ok();

  env::var("ENCRYPTION_KEY")
    .expect("There was an error retrieving the encryption key")

}

pub fn set_working_directory() {

  let path_to_executable = env::current_exe()
    .expect("There was an error retrieving the path to the executable");

  let exectable_directory = path_to_executable.parent();

  env::set_current_dir(exectable_directory.unwrap())
    .expect("There was an error setting the working directory");

}