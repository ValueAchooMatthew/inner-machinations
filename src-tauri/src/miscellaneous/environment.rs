use std::{env, fs, io::Write};
use diesel::sqlite::Sqlite;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rand::{distributions::Alphanumeric, Rng};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub fn get_encryption_key() -> String {

  let mut env_file = dotenv::dotenv();

  if env_file.is_err() {
    create_new_env_file();
    env_file = dotenv::dotenv();
  }
  
  env_file.ok();

  env::var("ENCRYPTION_KEY")
    .expect("There was an error retrieving the encryption key")

}

fn create_new_env_file() {
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
}

pub fn set_working_directory() {

  let path_to_executable = env::current_exe()
    .expect("There was an error retrieving the path to the executable");

  let exectable_directory = path_to_executable.parent();

  env::set_current_dir(exectable_directory.unwrap())
    .expect("There was an error setting the working directory");

}

pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) {

  // This will run the necessary migrations.
  connection
    .run_pending_migrations(MIGRATIONS)
    .expect("There was an error running the migrations");
}