use std::collections::{HashMap, HashSet};
use crate::models::State;

// Need to refactor to use less cloning in future
fn find_all_paths_to_state(start_state: &State, 
  end_state: &State, 
  consumed_string: &str, 
  strings_to_final_state: &mut HashMap<State, HashSet<String>>, 
  state_positions: &HashMap<String, State>,
  visited_states: HashSet<&State>) {

  if start_state == end_state {

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

  for (character_connection, state_keys) in &start_state.states_connected_to {

    for state_key in state_keys {

      let next_state = state_positions
        .get(state_key)
        .expect("There was a problem getting the state");

      let mut visited_states: HashSet<&State> = visited_states.clone();

      if visited_states.contains(next_state) {
        continue;
      }
      visited_states.insert(next_state);

      let string_consumed = consumed_string.to_owned() + character_connection;

      find_all_paths_to_state(next_state, 
        end_state, 
        string_consumed.as_str(), 
        strings_to_final_state, 
        &state_positions,
        visited_states);

    }
  }

}

fn find_unique_loops_to_given_state(
  current_state: &State, 
  end_state: &State, 
  strings_to_final_state: &mut HashMap<State, HashSet<String>>, 
  consumed_string: &str,
  state_positions: &HashMap<String, State>,
  visited_states: &mut HashMap<State, HashSet<State>>) {
  
  if current_state == end_state && consumed_string != "" {
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

  for (character_connection, state_keys) in &current_state.states_connected_to {

    for state_key in state_keys {

      let next_state = state_positions
      .get(state_key)
      .expect("There was a problem getting the state");

      if let Some(set) = visited_states.get_mut(current_state) {
        if set.contains(&next_state) {
          continue;
        }
        set.insert(next_state.clone());
      } else {
        visited_states.insert(current_state.clone(), HashSet::from([next_state.clone()]));
      }

      let string_consumed = consumed_string.to_owned() + character_connection;

      find_unique_loops_to_given_state(next_state, 
        end_state, 
        strings_to_final_state,
        string_consumed.as_str(),
        &state_positions,
        visited_states);

    }
  }

}

#[tauri::command]
pub fn determine_language_of_dfa(state_positions: HashMap<String, State>, start_state_key: String) {

  // Here's what I'm thinking
  // To determine the language of this dfa, i'm going to find every single unique path that leads to a final state.
  // Then, for every single final state, I'm going to determine if there exists any loops which bring us back to the SAME final state
  // if there is, than that loop can be repeated an indefinite number of times and thus consitutes a pattern matched by the dfa
  // To first find every single state connected to the start state, I must first perform a DFS of the graph

  let positions_of_final_states: &HashMap<&String, &State> = &state_positions
    .iter()
    .filter(|(_state_key, state)| state.is_final)
    .collect();

  let start_state = state_positions
    .get(&start_state_key)
    .expect("There was a problem getting the start state");

  let mut all_paths_to_reach_final_states = HashMap::new();
  let mut visited_states: HashSet<&State> = HashSet::new();
  visited_states.insert(start_state);

  let mut final_state_loops = HashMap::new();
  let mut visited_looping_states = HashMap::new();


  for final_state in positions_of_final_states.values() {
    find_all_paths_to_state(start_state, *final_state, "", &mut all_paths_to_reach_final_states, &state_positions, visited_states.clone());
  }

  for final_state in positions_of_final_states.values() {

    find_unique_loops_to_given_state(*final_state, *final_state, &mut final_state_loops, "", &state_positions, &mut visited_looping_states);
  }

  println!("reg: {:?}, loops:{:?}", all_paths_to_reach_final_states.values(), final_state_loops.values());

}