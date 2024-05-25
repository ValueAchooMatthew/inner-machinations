use crate::models::State;
use std::collections::HashMap;

#[tauri::command]
pub fn test_string_dfa(state_connections: HashMap<String, State>, start_state_coordinates: String, string_to_check: String) -> bool {

  let mut is_string_accepted: bool = false;

  let start_state: &State = match state_connections.get(&start_state_coordinates){
    Some(state) => state,
    None => return false
  };

  let mut current_state: &State = start_state;

  for connection_char in string_to_check.chars(){

    let next_state = match dfa_delta_function(&state_connections, &current_state, connection_char.to_string()) {
      Some(state) => state,
      None => return false
    };

    current_state = next_state;

    if current_state.is_final == true {
      is_string_accepted = true;
    } else {
      is_string_accepted = false;
    }
  }

  is_string_accepted

}

// In a DFA, there can only be one (if any) state connected to a given state with a single connection char
// Therefore, if its  
fn dfa_delta_function<'a>(state_connections: &'a HashMap<String, State>, s: &'a State, connection_char: String) -> Option<&'a State> {

  let connected_state_keys = match s.states_connected_to.get(&connection_char) {
    Some(state_keys) => state_keys,
    None => return None
  };

  return match connected_state_keys.get(0) {
    Some(state_key) => {
      state_connections.get(state_key)
    }
    None => return None
  };

}


fn nfa_delta_function(state_connections: &HashMap<String, State>, current_state: &State, string_to_check: &str) -> bool {

  if string_to_check.len() == 0 && current_state.is_final {
    return true;
  }

  let next_char = match string_to_check.chars().nth(0) {
    Some(char) => char.to_string(),
    None => String::new()
  };

  let binding = vec![];
  let connection_keys_with_char  = match current_state.states_connected_to.get(&next_char) {
    Some(connections) => connections,
    None => &binding
  };

  let connection_keys_with_epsilon = match current_state.states_connected_to.get(&"ϵ".to_owned()) {
    Some(connections) => connections,
    None => &binding
  };


  for connection_key in connection_keys_with_char {
    let next_state = match state_connections.get(connection_key){
      Some(state)  => state,
      None => return false
    };

    // Even after exhausting all the regular character options, the nfa may still be valid depending on the results of the epsilon connections, 
    // hence we are not returning unless it returns true
    let result = nfa_delta_function(state_connections, next_state, &string_to_check[1..]);
    if result == true {
      return true;
    }
  }

  for epsilon_key in connection_keys_with_epsilon {
    let next_state = match state_connections.get(epsilon_key){
      Some(state)  => state,
      None => return false
    };
    let result = nfa_delta_function(state_connections, next_state, &string_to_check);
    if result == true {
      return true;
    }

  }
  return false;

}

#[tauri::command]
pub fn test_string_nfa(state_connections: HashMap<String, State>, start_state_coordinates: String, string_to_check: String) -> bool {
  
  let start_state = match state_connections.get(&start_state_coordinates) {
    Some(state) => state,
    None => return false
  };

  return nfa_delta_function(&state_connections, start_state, &string_to_check);
}