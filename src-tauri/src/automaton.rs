// Automata with states numbered from 0, independent of where the states sit on the canvas.
// The commands convert the frontend's states, which are keyed by position, into this form,
// run the algorithms here, and convert the result back.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use crate::models::State;

#[cfg(test)]
mod tests;

// The connection character the frontend uses for epsilon transitions
pub const EPSILON: &str = "ϵ";

#[derive(Debug, Clone, Default)]
pub struct Automaton {
  start: Option<usize>,
  accepting: Vec<bool>,
  // A symbol of None is an epsilon transition
  transitions: Vec<BTreeMap<Option<char>, BTreeSet<usize>>>
}

impl Automaton {

  pub fn new() -> Self {
    Self::default()
  }

  // Builds an automaton from the frontend's states, and returns it with each state's key, in index order.
  // Connections to states that don't exist, and characters that aren't a single character, are left out
  pub fn from_state_positions(state_positions: &HashMap<String, State>, start_state_key: Option<&str>) -> (Self, Vec<String>) {

    let mut state_keys: Vec<String> = state_positions.keys().cloned().collect();
    state_keys.sort();

    let mut automaton = Automaton::new();
    {
      let index_of_key: HashMap<&str, usize> = state_keys
        .iter()
        .enumerate()
        .map(|(index, key)| (key.as_str(), index))
        .collect();

      for key in &state_keys {
        automaton.add_state(state_positions[key].is_final());
      }

      for (index, key) in state_keys.iter().enumerate() {
        for (connection_character, connected_state_keys) in state_positions[key].get_all_connections() {

          let Some(symbol) = parse_connection_character(connection_character) else {
            continue;
          };

          for connected_state_key in connected_state_keys {
            if let Some(&target) = index_of_key.get(connected_state_key.as_str()) {
              automaton.add_transition(index, symbol, target);
            }
          }
        }
      }

      automaton.start = start_state_key.and_then(|key| index_of_key.get(key).copied());
    }

    (automaton, state_keys)
  }

  pub fn add_state(&mut self, is_accepting: bool) -> usize {
    self.accepting.push(is_accepting);
    self.transitions.push(BTreeMap::new());
    self.accepting.len() - 1
  }

  pub fn add_transition(&mut self, from: usize, symbol: Option<char>, to: usize) {
    self.transitions[from]
      .entry(symbol)
      .or_default()
      .insert(to);
  }

  pub fn set_start(&mut self, state: usize) {
    self.start = Some(state);
  }

  pub fn set_accepting(&mut self, state: usize, is_accepting: bool) {
    self.accepting[state] = is_accepting;
  }

  pub fn start(&self) -> Option<usize> {
    self.start
  }

  pub fn state_count(&self) -> usize {
    self.accepting.len()
  }

  pub fn is_accepting(&self, state: usize) -> bool {
    self.accepting[state]
  }

