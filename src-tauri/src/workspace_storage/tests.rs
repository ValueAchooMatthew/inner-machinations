use std::collections::HashMap;

use diesel::result::{DatabaseErrorKind, Error};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use serde_json::Value;

use crate::database::in_memory_connection;
use crate::models::{BezierCurve, Connection, Coordinate, State, TypeOfAutomata};
use crate::schema::{saved_connections, saved_states, saved_workspaces};
use crate::workspace_storage::{
  create_workspace, delete_workspace, does_workspace_name_exist, get_saved_workspaces, retrieve_workspace_data,
  save_workspace, update_automata_type, update_default_connection_character, update_showing_string_traversal,
  update_strict_checking, update_workspace_alphabet, update_workspace_name
};

fn state(x: i32, y: i32, is_start: bool, is_final: bool, connections: &[(&str, &str)]) -> (String, State) {
  let mut state = State::new(Coordinate { x, y }, is_start, is_final);
  for (connection_character, target) in connections {
    state.add_connection(connection_character, *target);
  }
  (format!("{x},{y}"), state)
}

fn connection(character: &str, from: (i32, i32), to: (i32, i32)) -> Connection {
  Connection {
    curve: BezierCurve {
      start_point: Coordinate { x: from.0, y: from.1 },
      control_point_one: Coordinate { x: from.0 + 10, y: from.1 + 20 },
      control_point_two: Coordinate { x: to.0 - 30, y: to.1 - 40 },
      end_point: Coordinate { x: to.0, y: to.1 }
    },
    connection_character: character.to_owned(),
    element: String::from("Connection")
  }
}

fn example_automaton() -> (HashMap<String, State>, Vec<Connection>) {
  let states = HashMap::from([
    state(200, 200, true, false, &[("a", "500,200"), ("b", "200,200")]),
    state(500, 200, false, true, &[("ϵ", "200,200")])
  ]);
  let connections = vec![
    connection("a", (200, 200), (500, 200)),
    connection("b", (200, 200), (200, 200)),
    connection("ϵ", (500, 200), (200, 200))
  ];
  (states, connections)
}

// The workspace as the frontend receives it, as JSON
fn workspace_json(connection: &mut SqliteConnection, workspace_name: &str) -> Value {
  serde_json::to_value(retrieve_workspace_data(connection, workspace_name).unwrap()).unwrap()
}

// States as "x,y start final" and transitions as "x,y -a-> x,y", sorted
fn describe_states(json: &Value) -> (Vec<String>, Vec<String>) {
  let mut states = vec![];
  let mut transitions = vec![];
  for (key, state) in json["state_positions"].as_object().unwrap() {
    let mut description = key.clone();
    if state["is_start"] == true {
      description += " start";
    }
    if state["is_final"] == true {
      description += " final";
    }
    states.push(description);
    for (character, targets) in state["states_connected_to"].as_object().unwrap() {
      for target in targets.as_array().unwrap() {
        transitions.push(format!("{key} -{character}-> {}", target.as_str().unwrap()));
      }
    }
  }
  states.sort();
  transitions.sort();
  (states, transitions)
}

fn describe_connections(json: &Value) -> Vec<String> {
  let point = |point: &Value| format!("{},{}", point["x"], point["y"]);
  let mut connections: Vec<String> = json["list_of_connections"]
    .as_array()
    .unwrap()
    .iter()
    .map(|connection| {
      let curve = &connection["curve"];
      format!(
        "{} -{}-> {} via {} {}",
        point(&curve["start_point"]),
        connection["connection_character"].as_str().unwrap(),
        point(&curve["end_point"]),
        point(&curve["control_point_one"]),
        point(&curve["control_point_two"])
      )
    })
    .collect();
  connections.sort();
  connections
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

  let json = workspace_json(connection, "new workspace");
  assert_eq!(json["type_of_automata"], "DFA");
  assert_eq!(json["alphabet"], serde_json::json!(["a", "b"]));
  assert_eq!(json["should_strict_check"], false);
  assert_eq!(json["should_show_string_traversal"], false);
  assert_eq!(json["default_connection_character"], "a");
  assert_eq!(json["start_state_position"], Value::Null);
  assert_eq!(describe_states(&json), (vec![], vec![]));
  assert_eq!(describe_connections(&json), Vec::<String>::new());
}

