use std::collections::HashMap;

use crate::miscellaneous::common_models::State;

#[tauri::command(rename_all = "snake_case")]
pub fn verify_valid_dfa(state_positions: HashMap<String, State>, input_alphabet: Vec<String>) -> bool {

  for state in state_positions.values() {

    let all_connections_from_state = state.get_all_connections();

    let connected_state_keys = all_connections_from_state.keys();

    // If the number of keys in the state doesn't equal the alphabet, we know automatically
    // That the dfa isn't valid
    if &connected_state_keys.len() != &input_alphabet.len() {
      return false;
    };

    for character in connected_state_keys {
      // Strictly speaking, in a DFA, every single state must have exactly 
      // one connection for each character in the input alphabet
      if !input_alphabet.contains(character){
        return false;
      };
      
      match state.get_connections_by_character(character) {
        Some(connected_state_keys) => {
          if connected_state_keys.len() != 1 {
            return false
          };
        },
        None => return false
      };
    };

  };
  true
}