  pub fn transitions_from(&self, state: usize) -> impl Iterator<Item = (Option<char>, usize)> + '_ {
    self.transitions[state]
      .iter()
      .flat_map(|(symbol, targets)| targets.iter().map(move |target| (*symbol, *target)))
  }

  // Every character the automaton has a transition on, leaving out epsilon
  pub fn alphabet(&self) -> BTreeSet<char> {
    self.transitions
      .iter()
      .flat_map(|transitions| transitions.keys())
      .filter_map(|symbol| *symbol)
      .collect()
  }

  // The given states, and every state reachable from them using epsilon transitions alone.
  // The visited set means epsilon cycles are followed once rather than forever
  pub fn epsilon_closure(&self, states: impl IntoIterator<Item = usize>) -> BTreeSet<usize> {

    let mut closure = BTreeSet::new();
    let mut states_to_visit: Vec<usize> = states.into_iter().collect();

    while let Some(state) = states_to_visit.pop() {
      if !closure.insert(state) {
        continue;
      }
      if let Some(targets) = self.transitions[state].get(&None) {
        states_to_visit.extend(targets.iter().filter(|target| !closure.contains(*target)));
      }
    }

    closure
  }

  // The states reached from the given ones by reading the character, before following any epsilon transitions
  fn step(&self, states: &BTreeSet<usize>, character: char) -> BTreeSet<usize> {
    states
      .iter()
      .filter_map(|&state| self.transitions[state].get(&Some(character)))
      .flatten()
      .copied()
      .collect()
  }

  pub fn accepts(&self, string_to_check: &str) -> bool {

    let Some(start) = self.start else {
      return false;
    };

    let mut current_states = self.epsilon_closure([start]);

    for character in string_to_check.chars() {
      current_states = self.epsilon_closure(self.step(&current_states, character));
      if current_states.is_empty() {
        return false;
      }
    }

    current_states.iter().any(|&state| self.accepting[state])
  }

  // Returns whether the string is accepted, along with a path through the automaton for showing the traversal:
  // an accepting path if there is one, and otherwise a path that reads as much of the string as possible.
  // Epsilon transitions appear as steps of the path.
  // Searches pairs of (state, characters read so far), each at most once, so epsilon cycles cannot recurse forever
  pub fn trace(&self, string_to_check: &str) -> (bool, Vec<usize>) {

    let Some(start) = self.start else {
      return (false, vec![]);
    };

    let characters: Vec<char> = string_to_check.chars().collect();
    let start_configuration = (start, 0);

    let mut previous_configurations: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut visited_configurations = HashSet::from([start_configuration]);
    let mut configurations_to_visit = VecDeque::from([start_configuration]);
    let mut furthest_configuration = start_configuration;

    while let Some((state, characters_read)) = configurations_to_visit.pop_front() {

      if characters_read == characters.len() && self.accepting[state] {
        return (true, Self::path_to((state, characters_read), &previous_configurations));
      }

      if characters_read > furthest_configuration.1 {
        furthest_configuration = (state, characters_read);
      }

      for (symbol, target) in self.transitions_from(state) {

        let next_configuration = match symbol {
          None => (target, characters_read),
          Some(character) if characters.get(characters_read) == Some(&character) => (target, characters_read + 1),
          Some(_) => continue
        };

        if visited_configurations.insert(next_configuration) {
          previous_configurations.insert(next_configuration, (state, characters_read));
          configurations_to_visit.push_back(next_configuration);
        }
      }
    }

    (false, Self::path_to(furthest_configuration, &previous_configurations))
  }

  fn path_to(configuration: (usize, usize), previous_configurations: &HashMap<(usize, usize), (usize, usize)>) -> Vec<usize> {

    let mut path = vec![configuration.0];
    let mut current_configuration = configuration;

    while let Some(&previous_configuration) = previous_configurations.get(&current_configuration) {
      path.push(previous_configuration.0);
      current_configuration = previous_configuration;
    }

    path.reverse();
    path
  }

  // States reachable from the start state that can also reach an accepting state
  pub fn useful_states(&self) -> BTreeSet<usize> {

    let Some(start) = self.start else {
      return BTreeSet::new();
    };

    let mut reachable_states = BTreeSet::from([start]);
    let mut states_to_visit = vec![start];
    while let Some(state) = states_to_visit.pop() {
      for (_, target) in self.transitions_from(state) {
        if reachable_states.insert(target) {
          states_to_visit.push(target);
        }
      }
    }

    let mut states_leading_to = vec![vec![]; self.state_count()];
    for state in 0..self.state_count() {
      for (_, target) in self.transitions_from(state) {
        states_leading_to[target].push(state);
      }
    }

    let mut states_reaching_acceptance: BTreeSet<usize> = (0..self.state_count())
      .filter(|&state| self.accepting[state])
      .collect();
    let mut states_to_visit: Vec<usize> = states_reaching_acceptance.iter().copied().collect();
    while let Some(state) = states_to_visit.pop() {
      for &source in &states_leading_to[state] {
        if states_reaching_acceptance.insert(source) {
          states_to_visit.push(source);
        }
      }
    }

    reachable_states
      .intersection(&states_reaching_acceptance)
      .copied()
      .collect()
  }

  // The subset construction, over the given characters and every character the automaton uses.
  // Each state of the result is the epsilon closure of a set of this automaton's states, and only
  // states reachable from the start are created. None if there is no start state
  pub fn determinize(&self, input_alphabet: &BTreeSet<char>) -> Option<Dfa> {

    let start = self.start?;
    let alphabet: Vec<char> = input_alphabet
      .union(&self.alphabet())
      .copied()
      .collect();

    let mut dfa = Dfa {
      alphabet,
      transitions: vec![],
      accepting: vec![],
      members: vec![]
    };
    let mut index_of_members: BTreeMap<BTreeSet<usize>, usize> = BTreeMap::new();

    let start_members = self.epsilon_closure([start]);
    let start_state = dfa.add_state(start_members.clone(), self.contains_accepting_state(&start_members));
    index_of_members.insert(start_members, start_state);

    let mut states_to_visit = VecDeque::from([start_state]);

    while let Some(state) = states_to_visit.pop_front() {

      let members = dfa.members[state].clone();

      for symbol_index in 0..dfa.alphabet.len() {

        let target_members = self.epsilon_closure(self.step(&members, dfa.alphabet[symbol_index]));
        if target_members.is_empty() {
          continue;
        }

        let target = match index_of_members.get(&target_members) {
          Some(&target) => target,
          None => {
            let is_accepting = self.contains_accepting_state(&target_members);
            let target = dfa.add_state(target_members.clone(), is_accepting);
            index_of_members.insert(target_members, target);
            states_to_visit.push_back(target);
            target
          }
        };

        dfa.transitions[state][symbol_index] = Some(target);
      }
    }

    Some(dfa)
  }

  fn contains_accepting_state(&self, states: &BTreeSet<usize>) -> bool {
    states.iter().any(|&state| self.accepting[state])
  }

}

