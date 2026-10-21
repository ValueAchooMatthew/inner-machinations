// Regular expressions saved from the regular expression builder. Like workspace_storage, each function is
// given the connection to use
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::result::Error::NotFound;

use crate::models::{RegexData, SavedRegex};
use crate::schema::saved_regexes;

#[cfg(test)]
mod tests;

pub fn create_regex(connection: &mut SqliteConnection, regex_name: &str) -> QueryResult<()> {

  diesel::insert_into(saved_regexes::table)
    .values(saved_regexes::regex_name.eq(regex_name))
    .execute(connection)?;

  Ok(())

}

pub fn does_regex_name_exist(connection: &mut SqliteConnection, regex_name: &str) -> QueryResult<bool> {
  Ok(regex_named(regex_name).first::<SavedRegex>(connection).optional()?.is_some())
}

// Every saved regular expression, oldest first
pub fn get_saved_regexes(connection: &mut SqliteConnection) -> QueryResult<Vec<RegexData>> {
  let saved_regexes: Vec<SavedRegex> = saved_regexes::table
    .order(saved_regexes::id)
    .load(connection)?;
  Ok(saved_regexes.into_iter().map(regex_data).collect())
}

pub fn retrieve_regex(connection: &mut SqliteConnection, regex_name: &str) -> QueryResult<RegexData> {
  Ok(regex_data(regex_named(regex_name).get_result(connection)?))
}

pub fn save_regex(connection: &mut SqliteConnection, regex_name: &str, regex: &str) -> QueryResult<()> {

  let updated_rows = diesel::update(regex_named(regex_name))
    .set((
      saved_regexes::regex.eq(regex),
      saved_regexes::date_of_last_update.eq::<NaiveDateTime>(chrono::offset::Local::now().naive_local())
    ))
    .execute(connection)?;

  expect_one_row(updated_rows)

}

pub fn update_regex_name(
  connection: &mut SqliteConnection,
  original_regex_name: &str,
  new_regex_name: &str
) -> QueryResult<()> {

  let updated_rows = diesel::update(regex_named(original_regex_name))
    .set(saved_regexes::regex_name.eq(new_regex_name))
    .execute(connection)?;

  expect_one_row(updated_rows)

}

pub fn delete_regex(connection: &mut SqliteConnection, regex_name: &str) -> QueryResult<()> {
  let deleted_rows = diesel::delete(regex_named(regex_name)).execute(connection)?;
  expect_one_row(deleted_rows)
}

fn regex_named(regex_name: &str) -> diesel::dsl::Filter<saved_regexes::table, diesel::dsl::Eq<saved_regexes::regex_name, &str>> {
  saved_regexes::table.filter(saved_regexes::regex_name.eq(regex_name))
}

fn regex_data(saved_regex: SavedRegex) -> RegexData {
  RegexData {
    regex_name: saved_regex.regex_name,
    regex: saved_regex.regex,
    date_of_last_update: saved_regex.date_of_last_update.format("%Y-%m-%d %H:%M:%S").to_string()
  }
}

// A change that touched no rows was given the name of a regular expression that doesn't exist
fn expect_one_row(changed_rows: usize) -> QueryResult<()> {
  if changed_rows == 0 {
    return Err(NotFound);
  }
  Ok(())
}
