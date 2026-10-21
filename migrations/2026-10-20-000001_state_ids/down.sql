-- Back to states identified by their position as "x,y", and connections that name their states'
-- positions. If two states of a workspace share a position, only the first is kept.
CREATE TABLE new_saved_states (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_id INTEGER NOT NULL,
  position TEXT NOT NULL,
  is_start BOOLEAN NOT NULL DEFAULT FALSE,
  is_final BOOLEAN NOT NULL DEFAULT FALSE,
  FOREIGN KEY (workspace_id) REFERENCES saved_workspaces(id)
);

CREATE TABLE new_saved_connections (
  id INTEGER PRIMARY KEY NOT NULL UNIQUE,
  workspace_id INTEGER NOT NULL,
  start_point TEXT NOT NULL,
  control_point_one TEXT NOT NULL,
  control_point_two TEXT NOT NULL,
  end_point TEXT NOT NULL,
  connection_character TEXT NOT NULL,
  FOREIGN KEY (workspace_id) REFERENCES saved_workspaces(id)
);

INSERT INTO new_saved_states (id, workspace_id, position, is_start, is_final)
SELECT id, workspace_id, x || ',' || y, is_start, is_final
FROM saved_states
WHERE id IN (SELECT min(id) FROM saved_states GROUP BY workspace_id, x, y);

INSERT INTO new_saved_connections (
  id, workspace_id, start_point, control_point_one, control_point_two, end_point, connection_character
)
SELECT
  connection.id,
  connection.workspace_id,
  from_state.x || ',' || from_state.y,
  connection.control_point_one_x || ',' || connection.control_point_one_y,
  connection.control_point_two_x || ',' || connection.control_point_two_y,
  to_state.x || ',' || to_state.y,
  connection.connection_character
FROM saved_connections AS connection
JOIN saved_states AS from_state
  ON from_state.workspace_id = connection.workspace_id AND from_state.state_id = connection.from_state
JOIN saved_states AS to_state
  ON to_state.workspace_id = connection.workspace_id AND to_state.state_id = connection.to_state;

DROP TABLE saved_connections;
DROP TABLE saved_states;

ALTER TABLE new_saved_states RENAME TO saved_states;
ALTER TABLE new_saved_connections RENAME TO saved_connections;
