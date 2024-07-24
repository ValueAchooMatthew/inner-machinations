use app::models::{SmartState, State};
use std::collections::{HashMap, HashSet};

#[tauri::command]
pub fn test_string_dfa(state_positions: HashMap<String, State>, start_state_coordinates: String, string_to_check: String) -> (bool, Vec<State>) {

  let mut is_string_accepted: bool = false;
  let mut states_visited: Vec<State> = vec![];

  let start_state: &State = match state_positions.get(&start_state_coordinates){
    Some(state) => state,
    None => return (false, states_visited)
  };
  states_visited.push(start_state.to_owned());

  let mut current_state: &State = start_state;

  for connection_char in string_to_check.chars(){

    let next_state = match dfa_delta_function(&state_positions, current_state, connection_char.to_string()) {
      Some(state) => state,
      None => return (false, states_visited)
    };
    states_visited.push(next_state.to_owned());
    
    current_state = next_state;

    is_string_accepted = current_state.is_final();
  }

  (is_string_accepted, states_visited)
  
}

fn dfa_delta_function<'a>(state_positions: &'a HashMap<String, State>, s: &'a State, connection_character: String) -> Option<&'a State> {

  let connected_state_keys = s.get_connections_by_character(&connection_character)?;

  // Since in a DFA, every connection character must have exactly one connection, thus I just take the first return value
  // by iterating over the connected state keys

  for connected_state_key in connected_state_keys {
    return state_positions.get(connected_state_key);
  }

  // Done in case connected_state_key has len 0
  return None;

}

// For the purposes of this program, we will use a modified version of delta function as traditionally written in the form Q x a -> P,
// To allow for easier communication to and from the nfa string checking function
// The reason for the hashmap is so that, if a s

fn nfa_delta_function<'a>(state_positions: &'a HashMap<String, State>, 
  current_state: &'a State, 
  string_to_check: &String,
  states_visited: &'a mut Vec<State>) -> (bool, &'a mut Vec<State>) {
  
  if string_to_check.is_empty() && current_state.is_final() {
    return (true, states_visited);
  } else if string_to_check.is_empty() {
    let states_connected_by_epsilon = get_states_connected_by_epsilon_in_nfa(
      state_positions, 
      current_state);

    if states_connected_by_epsilon.is_some() {
      for state in states_connected_by_epsilon.unwrap() {

        // Checking if the bool value of states visited is true
        if nfa_delta_function(state_positions, state, string_to_check, states_visited).0 {
          return (true, states_visited);
        }
      };
    }
  
  }
  
  for character in string_to_check.chars() {
    let states_connected_by_character = get_states_connected_by_character_in_nfa(
      state_positions, 
      current_state,
      &character.to_string());
    
    let states_connected_by_epsilon = get_states_connected_by_epsilon_in_nfa(
      state_positions, 
      current_state);

    if states_connected_by_character.is_none() && states_connected_by_epsilon.is_none() {
      return (false, states_visited);
    }

    if states_connected_by_character.is_some() {

      for state in states_connected_by_character.unwrap() {

        // The rest of the string to be checked will be everything excluding the first character
        // Which was consumed when retrieving states_connected_by_character
        let string_to_check = String::from(&string_to_check[1..]);

        // Checking if the bool value of states visited is true
        if nfa_delta_function(state_positions, state, &string_to_check, states_visited).0 {
          states_visited.insert(0, state.to_owned());
          return (true, states_visited);
        } 
      }
    }

    if states_connected_by_epsilon.is_some() {

      for state in states_connected_by_epsilon.unwrap() {

        // Checking if the bool value of states visited is true
        if nfa_delta_function(state_positions, state, string_to_check, states_visited).0 {
          return (true, states_visited);
        }
      };
    }
  };
  return (false, states_visited);
}

fn get_states_connected_by_character_in_nfa<'a>(
  state_positions: &'a HashMap<String, State>, 
  s: &'a State, 
  connection_character: &String
  ) -> Option<HashSet<&'a State>> {

  s.get_connections_by_character(connection_character)
    .and_then(|connected_state_keys| {

      let mut state_references = HashSet::new();

      for connected_state_key in connected_state_keys {
        let connected_state = state_positions
          .get(connected_state_key)?;

        state_references.insert(connected_state);

      }

      return Some(state_references);
      
    })
    
}

fn get_states_connected_by_epsilon_in_nfa<'a>(
  state_positions: &'a HashMap<String, State>, 
  s: &'a State
) -> Option<HashSet<&'a State>> {

  s.get_connections_by_character("ϵ")
    .and_then(|connected_state_keys| {

      let mut state_references = HashSet::new();

      for connected_state_key in connected_state_keys {
        let connected_state = state_positions
          .get(connected_state_key)?;

        // Epsilon transitions connecting to the same state cannot be used as the program will just indefinitely check
        // The exact same string on the exact same state over and over again thus epsilon self looping cannot be sent to the delta function
        if s != connected_state {
          state_references.insert(connected_state);
        };
      };

      return Some(state_references);

    })

}

#[tauri::command]
pub fn test_string_nfa(
  state_positions: HashMap<String, State>, 
  start_state_coordinates: String, 
  string_to_check: String
  ) -> (bool, Vec<State>) {

  let mut states_visited: Vec<State> = vec![];

  let start_state = match state_positions.get(&start_state_coordinates) {
    Some(state) => state,
    None => return (false, states_visited)
  };

  // Ugly syntax but whatevs
  let result = nfa_delta_function(&state_positions, start_state, &string_to_check, &mut states_visited);
  return (result.0, result.1.to_owned());

}