-- States get an id, unique within their workspace, that stays the same when they move, and their
-- position is stored as two numbers instead of "x,y". Connections refer to the states they join by
-- id, so a connection can only join states that exist. Control points are stored as numbers too.
--
-- Existing states are numbered in the order they were saved, and existing connections find their
-- states by position. A position that isn't "x,y", or a connection that doesn't start and end on a
-- state, can't be carried over and is dropped.
--
-- As in the migration removing accounts, the tables are rebuilt rather than altered, since foreign
-- keys are enforced.
CREATE TABLE new_saved_states (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_id INTEGER NOT NULL,
  state_id INTEGER NOT NULL,
  x INTEGER NOT NULL,
  y INTEGER NOT NULL,
  is_start BOOLEAN NOT NULL DEFAULT FALSE,
  is_final BOOLEAN NOT NULL DEFAULT FALSE,
  FOREIGN KEY (workspace_id) REFERENCES saved_workspaces(id),
  UNIQUE (workspace_id, state_id)
);

CREATE TABLE new_saved_connections (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_id INTEGER NOT NULL,
  connection_id INTEGER NOT NULL,
  from_state INTEGER NOT NULL,
  to_state INTEGER NOT NULL,
  connection_character TEXT NOT NULL,
  control_point_one_x INTEGER NOT NULL,
  control_point_one_y INTEGER NOT NULL,
  control_point_two_x INTEGER NOT NULL,
  control_point_two_y INTEGER NOT NULL,
  FOREIGN KEY (workspace_id) REFERENCES saved_workspaces(id),
  FOREIGN KEY (workspace_id, from_state) REFERENCES new_saved_states(workspace_id, state_id),
  FOREIGN KEY (workspace_id, to_state) REFERENCES new_saved_states(workspace_id, state_id),
  UNIQUE (workspace_id, connection_id)
);

INSERT INTO new_saved_states (id, workspace_id, state_id, x, y, is_start, is_final)
SELECT
  id,
  workspace_id,
  ROW_NUMBER() OVER (PARTITION BY workspace_id ORDER BY id),
  CAST(substr(position, 1, instr(position, ',') - 1) AS INTEGER),
  CAST(substr(position, instr(position, ',') + 1) AS INTEGER),
  is_start,
  is_final
FROM saved_states
WHERE position GLOB '*[0-9],*[0-9]' AND position NOT GLOB '*[^0-9,-]*' AND position NOT GLOB '*,*,*';

INSERT INTO new_saved_connections (
  id, workspace_id, connection_id, from_state, to_state, connection_character,
  control_point_one_x, control_point_one_y, control_point_two_x, control_point_two_y
)
SELECT
  connection.id,
  connection.workspace_id,
  ROW_NUMBER() OVER (PARTITION BY connection.workspace_id ORDER BY connection.id),
  from_state.state_id,
  to_state.state_id,
  connection.connection_character,
  CAST(substr(connection.control_point_one, 1, instr(connection.control_point_one, ',') - 1) AS INTEGER),
  CAST(substr(connection.control_point_one, instr(connection.control_point_one, ',') + 1) AS INTEGER),
  CAST(substr(connection.control_point_two, 1, instr(connection.control_point_two, ',') - 1) AS INTEGER),
  CAST(substr(connection.control_point_two, instr(connection.control_point_two, ',') + 1) AS INTEGER)
FROM saved_connections AS connection
JOIN new_saved_states AS from_state
  ON from_state.workspace_id = connection.workspace_id
  AND from_state.x || ',' || from_state.y = connection.start_point
JOIN new_saved_states AS to_state
  ON to_state.workspace_id = connection.workspace_id
  AND to_state.x || ',' || to_state.y = connection.end_point
WHERE connection.control_point_one GLOB '*[0-9],*[0-9]' AND connection.control_point_one NOT GLOB '*[^0-9,-]*' AND connection.control_point_one NOT GLOB '*,*,*'
  AND connection.control_point_two GLOB '*[0-9],*[0-9]' AND connection.control_point_two NOT GLOB '*[^0-9,-]*' AND connection.control_point_two NOT GLOB '*,*,*';

DROP TABLE saved_connections;
DROP TABLE saved_states;

ALTER TABLE new_saved_states RENAME TO saved_states;
ALTER TABLE new_saved_connections RENAME TO saved_connections;
