use std::collections::{BTreeSet, HashMap};

use crate::automaton::{parse_input_alphabet, Automaton, Dfa};
use crate::models::{AutomatonData, ConnectionData, Coordinate, StateData};

// Builds an automaton from (accepting, transitions) per state, with state 0 as the start.
// A symbol of None is an epsilon transition
fn automaton(states: &[(bool, &[(Option<char>, usize)])]) -> Automaton {
  let mut automaton = Automaton::new();
  for (is_accepting, _) in states {
    automaton.add_state(*is_accepting);
  }
  for (state, (_, transitions)) in states.iter().enumerate() {
    for (symbol, target) in transitions.iter() {
      automaton.add_transition(state, *symbol, *target);
    }
  }
  automaton.set_start(0);
  automaton
}

fn alphabet(characters: &str) -> BTreeSet<char> {
  characters.chars().collect()
}

fn all_strings_up_to(length: usize, characters: &[char]) -> Vec<String> {
  let mut strings = vec![String::new()];
  let mut previous_length = vec![String::new()];
  for _ in 0..length {
    previous_length = previous_length
      .iter()
      .flat_map(|string| characters.iter().map(move |character| format!("{string}{character}")))
      .collect();
    strings.extend(previous_length.iter().cloned());
  }
  strings
}

fn assert_equivalent(first: &Automaton, second: &Automaton, length: usize) {
  for string in all_strings_up_to(length, &['a', 'b']) {
    assert_eq!(first.accepts(&string), second.accepts(&string), "the automata disagree on {string:?}");
  }
}

fn transitions_count(dfa: &Dfa) -> usize {
  (0..dfa.state_count()).map(|state| dfa.transitions_from(state).count()).sum()
}

#[test]
fn test_epsilon_closure_follows_cycles_once() {
  let cycle = automaton(&[
    (false, &[(None, 1)]),
    (false, &[(None, 2)]),
    (false, &[(None, 0), (Some('a'), 3)]),
    (true, &[])
  ]);
  assert_eq!(cycle.epsilon_closure([0]), BTreeSet::from([0, 1, 2]));
  assert_eq!(cycle.epsilon_closure([3]), BTreeSet::from([3]));
  assert!(cycle.accepts("a"));
  assert!(!cycle.accepts(""));
}

#[test]
fn test_trace_matches_accepts() {
  let nfa = automaton(&[
    (false, &[(Some('a'), 0), (Some('b'), 0), (Some('a'), 1)]),
    (false, &[(None, 2), (Some('b'), 2)]),
    (true, &[(None, 1)])
  ]);
  for string in all_strings_up_to(6, &['a', 'b']) {
    let (accepted, path) = nfa.trace(&string);
    assert_eq!(accepted, nfa.accepts(&string), "{string:?}");
    assert_eq!(path.first(), Some(&0));
    if accepted {
      assert!(nfa.is_accepting(*path.last().unwrap()));
    }
  }
}

#[test]
fn test_determinizing_an_epsilon_cycle_terminates() {
  // Removing the epsilon transitions of this cycle used to loop forever
  let cycle = automaton(&[
    (false, &[(None, 1), (Some('a'), 1)]),
    (false, &[(None, 2), (Some('b'), 0)]),
    (true, &[(None, 0)])
  ]);
  let dfa = cycle.determinize(&BTreeSet::new()).unwrap();
  assert_eq!(dfa.state_count(), 1);
  assert!(dfa.is_accepting(0));
  assert_equivalent(&dfa.to_automaton(), &cycle, 6);
}

#[test]
fn test_is_deterministic() {
  assert!(automaton(&[(false, &[(Some('a'), 1), (Some('b'), 0)]), (true, &[])]).is_deterministic());
  assert!(!automaton(&[(false, &[(Some('a'), 1), (Some('a'), 0)]), (true, &[])]).is_deterministic());
  assert!(!automaton(&[(false, &[(None, 1)]), (true, &[])]).is_deterministic());
}

#[test]
fn test_determinize_needs_a_start_state() {
  let mut no_start = Automaton::new();
  no_start.add_state(true);
  assert!(no_start.determinize(&BTreeSet::new()).is_none());
}

