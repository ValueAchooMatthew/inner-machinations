use std::collections::{HashMap, HashSet};

use app::models::TypeOfAutomata;

use app::models::State;
use crate::advanced_automata_funcs::reconstruct_nfa_state_positions;

// Need to refactor to use less cloning in future
fn find_all_paths_to_final_state(start_state: &State, 
  end_state: &State, 
  consumed_string: &str, 
  strings_to_final_state: &mut HashMap<State, HashSet<String>>, 
  state_positions: &HashMap<String, State>,
  visited_states: HashSet<&State>) {

  // Since we already handle the empty string prior to running this code
  // We will only count strings with a positive length

  if start_state == end_state && consumed_string.len() > 0 {

    match strings_to_final_state.get_mut(end_state) {
      Some(previous_ways) => {
        previous_ways
          .insert(consumed_string.to_owned());
      },
      None => {
        strings_to_final_state
          .insert(end_state.to_owned(), HashSet::from([consumed_string.to_owned()]));
      }
    };

    return;
  }

  for (character_connection, state_keys) in start_state.get_all_connections() {

    for state_key in state_keys {

      let next_state = state_positions
        .get(state_key)
        .expect("{There was a problem getting the state}");

      let mut visited_states: HashSet<&State> = visited_states.clone();

      if visited_states.contains(next_state) {
        continue;
      }
      visited_states.insert(next_state);

      let string_consumed = consumed_string.to_owned() + character_connection;

      find_all_paths_to_final_state(next_state, 
        end_state, 
        string_consumed.as_str(), 
        strings_to_final_state, 
        &state_positions,
        visited_states);

    }
  }

}

fn find_unique_loops_to_given_state(
  previous_state: Option<&State>,
  current_state: &State, 
  end_state: &State, 
  strings_to_final_state: &mut HashMap<State, HashSet<String>>, 
  consumed_string: &str,
  state_positions: &HashMap<String, State>,
  visited_states: &mut HashMap<State, HashSet<(String, State)>>
) {
  
  if current_state == end_state && consumed_string != "" {
    strings_to_final_state
      .entry(end_state.to_owned())
      .and_modify(|previous_ways_to_final_state| {
        previous_ways_to_final_state.insert(consumed_string.to_owned());
      }).or_insert(
        HashSet::from([consumed_string.to_owned()]
      ));
    return;
  }
  
  for (character_connection, state_keys) in current_state.get_all_connections() {
    
    for state_key in state_keys {
      
      let string_consumed = consumed_string.to_owned() + character_connection;
      
      let next_state = state_positions
        .get(state_key)
        .expect("There was a problem getting the state")
        .to_owned();
    
      // This is used to prevent redundant additional transitions. If we take connection "a" to a given state,
      // and the state we have transitioned to has a self loop of "a" to itself, without this there is a chance 
      // (since connections are visited in pseudo random order) that we take that self loop and then proceed to 
      // find all loops to the final state. Since the second "a" transition leads to us being in the same state, 
      // It does not qualify as unique and thus we want to continue if this is the case
      if previous_state.is_some() {
        let previous_state = previous_state.unwrap();

        if let Some(set) = visited_states.get(previous_state) {
          if set.contains(&(character_connection.to_owned(), current_state.clone())) && *current_state == next_state {
            continue;
          }
        }
      }
      
      if let Some(set) = visited_states.get_mut(current_state) {
        if set.contains(&(character_connection.to_owned(), next_state.clone())) {
          continue;
        }
        set.insert((character_connection.to_owned(), next_state.clone()));
      } else {
        visited_states.insert(current_state.clone(), HashSet::from([(character_connection.to_owned(), next_state.clone())]));
      }
      
      println!("cs: {current_state:?} c: {character_connection}, ns: {next_state:?} ");
      find_unique_loops_to_given_state(
        Some(&current_state),
        &next_state, 
        end_state, 
        strings_to_final_state,
        string_consumed.as_str(),
        &state_positions,
        visited_states
      );

    }
  }

}

