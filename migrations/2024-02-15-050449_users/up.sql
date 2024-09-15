-- Your SQL goes here

CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT FALSE,
  number_of_untitled_regular_automata_workspaces INTEGER NOT NULL DEFAULT 0,
  number_of_untitled_regex_workspaces INTEGER NOT NULL DEFAULT 0,
  code TEXT
);

CREATE TABLE saved_regular_automata_workspaces (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  user_id INTEGER NOT NULL,
  workspace_name TEXT NOT NULL,
  type_of_automata TEXT CHECK(type_of_automata IN ('DFA', 'NFA')) NOT NULL,
  date_of_last_update TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  alphabet TEXT NOT NULL DEFAULT 'a,b',
  should_show_string_traversal BOOLEAN NOT NULL DEFAULT FALSE,
  should_strict_check BOOLEAN NOT NULL DEFAULT FALSE,
  default_connection_character TEXT NOT NULL DEFAULT 'a',
  FOREIGN KEY (user_id) REFERENCES users(id),
  UNIQUE (user_id, workspace_name)
);

CREATE TABLE saved_states (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_id INTEGER NOT NULL, 
  position TEXT NOT NULL,
  is_start BOOLEAN NOT NULL DEFAULT FALSE,
  is_final BOOLEAN NOT NULL DEFAULT FALSE,
  FOREIGN KEY (workspace_id) REFERENCES saved_regular_automata_workspaces(id)
);

CREATE TABLE saved_regular_automata_connections (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_id INTEGER NOT NULL, 
  start_point TEXT NOT NULL, 
  control_point_one TEXT NOT NULL,
  control_point_two TEXT NOT NULL,
  end_point TEXT NOT NULL,
  connection_character TEXT NOT NULL,
  FOREIGN KEY (workspace_id) REFERENCES saved_regular_automata_workspaces(id)
);

CREATE TABLE saved_regex_workspaces (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  user_id INTEGER NOT NULL,
  regex_name TEXT NOT NULL,
  regex TEXT NOT NULL DEFAULT '',
  date_of_last_update TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id),
  UNIQUE (user_id, regex_name)
);