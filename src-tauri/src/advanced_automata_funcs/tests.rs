use std::collections::HashSet;

use crate::advanced_automata_funcs::{convert_nfa_to_dfa, default_control_points, minimize_dfa};
use crate::error::Error;
use crate::models::{AutomatonData, Coordinate};
use crate::test_support::{automaton, state, transitions};

fn alphabet(characters: &[&str]) -> Vec<String> {
  characters.iter().map(|character| character.to_string()).collect()
}

fn positions(automaton: &AutomatonData) -> Vec<(i32, i32, bool, bool)> {
  let mut positions: Vec<_> = automaton.states
    .iter()
    .map(|state| (state.position.x, state.position.y, state.is_start, state.is_final))
    .collect();
  positions.sort();
  positions
}

#[test]
fn test_minimizing_a_minimal_dfa_keeps_its_layout_and_curves() {
  let mut drawn = automaton([
    state(100, true, false, &[("a", 200)]),
    state(200, false, true, &[("b", 100)])
  ]);
  drawn.connections[0].control_points = [Coordinate { x: 150, y: 20 }, Coordinate { x: 180, y: 40 }];

  let minimized = minimize_dfa(drawn.clone(), alphabet(&["a", "b"])).unwrap();

  assert!(minimized.was_already_minimal);
  assert_eq!(positions(&minimized.automaton), vec![(100, 100, true, false), (200, 100, false, true)]);
  assert_eq!(transitions(&minimized.automaton), vec!["1 -a-> 2", "2 -b-> 1"]);
  let a_connection = minimized.automaton.connections.iter().find(|connection| connection.character == "a").unwrap();
  assert_eq!(a_connection.control_points, drawn.connections[0].control_points);
}

#[test]
fn test_minimizing_merges_equivalent_states() {
  // 200 and 300 both accept and loop back to the start on b
  let redundant = automaton([
    state(100, true, false, &[("a", 200), ("b", 100)]),
    state(200, false, true, &[("a", 300), ("b", 100)]),
    state(300, false, true, &[("a", 200), ("b", 100)])
  ]);

  let minimized = minimize_dfa(redundant, alphabet(&["a", "b"])).unwrap();

  assert!(!minimized.was_already_minimal);
  assert_eq!(positions(&minimized.automaton), vec![(100, 100, true, false), (200, 100, false, true)]);
  assert_eq!(transitions(&minimized.automaton), vec!["1 -a-> 2", "1 -b-> 1", "2 -a-> 2", "2 -b-> 1"]);
}

#[test]
fn test_unreachable_and_nondeterministic_dfas_aren_t_minimal() {
  let unreachable = automaton([
    state(100, true, true, &[("a", 100)]),
    state(200, false, false, &[])
  ]);
  assert!(!minimize_dfa(unreachable, alphabet(&["a"])).unwrap().was_already_minimal);

  let nondeterministic = automaton([
    state(100, true, false, &[("a", 100), ("a", 200)]),
    state(200, false, true, &[])
  ]);
  assert!(!minimize_dfa(nondeterministic, alphabet(&["a"])).unwrap().was_already_minimal);
}

#[test]
fn test_conversion_adds_a_dead_state_in_a_free_spot() {
  let nfa = automaton([
    state(100, true, false, &[("a", 200)]),
    state(200, false, true, &[])
  ]);

  let dfa = convert_nfa_to_dfa(nfa, alphabet(&["a", "b"])).unwrap();

  assert_eq!(positions(&dfa), vec![(100, 100, true, false), (200, 100, false, true), (300, 300, false, false)]);
  // Every state has exactly one connection on each character
  for state in &dfa.states {
    for character in ["a", "b"] {
      let count = dfa.connections.iter().filter(|connection| connection.from == state.id && connection.character == character).count();
      assert_eq!(count, 1);
    }
  }
}

#[test]
fn test_conversion_merges_states_joined_by_epsilon() {
  let nfa = automaton([
    state(100, true, false, &[("ϵ", 200)]),
    state(200, false, false, &[("ϵ", 100), ("a", 300)]),
    state(300, false, true, &[])
  ]);

  let dfa = convert_nfa_to_dfa(nfa, alphabet(&["a"])).unwrap();

  // {100, 200} goes where the start state was, {300} where 300 was, and the dead state to a free spot
  assert_eq!(positions(&dfa), vec![(100, 100, true, false), (300, 100, false, true), (300, 300, false, false)]);
  assert_eq!(transitions(&dfa), vec!["1 -a-> 2", "2 -a-> 3", "3 -a-> 3"]);
}

#[test]
fn test_a_start_state_is_needed() {
  let no_start = automaton([state(100, false, true, &[("a", 100)])]);
  assert_eq!(minimize_dfa(no_start.clone(), alphabet(&["a"])).unwrap_err(), Error::new("The DFA needs a start state to be minimized"));
  assert_eq!(convert_nfa_to_dfa(no_start, alphabet(&["a"])).unwrap_err(), Error::new("The NFA needs a start state to be converted"));
}

// Roughly where the frontend draws a connection's character: the middle of its curve
fn middle_of_curve(from: Coordinate, control_points: [Coordinate; 2], to: Coordinate) -> (i32, i32) {
  (
    (from.x + 3 * control_points[0].x + 3 * control_points[1].x + to.x) / 8,
    (from.y + 3 * control_points[0].y + 3 * control_points[1].y + to.y) / 8
  )
}

#[test]
fn test_connections_between_the_same_states_are_drawn_apart() {
  // Converting gives the dead state a loop on each character, and the final state two connections into it
  let nfa = automaton([
    state(100, true, false, &[("a", 200)]),
    state(200, false, true, &[])
  ]);
  let dfa = convert_nfa_to_dfa(nfa, alphabet(&["a", "b"])).unwrap();

  let position = |id: i32| dfa.states.iter().find(|state| state.id == id).unwrap().position;
  let middles: HashSet<(i32, i32)> = dfa.connections
    .iter()
    .map(|connection| middle_of_curve(position(connection.from), connection.control_points, position(connection.to)))
    .collect();
  assert_eq!(middles.len(), dfa.connections.len());
}

#[test]
fn test_default_control_points() {
  let left = Coordinate { x: 200, y: 200 };
  let right = Coordinate { x: 500, y: 200 };
  assert_eq!(default_control_points(left, right, false, 0), [left, right]);
  assert_eq!(default_control_points(left, right, false, 1), [Coordinate { x: 300, y: 300 }, Coordinate { x: 400, y: 300 }]);
  // The other way round, the next bend goes to the other side
  assert_eq!(default_control_points(right, left, false, 2), [Coordinate { x: 400, y: 100 }, Coordinate { x: 300, y: 100 }]);

  let centre = Coordinate { x: 500, y: 500 };
  assert_eq!(default_control_points(centre, centre, true, 0), [Coordinate { x: 300, y: 700 }, Coordinate { x: 300, y: 300 }]);
  assert_eq!(default_control_points(centre, centre, true, 1), [Coordinate { x: 300, y: 300 }, Coordinate { x: 700, y: 300 }]);
  assert_eq!(default_control_points(centre, centre, true, 4), [Coordinate { x: 200, y: 800 }, Coordinate { x: 200, y: 200 }]);
}