fn convert_acceptance_paths_to_string(acceptance_paths: &HashMap<String, HashSet<String>>) -> String {

  let mut representation_of_acceptance_path = String::from("{");

  for (direct_path, looping_paths) in acceptance_paths {

    if representation_of_acceptance_path.len() == 1 {
      representation_of_acceptance_path += direct_path.as_str();
    } else {
      representation_of_acceptance_path += format!(" + {direct_path}").as_str();
    }

    if looping_paths.len() > 0 {

      representation_of_acceptance_path += "(";
      let mut looping_paths_to_add = String::new();

      for looping_path in looping_paths {
        if looping_paths_to_add.len() == 0 {
          looping_paths_to_add += looping_path.as_str();

        } else {
          looping_paths_to_add += format!(" + {looping_path}").as_str();
        }
      }
      representation_of_acceptance_path += looping_paths_to_add.as_str();
      representation_of_acceptance_path += ")\u{207F}";
    }

  };

  representation_of_acceptance_path += " | n ε \u{2115}}";

  return representation_of_acceptance_path;

}

// Todo: Fix
#[tauri::command]
pub fn determine_language_of_automata(
  state_positions: HashMap<String, State>, 
  start_state_key: &str, 
  type_of_automata: TypeOfAutomata,
) -> String {

  match type_of_automata {
    TypeOfAutomata::DFA => {
      determine_language_of_dfa(&state_positions, &start_state_key)
    },
    TypeOfAutomata::NFA => {
      // We convert to a DFA first
      let state_positions = reconstruct_nfa_state_positions(
        &state_positions, 
        start_state_key, 
      );

      // Janky, maybe fix in future
      let mut start_state_key = String::new();
      for state in state_positions.values() {
        if state.is_start() {
          start_state_key = state.get_position_as_string();
        }
      }
      if start_state_key != "" {
        determine_language_of_dfa(&state_positions, &start_state_key)
      } else {
        panic!("The start state position must exist!");
      }

    }
  }

}

fn determine_language_of_dfa(state_positions: &HashMap<String, State>, start_state_key: &str) -> String {
  // Here's what I'm thinking
  // To determine the language of this dfa, i'm going to find every single unique path that leads to a final state.
  // Then, for every single final state, I'm going to determine if there exists any loops which bring us back to the SAME final state
  // if there is, than that loop can be repeated an indefinite number of times and thus consitutes a pattern matched by the dfa
  // To first find every single state connected to the start state, I must first perform a DFS of the graph

  let positions_of_final_states: &HashMap<&String, &State> = &state_positions
    .iter()
    .filter(|(_state_key, state)| state.is_final())
    .collect();

  let start_state = state_positions
    .get(start_state_key)
    .expect("There was a problem getting the start state");

  let mut all_paths_to_reach_final_states = HashMap::new();
  let mut visited_states: HashSet<&State> = HashSet::new();

  let mut all_paths_to_acceptance: HashMap<String, HashSet<String>> = HashMap::new();

  if start_state.is_final() {
    // Just specifies that if the start state is final, the empty string should be accepted as well 
    all_paths_to_acceptance.insert(String::from("ε"), HashSet::new());
  }

  visited_states.insert(start_state);

  let mut looping_paths_to_final_states = HashMap::new();
  let mut visited_looping_states = HashMap::new();

  for final_state in positions_of_final_states.values() {
    find_all_paths_to_final_state(
      start_state, 
      *final_state,
      "",
      &mut all_paths_to_reach_final_states,
      &state_positions,
      visited_states.clone()
    );
  }

  for final_state in positions_of_final_states.values() {
    find_unique_loops_to_given_state(
      None,
      *final_state, 
      *final_state, 
      &mut looping_paths_to_final_states, 
      "", 
      &state_positions, 
      &mut visited_looping_states
    );
    visited_looping_states.clear();
  }

  for (final_state, direct_paths) in all_paths_to_reach_final_states {

    for direct_path in direct_paths {

      let looping_paths = match all_paths_to_acceptance
        .get_mut(&direct_path) {
          Some(paths) => paths,
          None => {
            // The direct path hasn't been added to the hashmap yet so we'll insert the path
            // then retrieve the newly created corresponding hashset
            all_paths_to_acceptance
              .insert(direct_path.to_owned(), HashSet::new());

            all_paths_to_acceptance
              .get_mut(&direct_path)
              .unwrap()
          }
        };

      if let Some(looping_paths_from_state) = looping_paths_to_final_states.get(&final_state) {

        let unionized: HashSet<String> = looping_paths
          .union(looping_paths_from_state)
          // The union produces a hashset of type &String when collected so we need to clone
          .cloned()
          .collect();

        all_paths_to_acceptance.insert(direct_path, unionized);

      }
  
    }
    
  };

  return convert_acceptance_paths_to_string(&all_paths_to_acceptance);

}


