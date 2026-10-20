use app::automaton::Automaton;
use app::models::State;
use std::collections::HashMap;

#[tauri::command]
pub fn test_string_dfa(state_positions: HashMap<String, State>, start_state_coordinates: String, string_to_check: String) -> (bool, Vec<State>) {
  test_string(&state_positions, &start_state_coordinates, &string_to_check)
}

#[tauri::command]
pub fn test_string_nfa(
  state_positions: HashMap<String, State>,
  start_state_coordinates: String,
  string_to_check: String
  ) -> (bool, Vec<State>) {
  test_string(&state_positions, &start_state_coordinates, &string_to_check)
}

// DFAs and NFAs are checked the same way. A DFA with two connections for the same character from one state
// is treated as the NFA it is, rather than following one of the two arbitrarily.
// Returns whether the string was accepted and the states visited along the way, for showing the traversal
fn test_string(state_positions: &HashMap<String, State>, start_state_coordinates: &str, string_to_check: &str) -> (bool, Vec<State>) {

  let (automaton, state_keys) = Automaton::from_state_positions(state_positions, Some(start_state_coordinates));
  let (is_string_accepted, path) = automaton.trace(string_to_check);

  let states_visited = path
    .into_iter()
    .map(|state| state_positions[&state_keys[state]].to_owned())
    .collect();

  (is_string_accepted, states_visited)

}

#[cfg(test)]
mod tests {
  use super::{test_string_dfa, test_string_nfa};
  use app::models::{Coordinate, State};
  use std::collections::HashMap;

  fn state(x: i32, is_start: bool, is_final: bool, connections: &[(&str, i32)]) -> (String, State) {
    let mut state = State::new(Coordinate { x, y: 100 }, is_start, is_final);
    for (connection_character, target_x) in connections {
      state.add_connection(connection_character, format!("{target_x},100"));
    }
    (format!("{x},100"), state)
  }

  fn x_positions(states: &[State]) -> Vec<i32> {
    states.iter().map(|state| state.get_position().x).collect()
  }

  #[test]
  fn test_epsilon_cycle_does_not_overflow() {
    // 100 -ϵ-> 200 -ϵ-> 300 -ϵ-> 100, and 300 -a-> 400, which is final
    let state_positions = HashMap::from([
      state(100, true, false, &[("ϵ", 200)]),
      state(200, false, false, &[("ϵ", 300)]),
      state(300, false, false, &[("ϵ", 100), ("a", 400)]),
      state(400, false, true, &[])
    ]);

    let (accepted, path) = test_string_nfa(state_positions.clone(), "100,100".to_owned(), "a".to_owned());
    assert!(accepted);
    assert_eq!(x_positions(&path), vec![100, 200, 300, 400]);

    let (accepted, _) = test_string_nfa(state_positions.clone(), "100,100".to_owned(), "".to_owned());
    assert!(!accepted);
    let (accepted, _) = test_string_nfa(state_positions, "100,100".to_owned(), "aa".to_owned());
    assert!(!accepted);
  }

  #[test]
  fn test_non_ascii_characters() {
    let state_positions = HashMap::from([
      state(100, true, false, &[("é", 200)]),
      state(200, false, true, &[("ü", 200)])
    ]);

    assert!(test_string_nfa(state_positions.clone(), "100,100".to_owned(), "éüü".to_owned()).0);
    assert!(!test_string_nfa(state_positions.clone(), "100,100".to_owned(), "ü".to_owned()).0);
    assert!(test_string_dfa(state_positions, "100,100".to_owned(), "é".to_owned()).0);
  }

  #[test]
  fn test_dfa_with_two_connections_on_one_character_checks_both() {
    let state_positions = HashMap::from([
      state(100, true, false, &[("a", 200), ("a", 300)]),
      state(200, false, false, &[]),
      state(300, false, true, &[])
    ]);

    for _ in 0..20 {
      let (accepted, path) = test_string_dfa(state_positions.clone(), "100,100".to_owned(), "a".to_owned());
      assert!(accepted);
      assert_eq!(x_positions(&path), vec![100, 300]);
    }
  }

  #[test]
  fn test_rejected_string_shows_how_far_it_got() {
    let state_positions = HashMap::from([
      state(100, true, false, &[("a", 200)]),
      state(200, false, false, &[("b", 300)]),
      state(300, false, true, &[])
    ]);

    let (accepted, path) = test_string_dfa(state_positions.clone(), "100,100".to_owned(), "aa".to_owned());
    assert!(!accepted);
    assert_eq!(x_positions(&path), vec![100, 200]);

    let (accepted, path) = test_string_dfa(state_positions.clone(), "100,100".to_owned(), "a".to_owned());
    assert!(!accepted);
    assert_eq!(x_positions(&path), vec![100, 200]);

    let (accepted, path) = test_string_dfa(state_positions, "999,999".to_owned(), "a".to_owned());
    assert!(!accepted);
    assert!(path.is_empty());
  }

  #[test]
  fn test_connections_to_missing_states_and_empty_characters_are_ignored() {
    let state_positions = HashMap::from([
      state(100, true, false, &[("a", 900), ("", 200), ("ab", 200), ("b", 200)]),
      state(200, false, true, &[])
    ]);

    assert!(test_string_dfa(state_positions.clone(), "100,100".to_owned(), "b".to_owned()).0);
    assert!(!test_string_dfa(state_positions.clone(), "100,100".to_owned(), "a".to_owned()).0);
    assert!(!test_string_dfa(state_positions, "100,100".to_owned(), "".to_owned()).0);
  }
}
