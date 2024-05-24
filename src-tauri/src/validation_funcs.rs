use crate::models::State;
use std::collections::HashMap;

#[tauri::command]
pub fn verify_valid_dfa(state_connections: HashMap<String, State>, input_alphabet: Vec<String>) -> bool {

  for state in state_connections.values() {

    let connection_keys = state.states_connected_to.keys();
    // If the number of keys in the state doesn't equal the alphabet, we know automatically
    // That the dfa isn't valid
    if &connection_keys.len() != &input_alphabet.len() {
        return false;
    };

    for character in connection_keys {
      // Strictly speaking, in a DFA, every single state must have exactly 
      // one connection for each character in the input alphabet
      if !input_alphabet.contains(character){
        return false;
      };
      
      match state.states_connected_to.get(character) {
        Some(connected_state_keys) => {
          if connected_state_keys.len() != 1 {
            return false
          };
        },
        None => return false
      };
    };

  };
  return true;
}