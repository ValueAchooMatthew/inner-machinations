use std::collections::{HashMap, HashSet};
use crate::{create_connections_from_state_positions, create_unique_state_coordinates, models::WorkspaceData};
use crate::automaton::{parse_input_alphabet, Automaton, Dfa};

use crate::models::{Connection, Coordinate, State};

use tauri::State as Managed;

use crate::database::Database;
use crate::workspace_storage;

// DFA's are typically required to have a connection for each state for each character in the input alphabet.
// This however, is very unuser friendly as it is very easy to infer that an unspecified connection results in automatic unacceptance.
// So a missing connection counts as one to a dead state, and the minimized DFA leaves out the dead state and the connections
// into it. A DFA that specifies every connection keeps its dead state, if it has one, so it still passes strict checking
#[tauri::command]
pub fn minimize_dfa(
  database: Managed<'_, Database>,
  state_positions: HashMap<String, State>,
  connections: Vec<Connection>,
  input_alphabet: Vec<String>,
  start_state_position: Option<String>,
  workspace_name: &str
) -> WorkspaceData {

  let (automaton, state_keys) = Automaton::from_state_positions(&state_positions, start_state_position.as_deref());

  // Determinizing first means a DFA drawn with two connections on one character is minimized as the NFA it is.
  // Without a start state there is no language to preserve, so the workspace is saved unchanged
  let (state_positions, connections) = match automaton.determinize(&parse_input_alphabet(&input_alphabet)) {
    Some(dfa) => {
      let minimized_state_positions = create_state_positions_from_dfa(&dfa.minimize(), &automaton, &state_keys, &state_positions);
      let minimized_connections = create_connections_from_state_positions(&minimized_state_positions, &connections);
      (minimized_state_positions, minimized_connections)
    },
    None => (state_positions, connections)
  };

  let mut connection = database.connection();
  workspace_storage::save_workspace(&mut connection, workspace_name, &state_positions, &connections)
    .expect("There was an error saving the workspace");
  let workspace_data = workspace_storage::retrieve_workspace_data(&mut connection, workspace_name)
    .expect("There was an error retrieving the workspace");

  return workspace_data;

}

// The subset construction. The resulting DFA specifies every connection, using a dead state where needed,
// so it passes strict checking
#[tauri::command]
pub fn convert_nfa_to_dfa (
  database: Managed<'_, Database>,
  state_positions: HashMap<String, State>,
  connections: Vec<Connection>,
  input_alphabet: Vec<String>,
  start_state_position: Option<String>,
  workspace_name: &str
) -> WorkspaceData {

  let (automaton, state_keys) = Automaton::from_state_positions(&state_positions, start_state_position.as_deref());

  // Without a start state there is nothing to convert, so the workspace is saved unchanged
  let (state_positions, connections) = match automaton.determinize(&parse_input_alphabet(&input_alphabet)) {
    Some(dfa) => {
      let dfa_state_positions = create_state_positions_from_dfa(&dfa.complete(), &automaton, &state_keys, &state_positions);
      let dfa_connections = create_connections_from_state_positions(&dfa_state_positions, &connections);
      (dfa_state_positions, dfa_connections)
    },
    None => (state_positions, connections)
  };

  let mut connection = database.connection();
  workspace_storage::save_workspace(&mut connection, workspace_name, &state_positions, &connections)
    .expect("There was an error saving the workspace");
  let workspace_data = workspace_storage::retrieve_workspace_data(&mut connection, workspace_name)
    .expect("There was an error retrieving the workspace");

  return workspace_data;

}

// Turns a DFA made from the automaton back into the frontend's states. Each state goes where one of the original states
// it stands for was, if that spot is still free, so a DFA that needed no changes keeps its layout.
// The start state goes where the original start state was
fn create_state_positions_from_dfa(
  dfa: &Dfa,
  automaton: &Automaton,
  original_state_keys: &[String],
  original_state_positions: &HashMap<String, State>
) -> HashMap<String, State> {

  let mut taken_positions: HashSet<String> = HashSet::new();
  let mut positions: Vec<Coordinate> = vec![];

  for state in 0..dfa.state_count() {

    let original_start_state = if state == 0 { automaton.start() } else { None };

    let free_original_position = original_start_state
      .into_iter()
      .chain(dfa.members(state).iter().copied())
      .map(|original_state| original_state_positions[&original_state_keys[original_state]].get_position())
      .find(|position| {
        let position_key: String = (*position).into();
        !taken_positions.contains(&position_key)
      });

    let position = free_original_position
      .unwrap_or_else(|| create_unique_state_coordinates(&taken_positions));

    taken_positions.insert(position.into());
    positions.push(position);
  }

  let mut state_positions = HashMap::new();

  for state in 0..dfa.state_count() {

    let mut new_state = State::new(positions[state], state == 0, dfa.is_accepting(state));

    for (character, target) in dfa.transitions_from(state) {
      new_state.add_connection(&character.to_string(), positions[target]);
    }

    state_positions.insert(new_state.get_position_as_string(), new_state);
  }

  state_positions

}

