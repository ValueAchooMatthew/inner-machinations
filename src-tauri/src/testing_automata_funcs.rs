use crate::automaton::Automaton;
use crate::models::AutomatonData;

// DFAs and NFAs are checked the same way. A DFA with two connections for the same character from one state
// is treated as the NFA it is, rather than following one of the two arbitrarily.
// Returns whether the string was accepted, and the ids of the states it passed through, for showing the traversal
#[tauri::command]
pub fn test_string(automaton: AutomatonData, string_to_check: String) -> (bool, Vec<i32>) {

  let (nfa, state_ids) = Automaton::from_data(&automaton);
  let (is_string_accepted, path) = nfa.trace(&string_to_check);

  (is_string_accepted, path.into_iter().map(|state| state_ids[state]).collect())

}

#[cfg(test)]
mod tests {
  use super::test_string;
  use crate::test_support::{automaton, state};

  #[test]
  fn test_epsilon_cycle_does_not_overflow() {
    // 100 -ϵ-> 200 -ϵ-> 300 -ϵ-> 100, and 300 -a-> 400, which is final
    let automaton = automaton([
      state(100, true, false, &[("ϵ", 200)]),
      state(200, false, false, &[("ϵ", 300)]),
      state(300, false, false, &[("ϵ", 100), ("a", 400)]),
      state(400, false, true, &[])
    ]);

    assert_eq!(test_string(automaton.clone(), "a".to_owned()), (true, vec![100, 200, 300, 400]));
    assert!(!test_string(automaton.clone(), "".to_owned()).0);
    assert!(!test_string(automaton, "aa".to_owned()).0);
  }

  #[test]
  fn test_non_ascii_characters() {
    let automaton = automaton([
      state(100, true, false, &[("é", 200)]),
      state(200, false, true, &[("ü", 200)])
    ]);

    assert!(test_string(automaton.clone(), "éüü".to_owned()).0);
    assert!(!test_string(automaton.clone(), "ü".to_owned()).0);
    assert!(test_string(automaton, "é".to_owned()).0);
  }

  #[test]
  fn test_two_connections_on_one_character_are_both_followed() {
    let automaton = automaton([
      state(100, true, false, &[("a", 200), ("a", 300)]),
      state(200, false, false, &[]),
      state(300, false, true, &[])
    ]);

    for _ in 0..20 {
      assert_eq!(test_string(automaton.clone(), "a".to_owned()), (true, vec![100, 300]));
    }
  }

  #[test]
  fn test_rejected_string_shows_how_far_it_got() {
    let accepting_ab = automaton([
      state(100, true, false, &[("a", 200)]),
      state(200, false, false, &[("b", 300)]),
      state(300, false, true, &[])
    ]);

    assert_eq!(test_string(accepting_ab.clone(), "aa".to_owned()), (false, vec![100, 200]));
    assert_eq!(test_string(accepting_ab, "a".to_owned()), (false, vec![100, 200]));

    let no_start = automaton([state(100, false, true, &[])]);
    assert_eq!(test_string(no_start, "".to_owned()), (false, vec![]));
  }

  #[test]
  fn test_connections_to_missing_states_and_empty_characters_are_ignored() {
    let automaton = automaton([
      state(100, true, false, &[("a", 900), ("", 200), ("ab", 200), ("b", 200)]),
      state(200, false, true, &[])
    ]);

    assert!(test_string(automaton.clone(), "b".to_owned()).0);
    assert!(!test_string(automaton.clone(), "a".to_owned()).0);
    assert!(!test_string(automaton, "".to_owned()).0);
  }
}
