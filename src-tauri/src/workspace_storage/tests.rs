use diesel::result::{DatabaseErrorKind, Error};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::database::in_memory_connection;
use crate::models::{AutomatonData, ConnectionData, Coordinate, StateData, TypeOfAutomata};
use crate::schema::{saved_connections, saved_states, saved_workspaces};
use crate::workspace_storage::{
  create_workspace, delete_workspace, does_workspace_name_exist, get_saved_workspaces, retrieve_workspace_data,
  save_workspace, update_automata_type, update_default_connection_character, update_showing_string_traversal,
  update_strict_checking, update_workspace_alphabet, update_workspace_name
};

fn point(x: i32, y: i32) -> Coordinate {
  Coordinate { x, y }
}

// A start state and a final state, joined both ways, with a loop on the start state. The ids aren't in order
// or contiguous, as they needn't be after states are deleted
fn example_automaton() -> AutomatonData {
  AutomatonData {
    states: vec![
      StateData { id: 7, position: point(500, 200), is_start: false, is_final: true },
      StateData { id: 2, position: point(200, 200), is_start: true, is_final: false }
    ],
    connections: vec![
      ConnectionData { id: 1, from: 2, to: 7, character: "a".to_owned(), control_points: [point(210, 220), point(470, 160)] },
      ConnectionData { id: 4, from: 2, to: 2, character: "b".to_owned(), control_points: [point(0, 400), point(0, 0)] },
      ConnectionData { id: 9, from: 7, to: 2, character: "ϵ".to_owned(), control_points: [point(510, 220), point(170, 160)] }
    ]
  }
}

// The same automaton, in the order the database returns it: by id
fn sorted(mut automaton: AutomatonData) -> AutomatonData {
  automaton.states.sort_by_key(|state| state.id);
  automaton.connections.sort_by_key(|connection| connection.id);
  automaton
}

fn workspace_id(connection: &mut SqliteConnection, workspace_name: &str) -> i32 {
  saved_workspaces::table
    .filter(saved_workspaces::workspace_name.eq(workspace_name))
    .select(saved_workspaces::id)
    .get_result(connection)
    .unwrap()
}

fn saved_row_counts(connection: &mut SqliteConnection, workspace_id: i32) -> (i64, i64) {
  let states = saved_states::table
    .filter(saved_states::workspace_id.eq(workspace_id))
    .count()
    .get_result(connection)
    .unwrap();
  let connections = saved_connections::table
    .filter(saved_connections::workspace_id.eq(workspace_id))
    .count()
    .get_result(connection)
    .unwrap();
  (states, connections)
}

#[test]
fn test_new_workspace() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "new workspace").unwrap();

  assert!(does_workspace_name_exist(connection, "new workspace").unwrap());
  assert!(get_saved_workspaces(connection).unwrap().contains(&"new workspace".to_owned()));

  let workspace = retrieve_workspace_data(connection, "new workspace").unwrap();
  assert_eq!(workspace.automaton, AutomatonData::default());
  assert!(matches!(workspace.type_of_automata, TypeOfAutomata::DFA));
  assert_eq!(workspace.alphabet, vec!["a", "b"]);
  assert!(!workspace.should_strict_check);
  assert!(!workspace.should_show_string_traversal);
  assert_eq!(workspace.default_connection_character, "a");
}

#[test]
fn test_saved_automaton_comes_back_the_same() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "round trip").unwrap();

  save_workspace(connection, "round trip", &example_automaton(), vec!["a", "b"]).unwrap();

  assert_eq!(retrieve_workspace_data(connection, "round trip").unwrap().automaton, sorted(example_automaton()));
}

#[test]
fn test_workspaces_are_sent_in_the_frontend_s_shape() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "shape").unwrap();
  save_workspace(connection, "shape", &example_automaton(), vec!["a"]).unwrap();

  let json = serde_json::to_value(retrieve_workspace_data(connection, "shape").unwrap()).unwrap();
  assert_eq!(json["automaton"]["states"][0], serde_json::json!({
    "id": 2, "position": { "x": 200, "y": 200 }, "isStart": true, "isFinal": false
  }));
  assert_eq!(json["automaton"]["connections"][0], serde_json::json!({
    "id": 1, "from": 2, "to": 7, "character": "a", "controlPoints": [{ "x": 210, "y": 220 }, { "x": 470, "y": 160 }]
  }));
  assert_eq!(json["type_of_automata"], "DFA");
  assert_eq!(json["alphabet"], serde_json::json!(["a"]));
}

#[test]
fn test_saving_again_replaces_the_automaton() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "saved twice").unwrap();
  save_workspace(connection, "saved twice", &example_automaton(), vec!["a"]).unwrap();

  let smaller = AutomatonData {
    states: vec![StateData { id: 1, position: point(300, 300), is_start: false, is_final: true }],
    connections: vec![]
  };
  save_workspace(connection, "saved twice", &smaller, vec!["a"]).unwrap();

  assert_eq!(retrieve_workspace_data(connection, "saved twice").unwrap().automaton, smaller);
  let id = workspace_id(connection, "saved twice");
  assert_eq!(saved_row_counts(connection, id), (1, 0));
}

#[test]
fn test_saving_saves_the_alphabet() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "alphabet").unwrap();

  let saved_alphabet = save_workspace(connection, "alphabet", &example_automaton(), vec!["x", "", "x", "yz", "é", ",", "ϵ"]).unwrap();

  assert_eq!(saved_alphabet, vec!["x", "é"]);
  assert_eq!(retrieve_workspace_data(connection, "alphabet").unwrap().alphabet, vec!["x", "é"]);
}

