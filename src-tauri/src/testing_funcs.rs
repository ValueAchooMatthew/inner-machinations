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

fn nfa_looping(state_connections: &HashMap<String, State>, keys: &Vec<String>, string_to_check: &str) -> bool {
  for key in keys {

    let state =  match state_connections.get(key) {
      Some(state) => state,
      None => return false
    };

    let result = nfa_delta_function(state_connections, state, string_to_check);
    if result {
      return true;
    } else {
      continue;
    }
  };
  return false;
}

fn nfa_delta_function(state_connections: &HashMap<String, State>, current_state: &State, string_to_check: &str) -> bool {

  if string_to_check.len() == 0 && current_state.is_final {
    return true;
  }

  // Checking through epsilon transitions if no regular character transition exists
  // We are doing it separate from the regular character connections because the string to be checked
  // Must be different
  let binding = vec![];
  let epsilon_keys = match current_state.states_connected_to.get("ϵ") {
    Some(states) => states,
    None => &binding
  };

  let next_char = match &string_to_check.chars().nth(0){
    Some(char) => char.to_string(),
    None => { 
      if current_state.is_final {
        return true;
      }else{
        return nfa_looping(state_connections, &epsilon_keys, &string_to_check);
      }
    }
  };
  
  let connection_keys = match current_state.states_connected_to.get(&next_char) {
    Some(states) => states,
    None => return false
  };

  let rest_of_string = &string_to_check[1..];

  // For connections corresponding to the next character in the string to be checked
  // If the looping returns true, the NFA is guarenteed to accept the string, otherwise we must check
  // The epsilon transitions
  let does_connections_accept = nfa_looping(state_connections, connection_keys, &rest_of_string);
  if does_connections_accept {
    return true;
  }

  return nfa_looping(state_connections, &epsilon_keys, &string_to_check);

}

#[tauri::command]
pub fn test_string_nfa(state_connections: HashMap<String, State>, start_state_coordinates: String, string_to_check: String) -> bool {
  
  let start_state = match state_connections.get(&start_state_coordinates) {
    Some(state) => state,
    None => return false
  };

  return nfa_delta_function(&state_connections, start_state, &string_to_check);
}