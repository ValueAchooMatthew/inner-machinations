use magic_crypt::new_magic_crypt;
use app::{encrypt_user_data, models::User};
use diesel::SqliteConnection;
use app::{establish_connection, get_encryption_key};
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::schema::users;
use crate::diesel::ExpressionMethods;

#[tauri::command(rename_all = "snake_case")]
pub fn register_user(email: &str, password: &str) {
  let key = get_encryption_key();
  let cipher = new_magic_crypt!(&key, 256);
  let [encrypted_email, encrypted_password] = encrypt_user_data(&cipher, email, password);
  add_user_to_db(&encrypted_email, &encrypted_password);
}

#[tauri::command(rename_all = "snake_case")]
pub fn is_correct_log_in(email: &str, password: &str) -> bool {

  let key = get_encryption_key();
  let cipher = new_magic_crypt!(&key, 256);
  let [encrypted_email, encrypted_password] = encrypt_user_data(&cipher, email, password);
  
  let mut conn: SqliteConnection = establish_connection();
  let person: Result<User, diesel::result::Error> = users::table.filter(users::email.eq(encrypted_email))
    .filter(users::password.eq(encrypted_password))
    .get_result::<User>(&mut conn);

  person.ok().is_some()
}

#[tauri::command(rename_all = "snake_case")]
pub fn is_user_registered(email: &str) -> bool {

  let key = get_encryption_key();
  let cipher = new_magic_crypt!(&key, 256);

  let [encrypted_email, _] = encrypt_user_data(&cipher, email, "");
  retrieve_registered_user(&encrypted_email).is_some()
}

fn retrieve_registered_user(email: &str) -> Option<User> {

  let mut conn =  establish_connection();
  users::table
    .filter(users::email.eq(email))
    .limit(1)
    .get_result(&mut conn)
    .ok()

}

fn add_user_to_db(email: &str, password: &str) {
  let mut conn: SqliteConnection = establish_connection();
  let new_user = (users::email.eq(email), users::password.eq(password));
  diesel::insert_into(users::table)
    .values(&new_user)
    .execute(&mut conn)
    .expect("whoopsies, there was an error registering the user!");
}