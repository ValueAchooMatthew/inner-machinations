-- Brings back the accounts tables. The accounts themselves are gone, so every workspace is given to
-- a placeholder user. Rebuilt the same way as in up.sql, since foreign keys are enforced.
CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT FALSE,
  code TEXT
);

INSERT INTO users (id, email, password, verified) VALUES (1, '', '', FALSE);

CREATE TABLE new_saved_workspaces (
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

CREATE TABLE new_saved_states (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_id INTEGER NOT NULL,
  position TEXT NOT NULL,
  is_start BOOLEAN NOT NULL DEFAULT FALSE,
  is_final BOOLEAN NOT NULL DEFAULT FALSE,
  FOREIGN KEY (workspace_id) REFERENCES new_saved_workspaces(id)
);

CREATE TABLE new_saved_connections (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_id INTEGER NOT NULL,
  start_point TEXT NOT NULL,
  control_point_one TEXT NOT NULL,
  control_point_two TEXT NOT NULL,
  end_point TEXT NOT NULL,
  connection_character TEXT NOT NULL,
  FOREIGN KEY (workspace_id) REFERENCES new_saved_workspaces(id)
);

INSERT INTO new_saved_workspaces (
  id, user_id, workspace_name, type_of_automata, date_of_last_update, alphabet,
  should_show_string_traversal, should_strict_check, default_connection_character
)
SELECT
  id, 1, workspace_name, type_of_automata, date_of_last_update, alphabet,
  should_show_string_traversal, should_strict_check, default_connection_character
FROM saved_workspaces;

INSERT INTO new_saved_states (id, workspace_id, position, is_start, is_final)
SELECT id, workspace_id, position, is_start, is_final FROM saved_states;

INSERT INTO new_saved_connections (
  id, workspace_id, start_point, control_point_one, control_point_two, end_point, connection_character
)
SELECT id, workspace_id, start_point, control_point_one, control_point_two, end_point, connection_character
FROM saved_connections;

DROP TABLE saved_states;
DROP TABLE saved_connections;
DROP TABLE saved_workspaces;

ALTER TABLE new_saved_workspaces RENAME TO saved_workspaces;
ALTER TABLE new_saved_states RENAME TO saved_states;
ALTER TABLE new_saved_connections RENAME TO saved_connections;
