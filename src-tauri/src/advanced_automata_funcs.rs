use std::collections::{HashMap, HashSet};
use crate::models::State;


#[tauri::command]
pub fn is_dfa_minimized(
  state_connections: HashMap<String, State>, 
  input_alphabet: Vec<String>) -> bool {

  let mut were_changes_made: bool = true;

  // We are creating a hashmap to allow for O(1) getting and setting time for "marking" pairs of states, which is required
  // for this algorithm to work, without needing to add an additional field to our existing state struct
  let mut marked_states: HashSet<(&String, &String)> = HashSet::new();

  while were_changes_made {
    were_changes_made = false;
    for (first_state_key, first_state) in &state_connections {
      for (second_state_key, second_state) in &state_connections {

        let current_states_key_pair = (first_state_key, second_state_key);
        // We do not need to check if the current pair of strings is marked if they are already marked
        if marked_states.contains(&current_states_key_pair) {
          continue;
        }

        // We mark two states if the are not both final or not final
        if first_state.is_final != second_state.is_final {
          marked_states.insert(current_states_key_pair);
          were_changes_made = true;
        }
        
        // Secondly, if there exists a connection such that, for the same connection character, 
        // the current two states lead to a pair of states which were previously marked, we must mark the current pair of states.
        for connection_character in &input_alphabet {

          let first_state_connection = first_state.states_connected_to
            .get(connection_character)
            .and_then(|connected_states| connected_states.get(0))
            .to_owned();

          let second_state_connection = second_state.states_connected_to
            .get(connection_character)
            .and_then(|connected_states| connected_states.get(0))
            .to_owned();
          
          if first_state_connection.is_none() || second_state_connection.is_none() {
            continue;
          }

          let connected_states_key_pair = (first_state_connection.unwrap(), second_state_connection.unwrap());

          if marked_states.contains(&connected_states_key_pair) {
            marked_states.insert(current_states_key_pair);
            were_changes_made = true;
          }

        }

      }

    }
    
  }

  if marked_states.len() == (state_connections.len().pow(2) - state_connections.len()) {
    return true
  }

  return false;

}




// pub fn minimize_dfa(state_connections: HashMap<String, State>, start_state_coordinates: String, string_to_check: String) {






// }