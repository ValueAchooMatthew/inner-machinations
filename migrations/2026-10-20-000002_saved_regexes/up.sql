-- Regular expressions saved from the regular expression builder, each under a name of its own
CREATE TABLE saved_regexes (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  regex_name TEXT NOT NULL UNIQUE,
  regex TEXT NOT NULL DEFAULT '',
  date_of_last_update TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