#[test]
fn test_determinize_uses_the_input_alphabet() {
  let nfa = automaton(&[(false, &[(Some('a'), 1)]), (true, &[])]);
  let dfa = nfa.determinize(&alphabet("bc")).unwrap();
  assert!(!dfa.is_complete());

  let complete = dfa.complete();
  assert!(complete.is_complete());
  assert_eq!(complete.state_count(), 3);
  // Three characters from each of three states
  assert_eq!(transitions_count(&complete), 9);
}

#[test]
fn test_minimize_merges_equivalent_states() {
  // Strings over a and b that end in a, with states 1 and 2 equivalent
  let dfa = automaton(&[
    (false, &[(Some('a'), 1), (Some('b'), 0)]),
    (true, &[(Some('a'), 2), (Some('b'), 0)]),
    (true, &[(Some('a'), 1), (Some('b'), 0)])
  ])
  .determinize(&BTreeSet::new())
  .unwrap();

  let minimized = dfa.minimize();
  assert_eq!(minimized.state_count(), 2);
  assert!(minimized.is_complete());
  assert_equivalent(&minimized.to_automaton(), &dfa.to_automaton(), 6);
}

#[test]
fn test_minimize_removes_unreachable_states() {
  let dfa = automaton(&[
    (false, &[(Some('a'), 1)]),
    (true, &[]),
    (true, &[(Some('b'), 0)])
  ])
  .determinize(&BTreeSet::new())
  .unwrap();

  let minimized = dfa.minimize();
  assert_eq!(minimized.state_count(), 2);
}

#[test]
fn test_minimize_keeps_missing_transitions_missing() {
  // A partial DFA with an explicit dead state: the dead state goes, along with the transitions into it,
  // and nothing is redirected to a state that no longer exists
  let dfa = automaton(&[
    (false, &[(Some('a'), 1), (Some('b'), 2)]),
    (true, &[]),
    (false, &[(Some('a'), 2), (Some('b'), 2)])
  ])
  .determinize(&BTreeSet::new())
  .unwrap();

  let minimized = dfa.minimize();
  assert_eq!(minimized.state_count(), 2);
  assert_eq!(transitions_count(&minimized), 1);
  assert_equivalent(&minimized.to_automaton(), &dfa.to_automaton(), 6);
}

#[test]
fn test_minimize_keeps_the_dead_state_of_a_complete_dfa() {
  let dfa = automaton(&[
    (false, &[(Some('a'), 1), (Some('b'), 2)]),
    (true, &[(Some('a'), 2), (Some('b'), 2)]),
    (false, &[(Some('a'), 2), (Some('b'), 2)])
  ])
  .determinize(&BTreeSet::new())
  .unwrap();

  let minimized = dfa.minimize();
  assert_eq!(minimized.state_count(), 3);
  assert!(minimized.is_complete());
}

#[test]
fn test_minimize_an_empty_language_keeps_the_start_state() {
  let partial = automaton(&[(false, &[(Some('a'), 1)]), (false, &[])])
    .determinize(&BTreeSet::new())
    .unwrap()
    .minimize();
  assert_eq!(partial.state_count(), 1);
  assert!(!partial.is_accepting(0));
  assert_eq!(transitions_count(&partial), 0);

  let complete = automaton(&[(false, &[(Some('a'), 0)])])
    .determinize(&BTreeSet::new())
    .unwrap()
    .minimize();
  assert_eq!(complete.state_count(), 1);
  assert_eq!(transitions_count(&complete), 1);
}

#[test]
fn test_minimize_records_which_states_were_merged() {
  let dfa = automaton(&[
    (false, &[(Some('a'), 1), (Some('b'), 2)]),
    (true, &[]),
    (true, &[])
  ])
  .determinize(&BTreeSet::new())
  .unwrap();

  let minimized = dfa.minimize();
  assert_eq!(minimized.state_count(), 2);
  assert_eq!(minimized.members(0), &BTreeSet::from([0]));
  assert_eq!(minimized.members(1), &BTreeSet::from([1, 2]));
}

