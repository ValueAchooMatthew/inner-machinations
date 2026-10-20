use std::collections::HashMap;

use crate::automaton::Automaton;
use crate::models::{Coordinate, State};

use crate::language_determination_funcs::determine_language_of_automata;
use crate::regular_expression_funcs::{parse_regular_expression, convert_parse_tree_to_nfa};

fn state(x: i32, is_final: bool, connections: &[(&str, i32)]) -> (String, State) {
  let mut state = State::new(Coordinate { x, y: 100 }, x == 100, is_final);
  for (connection_character, target_x) in connections {
    state.add_connection(connection_character, format!("{target_x},100"));
  }
  (format!("{x},100"), state)
}

// The start state is always the one at x = 100
fn language(states: Vec<(String, State)>) -> String {
  determine_language_of_automata(HashMap::from_iter(states), "100,100")
}

#[test]
fn test_single_transition() {
  assert_eq!(language(vec![state(100, false, &[("a", 200)]), state(200, true, &[])]), "a");
}

#[test]
fn test_kleene_star() {
  assert_eq!(language(vec![state(100, true, &[("a", 100)])]), "a*");
  assert_eq!(language(vec![state(100, false, &[("a", 100), ("b", 200)]), state(200, true, &[])]), "a*b");
  assert_eq!(language(vec![state(100, true, &[("a", 100), ("b", 100)])]), "(a + b)*");
}

#[test]
fn test_empty_string_and_empty_language() {
  assert_eq!(language(vec![state(100, true, &[])]), "ε");
  assert_eq!(language(vec![state(100, false, &[("a", 200)]), state(200, false, &[])]), "∅");
  // A final state the start state can't reach
  assert_eq!(language(vec![state(100, false, &[]), state(200, true, &[])]), "∅");
  // A start state that doesn't exist
  let state_positions = HashMap::from([state(200, true, &[])]);
  assert_eq!(determine_language_of_automata(state_positions, "100,100"), "∅");
}

#[test]
fn test_loops_through_different_branches() {
  // Only one of the two loops used to be reported
  let states = vec![
    state(100, true, &[("a", 200), ("b", 300)]),
    state(200, false, &[("a", 100)]),
    state(300, false, &[("b", 100)])
  ];
  assert_eq!(language(states), "(aa + bb)*");
}

#[test]
fn test_simplifications() {
  // ε + aa* is a*
  let states = vec![state(100, true, &[("a", 200)]), state(200, true, &[("a", 200)])];
  assert_eq!(language(states), "a*");

  // ab + ac is a(b + c)
  let states = vec![
    state(100, false, &[("a", 200)]),
    state(200, false, &[("b", 300), ("c", 300)]),
    state(300, true, &[])
  ];
  assert_eq!(language(states), "a(b + c)");

  // Epsilon transitions disappear
  let states = vec![state(100, false, &[("ϵ", 200)]), state(200, true, &[("a", 200)])];
  assert_eq!(language(states), "a*");
}

#[test]
fn test_dead_and_unreachable_states_are_ignored() {
  let states = vec![
    state(100, false, &[("a", 200), ("b", 300)]),
    state(200, true, &[]),
    state(300, false, &[("a", 300)]),
    state(400, true, &[("a", 100)])
  ];
  assert_eq!(language(states), "a");
}

// A small deterministic random number generator, so the randomized tests are repeatable
struct Random(u64);

impl Random {
  fn below(&mut self, bound: usize) -> usize {
    self.0 ^= self.0 << 13;
    self.0 ^= self.0 >> 7;
    self.0 ^= self.0 << 17;
    (self.0 % bound as u64) as usize
  }
}

fn random_state_positions(random: &mut Random) -> HashMap<String, State> {
  let state_count = 1 + random.below(6);
  let mut states = vec![];
  for index in 0..state_count {
    let x = 100 * (index as i32 + 1);
    let mut connections = vec![];
    for _ in 0..random.below(4) {
      let character = ["a", "b", "ϵ"][random.below(3)];
      connections.push((character, 100 * (random.below(state_count) as i32 + 1)));
    }
    states.push(state(x, random.below(3) == 0, &connections));
  }
  HashMap::from_iter(states)
}

fn all_strings_up_to(length: usize) -> Vec<String> {
  let mut strings = vec![String::new()];
  let mut previous_length = vec![String::new()];
  for _ in 0..length {
    previous_length = previous_length
      .iter()
      .flat_map(|string| [format!("{string}a"), format!("{string}b")])
      .collect();
    strings.extend(previous_length.iter().cloned());
  }
  strings
}

// Reads each language back in with the regular expression builder and checks it matches the same strings
#[test]
fn test_random_automata_languages_round_trip() {
  let mut random = Random(0x2545F4914F6CDD1D);
  let strings = all_strings_up_to(6);

  for _ in 0..500 {
    let state_positions = random_state_positions(&mut random);
    let (automaton, _) = Automaton::from_state_positions(&state_positions, Some("100,100"));
    let language = determine_language_of_automata(state_positions.clone(), "100,100");

    if language == "∅" {
      for string in &strings {
        assert!(!automaton.accepts(string), "{state_positions:?} accepts {string:?} but its language is ∅");
      }
      continue;
    }

    let parse_tree = parse_regular_expression(&language)
      .unwrap_or_else(|error| panic!("{language:?} does not parse: {error:?}"));
    let nfa = convert_parse_tree_to_nfa(&parse_tree);

    for string in &strings {
      assert_eq!(
        nfa.accepts(string),
        automaton.accepts(string),
        "language {language:?} of {state_positions:?} disagrees on {string:?}"
      );
    }
  }
}

