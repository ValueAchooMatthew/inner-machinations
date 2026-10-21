use std::collections::HashSet;

use crate::automaton::parse_input_alphabet;
use crate::models::AutomatonData;

// What strict checking requires of a DFA: one start state, and from every state exactly one connection for each
// character in the input alphabet, to a state that exists, and no others. Epsilon is never in the alphabet, so
// epsilon connections aren't allowed either
#[tauri::command]
pub fn verify_valid_dfa(automaton: AutomatonData, input_alphabet: Vec<String>) -> bool {

  if automaton.states.iter().filter(|state| state.is_start).count() != 1 {
    return false;
  }

  let input_alphabet: HashSet<String> = parse_input_alphabet(&input_alphabet)
    .into_iter()
    .map(String::from)
    .collect();
  let state_ids: HashSet<i32> = automaton.states.iter().map(|state| state.id).collect();

  for state in &automaton.states {

    // Drawing the same connection twice still only specifies it once
    let mut targets_by_character: Vec<(&String, i32)> = automaton
      .connections
      .iter()
      .filter(|connection| connection.from == state.id)
      .map(|connection| (&connection.character, connection.to))
      .collect();
    targets_by_character.sort();
    targets_by_character.dedup();

    for (character, target) in &targets_by_character {
      if !input_alphabet.contains(*character) || !state_ids.contains(target) {
        return false;
      }
    }

    for character in &input_alphabet {
      if targets_by_character.iter().filter(|(connection_character, _)| *connection_character == character).count() != 1 {
        return false;
      }
    }

  };
  true
}

#[cfg(test)]
mod tests {
  use super::verify_valid_dfa;
  use crate::test_support::{automaton, state, TestState};

  fn alphabet() -> Vec<String> {
    vec!["a".to_owned(), "b".to_owned(), "".to_owned(), "a".to_owned()]
  }

  // The state at 100 is the start state
  fn dfa_state(x: i32, connections: &[(&str, i32)]) -> TestState {
    state(x, x == 100, false, connections)
  }

  #[test]
  fn test_complete_dfa_is_valid() {
    let complete = automaton([
      dfa_state(100, &[("a", 200), ("b", 100)]),
      dfa_state(200, &[("a", 200), ("b", 100)])
    ]);
    assert!(verify_valid_dfa(complete, alphabet()));

    let drawn_twice = automaton([dfa_state(100, &[("a", 100), ("a", 100), ("b", 100)])]);
    assert!(verify_valid_dfa(drawn_twice, alphabet()));
  }

  #[test]
  fn test_invalid_dfas() {
    let missing_character = automaton([dfa_state(100, &[("a", 100)])]);
    assert!(!verify_valid_dfa(missing_character, alphabet()));

    let two_targets = automaton([
      dfa_state(100, &[("a", 100), ("a", 200), ("b", 100)]),
      dfa_state(200, &[("a", 200), ("b", 200)])
    ]);
    assert!(!verify_valid_dfa(two_targets, alphabet()));

    let missing_target = automaton([dfa_state(100, &[("a", 100), ("b", 900)])]);
    assert!(!verify_valid_dfa(missing_target, alphabet()));

    let epsilon = automaton([dfa_state(100, &[("a", 100), ("b", 100), ("ϵ", 100)])]);
    assert!(!verify_valid_dfa(epsilon, alphabet()));

    let no_start = automaton([state(100, false, false, &[("a", 100), ("b", 100)])]);
    assert!(!verify_valid_dfa(no_start, alphabet()));

    let two_starts = automaton([
      state(100, true, false, &[("a", 100), ("b", 100)]),
      state(200, true, false, &[("a", 200), ("b", 200)])
    ]);
    assert!(!verify_valid_dfa(two_starts, alphabet()));
  }
}