#[test]
fn test_from_data() {
  let automaton_data = AutomatonData {
    states: vec![
      StateData { id: 7, position: Coordinate { x: 200, y: 100 }, is_start: false, is_final: true },
      StateData { id: 3, position: Coordinate { x: 100, y: 100 }, is_start: true, is_final: false }
    ],
    connections: vec![
      connection(1, 3, 7, "a"),
      connection(2, 3, 7, "ϵ"),
      connection(3, 3, 99, "b"),
      connection(4, 3, 7, "ab"),
      connection(5, 3, 7, "")
    ]
  };

  let (automaton, state_ids) = Automaton::from_data(&automaton_data);
  assert_eq!(state_ids, vec![3, 7]);
  assert_eq!(automaton.start(), Some(0));
  assert_eq!(automaton.transitions_from(0).collect::<Vec<_>>(), vec![(None, 1), (Some('a'), 1)]);
  assert!(automaton.accepts(""));

  let no_start = AutomatonData { states: vec![automaton_data.states[0].clone()], connections: vec![] };
  let (automaton, _) = Automaton::from_data(&no_start);
  assert_eq!(automaton.start(), None);
  assert!(!automaton.accepts(""));
}

fn connection(id: i32, from: i32, to: i32, character: &str) -> ConnectionData {
  let point = Coordinate { x: 0, y: 0 };
  ConnectionData { id, from, to, character: character.to_owned(), control_points: [point, point] }
}

#[test]
fn test_parse_input_alphabet() {
  let input_alphabet = ["a", "", "bc", "ϵ", "é", "a"].map(String::from);
  assert_eq!(parse_input_alphabet(&input_alphabet), BTreeSet::from(['a', 'é']));
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

fn random_nfa(random: &mut Random) -> Automaton {
  let state_count = 1 + random.below(6);
  let mut nfa = Automaton::new();
  for _ in 0..state_count {
    nfa.add_state(random.below(3) == 0);
  }
  for state in 0..state_count {
    for _ in 0..random.below(4) {
      let symbol = [None, Some('a'), Some('b')][random.below(3)];
      nfa.add_transition(state, symbol, random.below(state_count));
    }
  }
  nfa.set_start(0);
  nfa
}

// Two states of a complete DFA are distinguishable when reading the same string from each can end in one
// accepting state and one that isn't. Searches the pairs of states reachable from the pair
fn are_distinguishable(dfa: &Dfa, first: usize, second: usize) -> bool {
  let mut visited_pairs = BTreeSet::from([(first, second)]);
  let mut pairs_to_visit = vec![(first, second)];
  while let Some((first, second)) = pairs_to_visit.pop() {
    if dfa.is_accepting(first) != dfa.is_accepting(second) {
      return true;
    }
    let targets_from_second: HashMap<char, usize> = dfa.transitions_from(second).collect();
    for (character, first_target) in dfa.transitions_from(first) {
      let pair = (first_target, targets_from_second[&character]);
      if visited_pairs.insert(pair) {
        pairs_to_visit.push(pair);
      }
    }
  }
  false
}

#[test]
fn test_random_automata_determinize_and_minimize() {
  let mut random = Random(0x9E3779B97F4A7C15);

  for _ in 0..500 {
    let nfa = random_nfa(&mut random);
    let dfa = nfa.determinize(&alphabet("ab")).unwrap();
    assert_equivalent(&dfa.to_automaton(), &nfa, 6);

    let minimized = dfa.minimize();
    assert_equivalent(&minimized.to_automaton(), &nfa, 6);
    assert_eq!(minimized.minimize().state_count(), minimized.state_count());
    assert!(minimized.state_count() <= dfa.state_count());

    // Minimizing a complete DFA gives the complete minimal DFA, whose states are all distinguishable
    let complete_minimized = dfa.clone().complete().minimize();
    assert!(complete_minimized.is_complete());
    assert_equivalent(&complete_minimized.to_automaton(), &nfa, 6);
    for first in 0..complete_minimized.state_count() {
      for second in first + 1..complete_minimized.state_count() {
        assert!(are_distinguishable(&complete_minimized, first, second));
      }
    }

    // The partial result is the complete one without its dead state, if it has one
    assert!(minimized.state_count() <= complete_minimized.state_count());
    assert!(complete_minimized.state_count() <= minimized.state_count() + 1);
  }
}
