use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, Once};

use crate::models::{BezierCurve, Connection, Coordinate, State, TypeOfAutomata};
use crate::schema::{saved_connections, saved_states, saved_workspaces};
use crate::{establish_connection, set_database_path};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::Value;

use crate::saving_automata_funcs::{
  create_workspace, delete_workspace, does_workspace_name_exist, get_saved_workspaces, retrieve_workspace_data,
  save_workspace, update_automata_type, update_default_connection_character, update_showing_string_traversal,
  update_strict_checking, update_workspace_alphabet, update_workspace_name
};

static SET_UP_DATABASE: Once = Once::new();
static DATABASE_IN_USE: Mutex<()> = Mutex::new(());

// The commands open the one database the app is set up with, so the tests share a temporary database
// and take turns with it. Each test uses its own workspace names
fn use_database() -> MutexGuard<'static, ()> {
  SET_UP_DATABASE.call_once(|| {
    let database_path = std::env::temp_dir().join(format!("inner-machinations-test-{}.sqlite3", std::process::id()));
    std::fs::remove_file(&database_path).ok();
    set_database_path(database_path);
    crate::run_migrations(&mut establish_connection());
  });
  DATABASE_IN_USE.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn fresh_workspace(workspace_name: &str) {
  if does_workspace_name_exist(workspace_name) {
    delete_workspace(workspace_name);
  }
  create_workspace(workspace_name);
}

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
fn workspace_json(workspace_name: &str) -> Value {
  serde_json::to_value(retrieve_workspace_data(workspace_name)).unwrap()
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

fn workspace_id(workspace_name: &str) -> i32 {
  saved_workspaces::table
    .filter(saved_workspaces::workspace_name.eq(workspace_name))
    .select(saved_workspaces::id)
    .get_result(&mut establish_connection())
    .unwrap()
}

fn saved_row_counts(workspace_id: i32) -> (i64, i64) {
  let mut conn = establish_connection();
  let states = saved_states::table
    .filter(saved_states::workspace_id.eq(workspace_id))
    .count()
    .get_result(&mut conn)
    .unwrap();
  let connections = saved_connections::table
    .filter(saved_connections::workspace_id.eq(workspace_id))
    .count()
    .get_result(&mut conn)
    .unwrap();
  (states, connections)
}

#[test]
fn test_new_workspace() {
  let _database = use_database();
  fresh_workspace("new workspace");

  assert!(does_workspace_name_exist("new workspace"));
  assert!(get_saved_workspaces().contains(&"new workspace".to_owned()));

  let json = workspace_json("new workspace");
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
  let _database = use_database();
  fresh_workspace("round trip");
  let (states, connections) = example_automaton();

  save_workspace("round trip", states, connections);

  let json = workspace_json("round trip");
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
  let _database = use_database();
  fresh_workspace("saved twice");
  let (states, connections) = example_automaton();
  save_workspace("saved twice", states, connections);

  let states = HashMap::from([state(300, 300, false, true, &[])]);
  save_workspace("saved twice", states, vec![]);

  let json = workspace_json("saved twice");
  assert_eq!(describe_states(&json), (vec!["300,300 final".to_owned()], vec![]));
  assert_eq!(describe_connections(&json), Vec::<String>::new());
  assert_eq!(saved_row_counts(workspace_id("saved twice")), (1, 0));
}

#[test]
fn test_renaming_keeps_the_automaton() {
  let _database = use_database();
  fresh_workspace("old name");
  if does_workspace_name_exist("new name") {
    delete_workspace("new name");
  }
  let (states, connections) = example_automaton();
  save_workspace("old name", states, connections);

  update_workspace_name("old name", "new name");

  assert!(!does_workspace_name_exist("old name"));
  assert!(does_workspace_name_exist("new name"));
  assert_eq!(describe_states(&workspace_json("new name")).0.len(), 2);
}

#[test]
fn test_deleting_removes_states_and_connections() {
  let _database = use_database();
  fresh_workspace("to delete");
  let (states, connections) = example_automaton();
  save_workspace("to delete", states, connections);
  let id = workspace_id("to delete");
  assert_eq!(saved_row_counts(id), (2, 3));

  delete_workspace("to delete");

  assert!(!does_workspace_name_exist("to delete"));
  assert!(!get_saved_workspaces().contains(&"to delete".to_owned()));
  assert_eq!(saved_row_counts(id), (0, 0));
}

#[test]
fn test_options_are_saved() {
  let _database = use_database();
  fresh_workspace("options");

  // Characters must be single and distinct
  let sanitized = update_workspace_alphabet("options", vec!["x", "y", "x", "", "zz", "0"]);
  assert_eq!(sanitized, vec!["x", "y", "0"]);
  update_automata_type("options", TypeOfAutomata::NFA);
  update_strict_checking("options", true);
  update_showing_string_traversal("options", true);
  update_default_connection_character("options", "y".to_owned());

  let json = workspace_json("options");
  assert_eq!(json["alphabet"], serde_json::json!(["x", "y", "0"]));
  assert_eq!(json["type_of_automata"], "NFA");
  assert_eq!(json["should_strict_check"], true);
  assert_eq!(json["should_show_string_traversal"], true);
  assert_eq!(json["default_connection_character"], "y");
}

#[test]
fn test_saving_doesnt_touch_other_workspaces() {
  let _database = use_database();
  fresh_workspace("first of two");
  fresh_workspace("second of two");
  let (states, connections) = example_automaton();
  save_workspace("first of two", states, connections);

  assert_eq!(describe_states(&workspace_json("second of two")), (vec![], vec![]));
  assert_eq!(saved_row_counts(workspace_id("second of two")), (0, 0));
}
