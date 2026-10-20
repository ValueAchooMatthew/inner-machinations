-- The app no longer has accounts, so workspaces no longer belong to a user.
-- SQLite can't drop a column that is part of a constraint, so the tables are rebuilt. The app's SQLite
-- enforces foreign keys, which can't be switched off inside the transaction diesel runs this in, so the
-- states and connections are rebuilt as well: their new tables refer to the new workspaces table, and
-- renaming that table updates the references.
-- Workspace names were unique per user; a name used by several users keeps it on the oldest
-- workspace, and the others get their id appended.
CREATE TABLE new_saved_workspaces (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_name TEXT NOT NULL UNIQUE,
  type_of_automata TEXT CHECK(type_of_automata IN ('DFA', 'NFA')) NOT NULL,
  date_of_last_update TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  alphabet TEXT NOT NULL DEFAULT 'a,b',
  should_show_string_traversal BOOLEAN NOT NULL DEFAULT FALSE,
  should_strict_check BOOLEAN NOT NULL DEFAULT FALSE,
  default_connection_character TEXT NOT NULL DEFAULT 'a'
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
  id, workspace_name, type_of_automata, date_of_last_update, alphabet,
  should_show_string_traversal, should_strict_check, default_connection_character
)
SELECT
  id,
  CASE WHEN ROW_NUMBER() OVER (PARTITION BY workspace_name ORDER BY id) = 1
    THEN workspace_name
    ELSE workspace_name || ' (' || id || ')'
  END,
  type_of_automata, date_of_last_update, alphabet,
  should_show_string_traversal, should_strict_check, default_connection_character
FROM saved_workspaces;

-- Rows whose workspace no longer exists are left behind
INSERT INTO new_saved_states (id, workspace_id, position, is_start, is_final)
SELECT id, workspace_id, position, is_start, is_final
FROM saved_states
WHERE workspace_id IN (SELECT id FROM new_saved_workspaces);

INSERT INTO new_saved_connections (
  id, workspace_id, start_point, control_point_one, control_point_two, end_point, connection_character
)
SELECT id, workspace_id, start_point, control_point_one, control_point_two, end_point, connection_character
FROM saved_connections
WHERE workspace_id IN (SELECT id FROM new_saved_workspaces);

DROP TABLE saved_states;
DROP TABLE saved_connections;
DROP TABLE saved_workspaces;
DROP TABLE users;

ALTER TABLE new_saved_workspaces RENAME TO saved_workspaces;
ALTER TABLE new_saved_states RENAME TO saved_states;
ALTER TABLE new_saved_connections RENAME TO saved_connections;
