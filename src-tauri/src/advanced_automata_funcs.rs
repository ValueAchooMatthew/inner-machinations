use std::collections::{HashMap, HashSet};
use crate::models::{Connection, Coordinate, State};


fn mark_unequivalent_states_in_dfa(
  state_connections: &HashMap<String, State>, 
  input_alphabet: &Vec<String>,
) -> HashSet<(String, String)> {

  let mut were_changes_made: bool = true;

  // We are creating a hashmap to allow for O(1) getting and setting time for "marking" pairs of states, which is required
  // for this algorithm to work, without needing to add an additional field to our existing state struct
  let mut marked_states: HashSet<(String, String)> = HashSet::new();

  while were_changes_made {
    were_changes_made = false;
    for (first_state_key, first_state) in state_connections {
      for (second_state_key, second_state) in state_connections {

        let current_states_key_pair = (first_state_key.to_owned(), second_state_key.to_owned());
        // We do not need to check if the current pair of strings is marked if they are already marked
        if marked_states.contains(&current_states_key_pair) {
          continue;
        }

        // We mark two states if the are not both final or not final
        if first_state.is_final != second_state.is_final {
          marked_states.insert(current_states_key_pair.to_owned());
          were_changes_made = true;
        }

        // Secondly, if there exists a connection such that, for the same connection character, 
        // the current two states lead to a pair of states which were previously marked, we must mark the current pair of states.
        for connection_character in input_alphabet {

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

          let connected_states_key_pair = (
            first_state_connection
              .unwrap()
              .to_owned(), 
            second_state_connection
              .unwrap()
              .to_owned()
          );

          if marked_states.contains(&connected_states_key_pair) {
            marked_states.insert(current_states_key_pair.to_owned());
            were_changes_made = true;
          }

        }

      }

    }
    
  }
  return marked_states;



}

#[tauri::command]
pub fn is_dfa_minimal(
  state_connections: HashMap<String, State>, 
  input_alphabet: Vec<String>) -> bool {

  let marked_states = mark_unequivalent_states_in_dfa(&state_connections, &input_alphabet);

  // If this is true, we know that the provided DFA is already minimized and thus, there is no need to alter state_connections
  if marked_states.len() == (state_connections.len().pow(2) - state_connections.len()) {
    return true;
  }

  return false;

}

fn remove_redundant_state_connections(
  mut state: State, 
  equivalent_state_keys: &HashMap<&String, &String>,
  state_keys_to_be_ignored: &HashSet<&String>) -> State {
  // This function just replaces all instances of a state referenced in the given state's connections, with the non redundant equivalent version
  // Which is to be included in the minimized state connections

  let mut updated_state_connections: HashMap<String, Vec<String>> = HashMap::new();

  for (connection_character, connected_state_keys) in &state.states_connected_to {

    let mut updated_state_connections_by_character = vec![];

    for connected_state_key in connected_state_keys {

      let mut state_key_to_add = connected_state_key;

      if state_keys_to_be_ignored.contains(connected_state_key) {
        let connected_state_key = equivalent_state_keys
          .get(connected_state_key)
          .expect("There was an error retrieving the state's equivalent")
          .to_owned();

        state_key_to_add = connected_state_key;

      }

      if !updated_state_connections_by_character.contains(state_key_to_add) {
        updated_state_connections_by_character.push(state_key_to_add.to_owned());
      }
    
    }
    updated_state_connections.insert(connection_character.to_owned(), updated_state_connections_by_character);

  }

  state.states_connected_to = updated_state_connections;

  return state;

}


fn remove_redundant_connections(
  connections: Vec<Connection>,
  equivalent_state_keys: &HashMap<&String, &String>,
  state_keys_to_be_ignored: &HashSet<&String>
) -> Vec<Connection> {

  let mut updated_connections = vec![];

  for mut connection in connections {

    let start_state_key = connection.curve.start_point.convert_coords_to_string();
    let end_state_key = connection.curve.end_point.convert_coords_to_string();
    
    if !state_keys_to_be_ignored.contains(&start_state_key) 
      && !state_keys_to_be_ignored.contains(&end_state_key) {

      updated_connections.push(connection);

    } else if !state_keys_to_be_ignored.contains(&start_state_key) {

      if let Some(equivalent_state_key) = equivalent_state_keys.get(&end_state_key) {
        
        let equivalent_state_key = Coordinate::convert_string_to_coords(*equivalent_state_key)
          .expect("There was an error parsing the coordinate to a tring");

        connection.curve.end_point = equivalent_state_key.to_owned();

        updated_connections.push(connection);

      }

    }

  };

  return updated_connections;

}

#[tauri::command]
pub fn minimize_dfa(
  state_connections: HashMap<String, State>,
  connections: Vec<Connection>,
  input_alphabet: Vec<String>
  ) -> (Option<usize>, Vec<State>, Vec<Connection>, HashMap<String, State>){

  let marked_states = mark_unequivalent_states_in_dfa(&state_connections, &input_alphabet);

  let mut start_state_index: Option<usize> = None;
  let mut minimized_states = vec![];
  let mut minimized_state_connections: HashMap<String, State> = HashMap::new();
  let mut state_keys_to_be_ignored: HashSet<&String> = HashSet::new();
  let mut equivalent_state_keys: HashMap<&String, &String> = HashMap::new();

  for (first_state_key, first_state) in &state_connections {

    // The state we are currently on is equivalent to one which we've already included in our
    // minimized state connections, thus we do not to to verify if it is equivalent to any other states.
    // This is because equivalence is reflexive, if we know a = b, we do not need to check if b = c, because a = c
    // is by definition true and would have been accounted for in an earlier iteration
    if state_keys_to_be_ignored.contains(first_state_key) {
      continue;
    }
    let mut first_state = first_state
      .to_owned();

    for (second_state_key, second_state) in &state_connections {

      let current_states_key_pair = (first_state_key.to_owned(), second_state_key.to_owned());

      // If a given pair of states are not present in the hash set, that means they are equivalent. 
      // Since a given state is always equivalent to itself, we will only make alterations if two states are equivalent,
      // And they do not have the same key

      if !marked_states.contains(&current_states_key_pair) && first_state_key != second_state_key {
        // First state is equivalent to the second state
        state_keys_to_be_ignored.insert(&second_state_key);
        equivalent_state_keys.insert(&second_state_key, first_state_key);

        // If the state we are choosing to keep in our minimized state connections is equivalent to a start state, 
        // Then it must therefore also be the start state in the minimized state connections
        if second_state.is_start {
          first_state.is_start = true;
        }
      }

    }

    minimized_states.push(first_state.to_owned());

  };

  // If a state does not need to be ignored, we will still need to change all references made to an ignored state to its equivalent
  // Which we chose not to ignore
  // This must be done after the final list of all states to be ignored has been made
  for (index, state) in minimized_states.iter_mut().enumerate() {

    if state.is_start {
      start_state_index = Some(index);
    }
    *state = remove_redundant_state_connections(state.to_owned(), &equivalent_state_keys, &state_keys_to_be_ignored);
    minimized_state_connections.insert(state.position.convert_coords_to_string().to_owned(), state.to_owned());
  }

  let connections = remove_redundant_connections(connections, &equivalent_state_keys , &state_keys_to_be_ignored);

  return (start_state_index, minimized_states, connections, minimized_state_connections);

}