// A DFA whose start state is state 0, and whose states are all reachable from it.
// A state may have no transition on a character, in which case reading it rejects the string
#[derive(Debug, Clone)]
pub struct Dfa {
  alphabet: Vec<char>,
  // transitions[state][index of a character in the alphabet]
  transitions: Vec<Vec<Option<usize>>>,
  accepting: Vec<bool>,
  // The states of the original automaton that each state stands for
  members: Vec<BTreeSet<usize>>
}

impl Dfa {

  fn add_state(&mut self, members: BTreeSet<usize>, is_accepting: bool) -> usize {
    self.transitions.push(vec![None; self.alphabet.len()]);
    self.accepting.push(is_accepting);
    self.members.push(members);
    self.accepting.len() - 1
  }

  pub fn state_count(&self) -> usize {
    self.accepting.len()
  }

  pub fn is_accepting(&self, state: usize) -> bool {
    self.accepting[state]
  }

  pub fn members(&self, state: usize) -> &BTreeSet<usize> {
    &self.members[state]
  }

  pub fn transitions_from(&self, state: usize) -> impl Iterator<Item = (char, usize)> + '_ {
    self.alphabet
      .iter()
      .zip(&self.transitions[state])
      .filter_map(|(character, target)| Some((*character, (*target)?)))
  }

  // Whether every state has a transition on every character
  pub fn is_complete(&self) -> bool {
    self.transitions
      .iter()
      .all(|transitions| transitions.iter().all(Option::is_some))
  }

  // Sends every missing transition to a new dead state, which stands for no original states
  pub fn complete(mut self) -> Self {

    if self.is_complete() {
      return self;
    }

    let dead_state = self.add_state(BTreeSet::new(), false);
    for transitions in &mut self.transitions {
      for target in transitions.iter_mut() {
        if target.is_none() {
          *target = Some(dead_state);
        }
      }
    }

    self
  }

  // Merges equivalent states by partition refinement. A missing transition counts as one to a dead state.
  // When this DFA has missing transitions, so does the result: states that can never reach acceptance are
  // removed, along with the transitions into them. When it has none, the result keeps its dead state, if it needs one.
  // The start state stays even when the language is empty
  pub fn minimize(&self) -> Self {

    let state_count = self.state_count();
    let alphabet_size = self.alphabet.len();
    let is_complete = self.is_complete();

    // The dead state that missing transitions go to is numbered state_count
    let dead_state = state_count;
    let target = |state: usize, symbol_index: usize| -> usize {
      if state == dead_state {
        return dead_state;
      }
      self.transitions[state][symbol_index].unwrap_or(dead_state)
    };

    // Start by separating accepting states from the rest, then split states apart whenever some character
    // takes them to different classes, until nothing changes
    let mut class_of: Vec<usize> = (0..=state_count)
      .map(|state| if state < state_count && self.accepting[state] { 1 } else { 0 })
      .collect();

    loop {
      let class_count = class_of.iter().collect::<BTreeSet<_>>().len();
      let mut class_of_signature: BTreeMap<(usize, Vec<usize>), usize> = BTreeMap::new();
      let mut refined_class_of = Vec::with_capacity(state_count + 1);

      for state in 0..=state_count {
        let signature = (
          class_of[state],
          (0..alphabet_size).map(|symbol_index| class_of[target(state, symbol_index)]).collect()
        );
        let next_class = class_of_signature.len();
        refined_class_of.push(*class_of_signature.entry(signature).or_insert(next_class));
      }

      class_of = refined_class_of;
      if class_of_signature.len() == class_count {
        break;
      }
    }

    let dead_class = class_of[dead_state];
    let start_class = class_of[0];

    // Number the classes in the order they are reached from the start
    let mut minimized = Dfa {
      alphabet: self.alphabet.clone(),
      transitions: vec![],
      accepting: vec![],
      members: vec![]
    };
    let mut state_of_class: BTreeMap<usize, usize> = BTreeMap::new();
    let mut representatives: Vec<usize> = vec![];

    state_of_class.insert(start_class, minimized.add_state(BTreeSet::new(), self.accepting[0]));
    representatives.push(0);

    let mut next_state = 0;
    while next_state < representatives.len() {

      let representative = representatives[next_state];

      for symbol_index in 0..alphabet_size {

        let target_state = target(representative, symbol_index);
        let target_class = class_of[target_state];

        if !is_complete && target_class == dead_class {
          continue;
        }

        let minimized_target = match state_of_class.get(&target_class) {
          Some(&minimized_target) => minimized_target,
          None => {
            let minimized_target = minimized.add_state(BTreeSet::new(), self.accepting[target_state]);
            state_of_class.insert(target_class, minimized_target);
            representatives.push(target_state);
            minimized_target
          }
        };

        minimized.transitions[next_state][symbol_index] = Some(minimized_target);
      }

      next_state += 1;
    }

    for state in 0..state_count {
      if let Some(&minimized_state) = state_of_class.get(&class_of[state]) {
        minimized.members[minimized_state].extend(&self.members[state]);
      }
    }

    minimized
  }

  pub fn to_automaton(&self) -> Automaton {

    let mut automaton = Automaton::new();

    for state in 0..self.state_count() {
      automaton.add_state(self.accepting[state]);
    }

    for state in 0..self.state_count() {
      for (character, target) in self.transitions_from(state) {
        automaton.add_transition(state, Some(character), target);
      }
    }

    automaton.set_start(0);
    automaton
  }

}

// Some(None) for an epsilon transition and Some(Some(character)) for a single character.
// None for anything else, which no string can follow
fn parse_connection_character(connection_character: &str) -> Option<Option<char>> {

  if connection_character == EPSILON {
    return Some(None);
  }

  let mut characters = connection_character.chars();
  match (characters.next(), characters.next()) {
    (Some(character), None) => Some(Some(character)),
    _ => None
  }
}

// The single characters of an input alphabet, leaving out empty entries and epsilon
pub fn parse_input_alphabet(input_alphabet: &[String]) -> BTreeSet<char> {
  input_alphabet
    .iter()
    .filter_map(|input_character| parse_connection_character(input_character).flatten())
    .collect()
}
