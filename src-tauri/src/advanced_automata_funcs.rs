use std::{cell::RefCell, collections::{HashMap, HashSet}};
use app::{create_connections_from_state_positions, create_unique_state_coordinates, remove_all_epsilon_transitions};

use app::models::{Connection, Coordinate, SmartState, State};

fn mark_unequivalent_states_in_dfa(
  state_connections: &HashMap<String, State>, 
  input_alphabet: &Vec<String>,
) -> HashSet<(String, String)> {

  let mut finished = false;

  // We are creating a hashmap to allow for O(1) getting and setting time for "marking" pairs of states, which is required
  // for this algorithm to work, without needing to add an additional field to our existing state struct
  let mut marked_states: HashSet<(String, String)> = HashSet::new();

  while !finished {
    finished = true;
    for (first_state_key, first_state) in state_connections {
      for (second_state_key, second_state) in state_connections {

        let current_states_key_pair = (first_state_key.to_owned(), second_state_key.to_owned());
        // We do not need to check if the current pair of strings is marked if they are already marked
        if marked_states.contains(&current_states_key_pair) {
          continue;
        }

        // We mark two states if the are not both final or not final
        if first_state.is_final() != second_state.is_final() {
          marked_states.insert(current_states_key_pair.to_owned());
          finished = false;
        }

        // Secondly, if there exists a connection such that, for the same connection character, 
        // the current two states lead to a pair of states which were previously marked, we must mark the current pair of states.
        for connection_character in input_alphabet {

          let binding = HashSet::new();

          let set_as_vec = Vec::from_iter(first_state
            .get_connections_by_character(connection_character)
            .unwrap_or(&binding));

          let first_state_connection = set_as_vec.get(0);

          let set_as_vec = Vec::from_iter(second_state
            .get_connections_by_character(connection_character)
            .unwrap_or(&binding));

          let second_state_connection = set_as_vec.get(0);
          
          if first_state_connection.is_none() || second_state_connection.is_none() {
            continue;
          }

          let connected_states_key_pair = (
            first_state_connection
              .unwrap()
              .to_owned()
              .to_owned(),
            second_state_connection
              .unwrap()
              .to_owned()
              .to_owned()
          );

          if marked_states.contains(&connected_states_key_pair) {
            marked_states.insert(current_states_key_pair.to_owned());
            finished = false;
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
  // Which is to be included in the minimized state positions

  for (_, connected_state_keys) in state.get_all_connections_mut() {

    for connected_state_key in connected_state_keys.clone().iter() {

      if state_keys_to_be_ignored.contains(connected_state_key) {
        let equivalent_state_key = equivalent_state_keys
          .get(connected_state_key)
          .expect("There was an error retrieving the state's equivalent")
          .to_owned();

        connected_state_keys.remove(connected_state_key);
        connected_state_keys.insert(equivalent_state_key.to_owned());

      }
    
    }

  }

  return state;

}

fn remove_redundant_connections(
  connections: Vec<Connection>,
  equivalent_state_keys: &HashMap<&String, &String>,
  state_keys_to_be_ignored: &HashSet<&String>
) -> Vec<Connection> {

  let mut updated_connections = vec![];

  for mut connection in connections {

    let start_state_key: String = connection.curve.start_point.into();
    let end_state_key: String = connection.curve.end_point.into();
    
    if !state_keys_to_be_ignored.contains(&start_state_key) 
      && !state_keys_to_be_ignored.contains(&end_state_key) {

      updated_connections.push(connection);

    } else if !state_keys_to_be_ignored.contains(&start_state_key) {

      if let Some(equivalent_state_key) = equivalent_state_keys.get(&end_state_key) {
        
        // Looks ugly, fix in future
        let equivalent_state_key: Coordinate = equivalent_state_key
          .to_owned()
          .try_into()
          .expect("There was an error parsing the coordinate to a string");

        connection.curve.end_point = equivalent_state_key;

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
        if second_state.is_start() {
          first_state.make_start();
        }
      }

    }

    minimized_states.push(first_state.to_owned());

  };

  // If a state does not need to be ignored, we will still need to change all references made to an ignored state to its equivalent
  // Which we chose not to ignore
  // This must be done after the final list of all states to be ignored has been made
  for (index, state) in minimized_states.iter_mut().enumerate() {

    if state.is_start() {
      start_state_index = Some(index);
    }
    *state = remove_redundant_state_connections(state.to_owned(), &equivalent_state_keys, &state_keys_to_be_ignored);
    minimized_state_connections.insert(state.get_position_as_string(), state.to_owned());
  }

  let connections = remove_redundant_connections(connections, &equivalent_state_keys , &state_keys_to_be_ignored);

  return (start_state_index, minimized_states, connections, minimized_state_connections);

}

// Todo: Fix for a+b* and similar such cases
#[tauri::command]
pub fn convert_nfa_to_dfa (
  mut state_positions: HashMap<String, State>,
  start_state_position: String
  ) -> (Option<usize>, Vec<State>, Vec<Connection>, HashMap<String, State>) {
  
  remove_all_epsilon_transitions(&mut state_positions);
    
  let mut reconstructed_state_positions: HashMap<String, RefCell<State>> = HashMap::new();
  let mut start_state_index: Option<usize> = None;
  let mut reconstructed_states: Vec<State> = vec![];
  // I would like to be able to use a hashset of Strings here however unfortunately 
  // Hashsets cannot be hashed so we have to convert and ensure that we correctly sort the
  // hashsets we push into vecs
  let mut hashed_state_keys: HashMap<Vec<String>, String> = HashMap::new();

  let start_state = state_positions
    .get(&start_state_position)
    .expect("There was an error retrieving the start state")
    .to_owned();

  reconstructed_state_positions
    .insert(start_state_position.to_owned(), RefCell::from(start_state.to_owned()));
  
  let mut finished = false;

  while !finished {
    finished = true;
    let mut states_to_add = HashSet::new();

    'outer: for state in reconstructed_state_positions.values() {

      let mut state = state.borrow_mut();
      
      // The state's connection has to be be cloned to allow us to iterate over the existing states
      // without requiring an immutable reference, which would prevent us from later mutating the states 
      // based on the information obtained by iterating
      let connected_states = &state
        .get_all_connections()
        .to_owned();
      
      for (connection_character, connected_state_keys) in connected_states {

        let mut connected_state_keys_as_vec = Vec::from_iter(connected_state_keys.to_owned().into_iter());
        // Sorted so that anytime we do not insert [a, b] and [b, a] into the hashed_state_keys hashmap
        connected_state_keys_as_vec.sort();

        if connected_state_keys_as_vec.len() == 1 {

          let state_key_reference = connected_state_keys_as_vec
            .get(0)
            .expect("The hashset should contain at least 1 value");
          if reconstructed_state_positions.contains_key(state_key_reference) {
            // If the above condition is met, the state connection we're iterating over is a previously hashed vec which
            // we have renamed via the next else if clause. we thus do not need to keep renaming it and should continue
            // iterating
            continue;
          }
          finished = false;

        } 
        if !hashed_state_keys.contains_key(&connected_state_keys_as_vec) && connected_state_keys_as_vec.len() > 0 { 

          finished = false;

          let state_positions_in_reconstructed: HashSet<String> = reconstructed_state_positions
            .keys()
            .cloned()
            .collect();

          let all_reserved_positions: HashSet<String> = state_positions_in_reconstructed
            .union(&state_positions.keys().cloned().collect())
            .cloned()
            .collect();

          let unique_state_coords = create_unique_state_coordinates(&all_reserved_positions);
          let should_be_final = does_contain_final_state(&state_positions, &connected_state_keys);

          let all_connected_states = get_all_connected_state_keys(&state_positions, connected_state_keys);
          let mut new_state = State::new(unique_state_coords, false, should_be_final);
          
          new_state
            .set_all_connections(all_connected_states);

          state
            .remove_all_connections_by_character(connection_character);

          state
            .add_connection(connection_character, unique_state_coords);

          hashed_state_keys
            .insert(connected_state_keys_as_vec, unique_state_coords.into());

          states_to_add
            .insert(new_state);
          break 'outer;

        } else if connected_state_keys.len() > 0 { 

          finished = false;

          state
            .remove_all_connections_by_character(connection_character);

          state.add_connection(connection_character, 
            hashed_state_keys.get(&connected_state_keys_as_vec)
            .unwrap()
            .to_owned()
          );
          break 'outer;
        };
      };
    }

    for state in states_to_add {
      let state_key = state.get_position_as_string();

      if !reconstructed_state_positions.contains_key(&state_key) {
        reconstructed_state_positions.insert(state_key, RefCell::from(state));
      }
    }

  };

  // All necessary changes have been made to the reconstructed state positions, thus
  // we are safe to reconstruct the states and connections without fear they may later
  // become incorrect
  let mut reconstructed_state_positions_as_owned = HashMap::new();

  let connections = create_connections_from_state_positions(&reconstructed_state_positions);

  for (state_key, state) in reconstructed_state_positions {

    let state = state.borrow();

    reconstructed_states
      .push(state.to_owned());

    reconstructed_state_positions_as_owned
      .insert(state_key, state.to_owned());

    if state.is_start() {
      start_state_index = Some(reconstructed_states.len() - 1);
    }
  
  };
  // println!("{:?}", reconstructed_state_positions_as_owned.values());

  return (start_state_index, reconstructed_states, connections, reconstructed_state_positions_as_owned);

}

fn get_all_connected_state_keys (
  state_positions: &HashMap<String, State>, 
  state_keys: &HashSet<String>) -> HashMap<String, HashSet<String>> {

  let mut all_connected_state_keys = HashMap::new();

  for state_key in state_keys {

    let state = state_positions
      .get(state_key)
      .expect("Could not retrieve the specified state");

    for (connection_character, state_keys_connected_by_character) in state.get_all_connections().clone() {

      // In a DFA, we are not permitted to have epsilon state transitions and thus if we encounter an epsilon transition
      // We instead follow it and add the states which can be accessed by the epsilon transition
      if connection_character == "ϵ" {
        continue;
      }

      all_connected_state_keys
        .entry(connection_character)
        .and_modify(|current_entries: &mut HashSet<String>| {
          for key in &state_keys_connected_by_character {
            current_entries.insert(key.to_owned()); 
          }
        }
      )
      .or_insert(state_keys_connected_by_character);

    };

  };

  return all_connected_state_keys;

}

fn does_contain_final_state(state_positions: &HashMap<String, State>, state_keys: &HashSet<String>) -> bool {

  for state_key in state_keys {

    let is_final_state = state_positions
      .get(state_key)
      .expect("There was a problem retrieving the state");

    if is_final_state.is_final() {
      return true;
    }

  }
  return false;
}
