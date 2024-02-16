extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
      MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use models::User;
pub fn check_for_user(em: &str) -> Option<User> {
  use crate::schema::users::dsl::*;

  let mut connection =  establish_connection();
  let mut result: Vec<User> = users
  .filter(email.eq(em))
  .limit(1)
  .load(&mut connection)
  .expect("whoops, there was an error checking for the user!");

  match result.pop() {
      Some(user) => Some(user),
      None => None
  }
}

use diesel::RunQueryDsl;
use crate::models::Register;
use crate::schema::users;
pub fn add_user_to_db(email: &str, password: &str) -> () {
  let mut conn: MysqlConnection = establish_connection();
  let new_user = Register { email, password };
  diesel::insert_into(users::table)
      .values(&new_user)
      .execute(&mut conn)
      .expect("whoopsies, there was an error registering the user!");
}