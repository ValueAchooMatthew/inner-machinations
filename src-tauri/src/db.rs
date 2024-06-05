use std::env;
use dotenv::dotenv;
use magic_crypt::new_magic_crypt;
use app::{add_user_to_db, encrypt_user_data, models::User};
use diesel::MysqlConnection;
use app::establish_connection;
use crate::diesel::query_dsl::methods::FilterDsl;
use crate::diesel::RunQueryDsl;


#[tauri::command]
pub fn register_user(email: &str, password: &str) {

  dotenv().ok();
  let key = env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");
  let cipher = new_magic_crypt!(&key, 256);
  let [encrypted_email, encrypted_password] = encrypt_user_data(&cipher, email, password);
  add_user_to_db(&encrypted_email, &encrypted_password);
}


#[tauri::command]
pub fn is_correct_log_in(email_address: &str, pwrd: &str) -> bool{
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

  person.ok().is_some()
}

