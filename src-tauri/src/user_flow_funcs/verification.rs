use diesel::{RunQueryDsl, SqliteConnection};
use diesel::query_dsl::methods::FilterDsl;
use lettre::{message::{header::ContentType, Mailbox}, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCrypt256;
use crate::miscellaneous::database_models_and_utilities::User;
use crate::{diesel::ExpressionMethods, miscellaneous::{database_models_and_utilities::establish_connection, environment::get_encryption_key}, schema::users};

#[tauri::command(rename_all = "snake_case")]
pub fn send_verification_email(email: &str) -> Option<String> {

  let code = generate_verification_code();
  let key = get_encryption_key();
  let cipher = magic_crypt::new_magic_crypt!(&key, 256);

  update_users_verification_code(&cipher, &code, email);

  let email = Message::builder()
    .from("Matthew <info.innermachinations@gmail.com>".parse().unwrap())
    .to(email.parse::<Mailbox>().unwrap())
    .subject("Inner Machinations Verification")
    .header(ContentType::TEXT_PLAIN)
    .body("Please enter the following code to verify your email: ".to_owned() + &code)
    .unwrap();

  let creds = Credentials::new("matthewtamerfarah@gmail.com".to_owned(), "djdw kldc kuki gwsf".to_owned());
  
  // Open a remote connection to gmail
  let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();
  
  // Send the email
  match mailer.send(&email) {
    Ok(_) => println!("Email sent successfully!"),
    Err(_) => return None
  }
  return Some(code);

}

#[tauri::command(rename_all = "snake_case")]
pub fn verify_user(email: &str){

  let key = get_encryption_key();
  let cipher = new_magic_crypt!(&key, 256);

  let [encrypted_email, _] = encrypt_user_data(&cipher, email, "");
  let mut conn: SqliteConnection = establish_connection();

  diesel::update(users::dsl::users)
    .filter(users::dsl::email.eq(encrypted_email))
    .set(users::dsl::verified.eq(true))
    .execute(&mut conn)
    .expect("There was an error assigning a code for the user");
}

#[tauri::command(rename_all = "snake_case")]
pub fn is_user_verified(email: &str) -> bool {
  use crate::diesel::ExpressionMethods;

  let key = get_encryption_key();
  let cipher = new_magic_crypt!(&key, 256);

  let [encrypted_email, _] = encrypt_user_data(&cipher, email, "");
  let mut conn: SqliteConnection = establish_connection();

  // If a person with the given email AND has a verified column of true exists, the person is verified
  let person: Result<User, diesel::result::Error> = users::dsl::users
    .filter(users::dsl::email.eq(encrypted_email))
    .filter(users::dsl::verified.eq(true))
    .get_result::<User>(&mut conn);

  match person {
    Ok(_person) => {
      true
    },
    Err(_) => false
  }
}

fn update_users_verification_code(cipher: &MagicCrypt256, generated_code: &str, email: &str) {
  let [encrypted_email, _] = encrypt_user_data(cipher, email, "");

  let mut conn: SqliteConnection = establish_connection();
  diesel::update(users::table)
    .filter(users::email.eq(encrypted_email))
    .set(users::code.eq(generated_code))
    .execute(&mut conn)
    .expect("There was an error assigning a code for the user");
}

use rand::{distributions::Alphanumeric, Rng};

use super::user_models::encrypt_user_data;
fn generate_verification_code() -> String {
  let code: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(6)
    .map(char::from)
    .collect();
  code
}