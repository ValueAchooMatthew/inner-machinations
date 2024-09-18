use diesel::{ExpressionMethods, SqliteConnection};
use magic_crypt::new_magic_crypt;
use crate::{diesel::query_dsl::methods::{FilterDsl, LimitDsl}, miscellaneous::database_models_and_utilities::{establish_connection, User}, 
schema::users};
use diesel::RunQueryDsl;

use magic_crypt::{MagicCrypt256, MagicCryptTrait};

impl User {
  // Changes both the number stored in the object itself and the database
  // Get and update methods tied together as I do not expect for retrieval of this info
  // To ever occur without updating
  pub fn get_and_update_number_of_untitled_regular_automata_workspaces(&mut self) -> i32 {

    self.number_of_untitled_regular_automata_workspaces += 1;
    let mut conn = establish_connection();

    diesel::update(users::table
      .filter(users::id.eq(self.id)))
      .set(users::number_of_untitled_regular_automata_workspaces.eq(self.number_of_untitled_regular_automata_workspaces))
      .execute(&mut conn)
      .expect("There was an error updating the number of untitled regular automata workspaces in the database");

    self.number_of_untitled_regular_automata_workspaces

  }

  pub fn get_and_update_number_of_untitled_regex_workspaces(&mut self) -> i32 {

    self.number_of_untitled_regex_workspaces += 1;
    let mut conn = establish_connection();

    diesel::update(users::table
      .filter(users::id.eq(self.id)))
      .set(users::number_of_untitled_regex_workspaces.eq(self.number_of_untitled_regex_workspaces))
      .execute(&mut conn)
      .expect("There was an error updating the number of untitled regex workspaces in the database");

    self.number_of_untitled_regex_workspaces
  }

}

pub fn get_user_id(email: &str, conn: &mut SqliteConnection) -> i32 {

  let key = std::env::var("ENCRYPTION_KEY")
    .expect("Encryption Key must be set as a .env variable");

  let cipher = new_magic_crypt!(&key, 256);
  let [encrypted_user_email, _ ] = encrypt_user_data(&cipher, &email, "");

  let user: User = users::table
    .filter(users::email.eq(&encrypted_user_email))
    .limit(1)
    .get_result::<User>(conn)
    .expect("There was an error retrieving the user's id");

  user.id
}

pub fn get_user(user_id: i32, conn: &mut SqliteConnection) -> User {

  users::table
    .filter(users::id.eq(user_id))
    .limit(1)
    .get_result::<User>(conn)
    .expect("There was an error retrieving the user")

}

pub fn encrypt_user_data(cipher: &MagicCrypt256, email: &str, password: &str) -> [String; 2] {
  let encrypted_email = cipher.encrypt_str_to_base64(email);
  let encrypted_password = cipher.encrypt_str_to_base64(password);
  [encrypted_email, encrypted_password]
}