#[test]
fn test_a_connection_to_a_missing_state_saves_nothing() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "broken").unwrap();
  save_workspace(connection, "broken", &example_automaton(), vec!["a", "b"]).unwrap();

  let mut broken = example_automaton();
  broken.connections[0].to = 99;
  let error = save_workspace(connection, "broken", &broken, vec!["c"]).unwrap_err();
  assert!(matches!(error, Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _)));

  // The earlier save is still there, alphabet and all
  let workspace = retrieve_workspace_data(connection, "broken").unwrap();
  assert_eq!(workspace.automaton, sorted(example_automaton()));
  assert_eq!(workspace.alphabet, vec!["a", "b"]);
}

#[test]
fn test_repeated_ids_save_nothing() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "repeated").unwrap();

  let mut repeated_state = example_automaton();
  repeated_state.states[1].id = 7;
  repeated_state.connections.clear();
  assert!(matches!(
    save_workspace(connection, "repeated", &repeated_state, vec![]).unwrap_err(),
    Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
  ));

  let mut repeated_connection = example_automaton();
  repeated_connection.connections[1].id = 1;
  assert!(matches!(
    save_workspace(connection, "repeated", &repeated_connection, vec![]).unwrap_err(),
    Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
  ));

  assert_eq!(retrieve_workspace_data(connection, "repeated").unwrap().automaton, AutomatonData::default());
}

#[test]
fn test_renaming_keeps_the_automaton() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "old name").unwrap();
  save_workspace(connection, "old name", &example_automaton(), vec!["a"]).unwrap();

  update_workspace_name(connection, "old name", "new name").unwrap();

  assert!(!does_workspace_name_exist(connection, "old name").unwrap());
  assert_eq!(retrieve_workspace_data(connection, "new name").unwrap().automaton, sorted(example_automaton()));
}

#[test]
fn test_deleting_removes_states_and_connections() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "to delete").unwrap();
  save_workspace(connection, "to delete", &example_automaton(), vec!["a"]).unwrap();
  let id = workspace_id(connection, "to delete");
  assert_eq!(saved_row_counts(connection, id), (2, 3));

  delete_workspace(connection, "to delete").unwrap();

  assert!(!does_workspace_name_exist(connection, "to delete").unwrap());
  assert!(!get_saved_workspaces(connection).unwrap().contains(&"to delete".to_owned()));
  assert_eq!(saved_row_counts(connection, id), (0, 0));
}

#[test]
fn test_options_are_saved() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "options").unwrap();

  // Characters must be single and distinct
  let sanitized = update_workspace_alphabet(connection, "options", vec!["x", "y", "x", "", "zz", "0"]).unwrap();
  assert_eq!(sanitized, vec!["x", "y", "0"]);
  update_automata_type(connection, "options", TypeOfAutomata::NFA).unwrap();
  update_strict_checking(connection, "options", true).unwrap();
  update_showing_string_traversal(connection, "options", true).unwrap();
  update_default_connection_character(connection, "options", "y").unwrap();

  let workspace = retrieve_workspace_data(connection, "options").unwrap();
  assert_eq!(workspace.alphabet, vec!["x", "y", "0"]);
  assert!(matches!(workspace.type_of_automata, TypeOfAutomata::NFA));
  assert!(workspace.should_strict_check);
  assert!(workspace.should_show_string_traversal);
  assert_eq!(workspace.default_connection_character, "y");
}

#[test]
fn test_saving_doesnt_touch_other_workspaces() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "first of two").unwrap();
  create_workspace(connection, "second of two").unwrap();
  save_workspace(connection, "first of two", &example_automaton(), vec!["a"]).unwrap();

  // States in different workspaces may share ids
  save_workspace(connection, "second of two", &example_automaton(), vec!["a"]).unwrap();
  delete_workspace(connection, "first of two").unwrap();

  assert_eq!(retrieve_workspace_data(connection, "second of two").unwrap().automaton, sorted(example_automaton()));
}

#[test]
fn test_missing_workspaces_are_errors() {
  let connection = &mut in_memory_connection();

  assert_eq!(retrieve_workspace_data(connection, "missing").unwrap_err(), Error::NotFound);
  assert_eq!(save_workspace(connection, "missing", &example_automaton(), vec![]).unwrap_err(), Error::NotFound);
  assert_eq!(delete_workspace(connection, "missing").unwrap_err(), Error::NotFound);
  assert_eq!(update_workspace_name(connection, "missing", "other").unwrap_err(), Error::NotFound);
  assert_eq!(update_workspace_alphabet(connection, "missing", vec!["a"]).unwrap_err(), Error::NotFound);
  assert_eq!(update_automata_type(connection, "missing", TypeOfAutomata::NFA).unwrap_err(), Error::NotFound);
  assert_eq!(update_strict_checking(connection, "missing", true).unwrap_err(), Error::NotFound);
  assert_eq!(update_showing_string_traversal(connection, "missing", true).unwrap_err(), Error::NotFound);
  assert_eq!(update_default_connection_character(connection, "missing", "a").unwrap_err(), Error::NotFound);
  assert!(!does_workspace_name_exist(connection, "missing").unwrap());
}

#[test]
fn test_names_must_be_unique() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "taken").unwrap();
  create_workspace(connection, "other").unwrap();

  assert!(matches!(
    create_workspace(connection, "taken").unwrap_err(),
    Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
  ));
  assert!(matches!(
    update_workspace_name(connection, "other", "taken").unwrap_err(),
    Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
  ));
  assert_eq!(get_saved_workspaces(connection).unwrap(), vec!["taken", "other"]);
}

#[test]
fn test_workspaces_are_listed_oldest_first() {
  let connection = &mut in_memory_connection();
  for workspace_name in ["b", "c", "a"] {
    create_workspace(connection, workspace_name).unwrap();
  }
  assert_eq!(get_saved_workspaces(connection).unwrap(), vec!["b", "c", "a"]);
}