#[cfg(test)]
mod tests {
  use super::create_state_positions_from_dfa;
  use crate::automaton::Automaton;
  use crate::create_connections_from_state_positions;
  use crate::models::{Connection, Coordinate, State};
  use std::collections::{BTreeSet, HashMap, HashSet};

  fn state(x: i32, is_start: bool, is_final: bool, connections: &[(&str, i32)]) -> (String, State) {
    let mut state = State::new(Coordinate { x, y: 100 }, is_start, is_final);
    for (connection_character, target_x) in connections {
      state.add_connection(connection_character, format!("{target_x},100"));
    }
    (format!("{x},100"), state)
  }

  #[test]
  fn test_deterministic_automaton_keeps_its_layout() {
    let state_positions = HashMap::from([
      state(100, true, false, &[("a", 200)]),
      state(200, false, true, &[("b", 100)])
    ]);
    let (automaton, state_keys) = Automaton::from_state_positions(&state_positions, Some("100,100"));
    let dfa = automaton.determinize(&BTreeSet::new()).unwrap();

    let converted = create_state_positions_from_dfa(&dfa.minimize(), &automaton, &state_keys, &state_positions);

    let mut keys: Vec<&String> = converted.keys().collect();
    keys.sort();
    assert_eq!(keys, vec!["100,100", "200,100"]);
    assert!(converted["100,100"].is_start());
    assert!(converted["200,100"].is_final());
    assert_eq!(converted["100,100"].get_connections_by_character("a").unwrap().len(), 1);
    assert!(converted["100,100"].get_connections_by_character("b").is_none());
  }

  #[test]
  fn test_conversion_adds_dead_state_in_a_free_spot() {
    let state_positions = HashMap::from([
      state(100, true, false, &[("a", 200)]),
      state(200, false, true, &[])
    ]);
    let (automaton, state_keys) = Automaton::from_state_positions(&state_positions, Some("100,100"));
    let dfa = automaton.determinize(&BTreeSet::from(['a', 'b'])).unwrap().complete();

    let converted = create_state_positions_from_dfa(&dfa, &automaton, &state_keys, &state_positions);

    assert_eq!(converted.len(), 3);
    for state in converted.values() {
      assert_eq!(state.get_connections_by_character("a").unwrap().len(), 1);
      assert_eq!(state.get_connections_by_character("b").unwrap().len(), 1);
    }
    assert_eq!(converted.values().filter(|state| state.is_start()).count(), 1);
  }

  // Roughly where the frontend draws a connection's character: the middle of its curve
  fn middle_of_curve(connection: &Connection) -> (i32, i32) {
    let curve = &connection.curve;
    (
      (curve.start_point.x + 3 * curve.control_point_one.x + 3 * curve.control_point_two.x + curve.end_point.x) / 8,
      (curve.start_point.y + 3 * curve.control_point_one.y + 3 * curve.control_point_two.y + curve.end_point.y) / 8
    )
  }

  #[test]
  fn test_connections_between_the_same_states_are_drawn_apart() {
    let state_positions = HashMap::from([
      state(100, true, false, &[("a", 400), ("b", 400), ("a", 100), ("b", 100), ("c", 100)]),
      state(400, false, true, &[("a", 100), ("b", 100)])
    ]);

    let connections = create_connections_from_state_positions(&state_positions, &[]);
    assert_eq!(connections.len(), 7);

    let middles: HashSet<(i32, i32)> = connections.iter().map(middle_of_curve).collect();
    assert_eq!(middles.len(), 7);
  }

  #[test]
  fn test_connections_keep_the_curves_the_user_drew() {
    let state_positions = HashMap::from([
      state(100, true, false, &[("a", 400), ("b", 400)]),
      state(400, false, true, &[])
    ]);
    let mut previous_connection = create_connections_from_state_positions(&state_positions, &[])
      .into_iter()
      .find(|connection| connection.connection_character == "b")
      .unwrap();
    previous_connection.curve.control_point_one = Coordinate { x: 250, y: 900 };

    let connections = create_connections_from_state_positions(&state_positions, &[previous_connection]);
    let b_connection = connections.iter().find(|connection| connection.connection_character == "b").unwrap();
    assert_eq!(b_connection.curve.control_point_one, Coordinate { x: 250, y: 900 });
  }
}
