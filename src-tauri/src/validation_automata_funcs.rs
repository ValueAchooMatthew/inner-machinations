use app::automaton::parse_input_alphabet;
use app::models::State;
use std::collections::HashMap;

#[tauri::command]
pub fn verify_valid_dfa(
  state_positions: HashMap<String, State>,
  input_alphabet: Vec<String>,
  start_state_position: Option<String>
) -> bool {

  let has_start_state = start_state_position
    .is_some_and(|start_state_position| state_positions.contains_key(&start_state_position));

  if !has_start_state {
    return false;
  }

  let input_alphabet: Vec<String> = parse_input_alphabet(&input_alphabet)
    .into_iter()
    .map(String::from)
    .collect();

  for state in state_positions.values() {

    // Characters whose connections were all moved to another character are left behind with none
    let connected_characters: Vec<&String> = state
      .get_all_connections()
      .iter()
      .filter(|(_, connected_state_keys)| !connected_state_keys.is_empty())
      .map(|(character, _)| character)
      .collect();

    // If the number of characters with connections doesn't equal the size of the alphabet, we know automatically
    // that the dfa isn't valid
    if connected_characters.len() != input_alphabet.len() {
      return false;
    };

    for character in connected_characters {
      // Strictly speaking, in a DFA, every single state must have exactly
      // one connection for each character in the input alphabet, to a state that exists.
      // Epsilon is never in the alphabet, so epsilon connections are caught here too
      if !input_alphabet.contains(character) {
        return false;
      };

      match state.get_connections_by_character(character) {
        Some(connected_state_keys) => {
          if connected_state_keys.len() != 1 {
            return false
          };
          if !connected_state_keys.iter().all(|key| state_positions.contains_key(key)) {
            return false
          };
        },
        None => return false
      };
    };

  };
  true
}

#[cfg(test)]
mod tests {
  use super::verify_valid_dfa;
  use app::models::{Coordinate, State};
  use std::collections::HashMap;

  fn state(x: i32, connections: &[(&str, i32)]) -> (String, State) {
    let mut state = State::new(Coordinate { x, y: 100 }, false, false);
    for (connection_character, target_x) in connections {
      state.add_connection(connection_character, format!("{target_x},100"));
    }
    (format!("{x},100"), state)
  }

  fn alphabet() -> Vec<String> {
    vec!["a".to_owned(), "b".to_owned(), "".to_owned(), "a".to_owned()]
  }

  fn start() -> Option<String> {
    Some("100,100".to_owned())
  }

  #[test]
  fn test_complete_dfa_is_valid() {
    let state_positions = HashMap::from([
      state(100, &[("a", 200), ("b", 100)]),
      state(200, &[("a", 200), ("b", 100)])
    ]);
    assert!(verify_valid_dfa(state_positions, alphabet(), start()));
  }

  #[test]
  fn test_invalid_dfas() {
    let missing_character = HashMap::from([state(100, &[("a", 100)])]);
    assert!(!verify_valid_dfa(missing_character, alphabet(), start()));

    let two_targets = HashMap::from([
      state(100, &[("a", 100), ("a", 200), ("b", 100)]),
      state(200, &[("a", 200), ("b", 200)])
    ]);
    assert!(!verify_valid_dfa(two_targets, alphabet(), start()));

    let missing_target = HashMap::from([state(100, &[("a", 100), ("b", 900)])]);
    assert!(!verify_valid_dfa(missing_target, alphabet(), start()));

    let epsilon = HashMap::from([state(100, &[("a", 100), ("b", 100), ("ϵ", 100)])]);
    assert!(!verify_valid_dfa(epsilon, alphabet(), start()));

    let complete = HashMap::from([state(100, &[("a", 100), ("b", 100)])]);
    assert!(!verify_valid_dfa(complete.clone(), alphabet(), None));
    assert!(!verify_valid_dfa(complete, alphabet(), Some("900,900".to_owned())));
  }

  #[test]
  fn test_emptied_characters_are_ignored() {
    let mut state_positions = HashMap::from([state(100, &[("a", 100), ("b", 100), ("c", 100)])]);
    state_positions
      .get_mut("100,100")
      .unwrap()
      .remove_connection_by_character("c", "100,100")
      .unwrap();
    assert!(verify_valid_dfa(state_positions, alphabet(), start()));
  }
}