#[test]
fn test_saved_automaton_comes_back_the_same() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "round trip").unwrap();
  let (states, connections) = example_automaton();

  save_workspace(connection, "round trip", &states, &connections).unwrap();

  let json = workspace_json(connection, "round trip");
  assert_eq!(describe_states(&json), (
    vec!["200,200 start".to_owned(), "500,200 final".to_owned()],
    vec!["200,200 -a-> 500,200".to_owned(), "200,200 -b-> 200,200".to_owned(), "500,200 -ϵ-> 200,200".to_owned()]
  ));
  assert_eq!(describe_connections(&json), vec![
    "200,200 -a-> 500,200 via 210,220 470,160",
    "200,200 -b-> 200,200 via 210,220 170,160",
    "500,200 -ϵ-> 200,200 via 510,220 170,160"
  ]);
  assert_eq!(json["start_state_position"], "200,200");
  assert_eq!(json["list_of_states"].as_array().unwrap().len(), 2);
}

#[test]
fn test_saving_again_replaces_the_automaton() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "saved twice").unwrap();
  let (states, connections) = example_automaton();
  save_workspace(connection, "saved twice", &states, &connections).unwrap();

  let states = HashMap::from([state(300, 300, false, true, &[])]);
  save_workspace(connection, "saved twice", &states, &[]).unwrap();

  let json = workspace_json(connection, "saved twice");
  assert_eq!(describe_states(&json), (vec!["300,300 final".to_owned()], vec![]));
  assert_eq!(describe_connections(&json), Vec::<String>::new());
  let id = workspace_id(connection, "saved twice");
  assert_eq!(saved_row_counts(connection, id), (1, 0));
}

#[test]
fn test_renaming_keeps_the_automaton() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "old name").unwrap();
  let (states, connections) = example_automaton();
  save_workspace(connection, "old name", &states, &connections).unwrap();

  update_workspace_name(connection, "old name", "new name").unwrap();

  assert!(!does_workspace_name_exist(connection, "old name").unwrap());
  assert!(does_workspace_name_exist(connection, "new name").unwrap());
  assert_eq!(describe_states(&workspace_json(connection, "new name")).0.len(), 2);
}

#[test]
fn test_deleting_removes_states_and_connections() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "to delete").unwrap();
  let (states, connections) = example_automaton();
  save_workspace(connection, "to delete", &states, &connections).unwrap();
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

  let json = workspace_json(connection, "options");
  assert_eq!(json["alphabet"], serde_json::json!(["x", "y", "0"]));
  assert_eq!(json["type_of_automata"], "NFA");
  assert_eq!(json["should_strict_check"], true);
  assert_eq!(json["should_show_string_traversal"], true);
  assert_eq!(json["default_connection_character"], "y");
}

#[test]
fn test_saving_doesnt_touch_other_workspaces() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "first of two").unwrap();
  create_workspace(connection, "second of two").unwrap();
  let (states, connections) = example_automaton();
  save_workspace(connection, "first of two", &states, &connections).unwrap();

  assert_eq!(describe_states(&workspace_json(connection, "second of two")), (vec![], vec![]));
  let id = workspace_id(connection, "second of two");
  assert_eq!(saved_row_counts(connection, id), (0, 0));
}

#[test]
fn test_missing_workspaces_are_errors() {
  let connection = &mut in_memory_connection();
  let (states, connections) = example_automaton();

  assert_eq!(retrieve_workspace_data(connection, "missing").unwrap_err(), Error::NotFound);
  assert_eq!(save_workspace(connection, "missing", &states, &connections).unwrap_err(), Error::NotFound);
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

#[test]
fn test_damaged_positions_are_errors() {
  let connection = &mut in_memory_connection();
  create_workspace(connection, "damaged").unwrap();
  let id = workspace_id(connection, "damaged");
  diesel::insert_into(saved_states::table)
    .values((saved_states::workspace_id.eq(id), saved_states::position.eq("nonsense")))
    .execute(connection)
    .unwrap();

  assert!(matches!(retrieve_workspace_data(connection, "damaged").unwrap_err(), Error::DeserializationError(_)));
}
