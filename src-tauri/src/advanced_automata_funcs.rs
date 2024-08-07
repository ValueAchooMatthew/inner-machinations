use std::{borrow::BorrowMut, cell::RefCell, collections::{HashMap, HashSet}};
use crate::models::{BezierCurve, Connection, Coordinate, State};

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
    minimized_state_connections.insert(state.position.into(), state.to_owned());
  }

  let connections = remove_redundant_connections(connections, &equivalent_state_keys , &state_keys_to_be_ignored);

  return (start_state_index, minimized_states, connections, minimized_state_connections);

}

#[tauri::command]
pub fn convert_nfa_to_dfa (
  state_positions: HashMap<String, State>,
  start_state_position: String
  ) -> (Option<usize>, Vec<State>, Vec<Connection>, HashMap<String, RefCell<State>>) {

  let mut reconstructed_state_positions: HashMap<String, RefCell<State>> = HashMap::new();
  let mut start_state_index: Option<usize> = None;
  let mut reconstructed_states: Vec<State> = vec![];
  let mut hashed_state_keys: HashMap<Vec<String>, String> = HashMap::new();

  let start_state = state_positions
    .get(&start_state_position)
    .expect("There was an error retrieving the start state")
    .to_owned();

  reconstructed_state_positions
    .insert(start_state_position, RefCell::from(start_state));
  
  let mut finished = false;

  while !finished {
    finished = true;
    let mut states_to_add = vec![];

    for state in  reconstructed_state_positions.values() {

      let mut state = state.borrow_mut();
      // The state's connection has to be be cloned to allow us to iterate over the existing states
      // without requiring an immutable reference, which would prevent us from later mutating the states 
      // based on the information obtained by iterating
      let mut states_connected_to = state
        .clone()
        .states_connected_to;
      
      for (connection_character, states_connected_to) in &mut states_connected_to {
        
        // May way to change this to work with binary sort later for faster performance
        states_connected_to.sort();

        if states_connected_to.len() == 1 && reconstructed_state_positions.contains_key(states_connected_to.get(0).unwrap()) {
          // If the above condition is met, the state connection we're iterating over is a previously hashed vec which
          // we have renamed via the next else if clause. we thus do not need to keep renaming it and should continue
          // iterating
          continue;

        } else if !hashed_state_keys.contains_key(states_connected_to) && states_connected_to.len() > 0 { 

          finished = false;
          let unique_state_coords = create_unique_state_coordinates(&reconstructed_state_positions);
          let should_be_final = does_vec_contain_final_state(&state_positions, &states_connected_to);

          let all_connected_states = get_all_state_keys_connected_by_vec(&state_positions, &states_connected_to);

          let new_state = State {
            position: unique_state_coords,
            states_connected_to: all_connected_states,
            is_final: should_be_final,
            is_start: false,
            element: String::from("State")
          };  

          state.borrow_mut()
            .states_connected_to.insert(connection_character.to_owned(), vec![unique_state_coords.into()]);
          hashed_state_keys
            .insert(states_connected_to.to_owned(), unique_state_coords.into());
          states_to_add
            .push((unique_state_coords, new_state));

        } else if states_connected_to.len() > 0 { 
          // If the state connection we're iterating over has a length but has not yet 

          state
            .states_connected_to.insert(connection_character.to_owned(), vec![
            hashed_state_keys.get(states_connected_to)
            .unwrap()
            .to_owned()
          ]);
        }
      };
    }

    for (state_coords, state) in states_to_add {
      let state_key: String = state_coords
        .into();

      if !reconstructed_state_positions.contains_key::<String>(&state_key) {
        reconstructed_state_positions.insert(state_key, RefCell::from(state));
      }
    }

  };

  // All necessary changes have been made to the reconstructed state positions, thus
  // we are safe to reconstruct the states and connections without fear they may later
  // become incorrect

  for state in reconstructed_state_positions.values_mut() {

    let state = state
      .get_mut();

    reconstructed_states
      .push(state.to_owned());

    if state.is_start {
      start_state_index = Some(reconstructed_states.len() - 1);
    }
  
  };

  let connections = create_connections_from_state_positions(&reconstructed_state_positions);

  return (start_state_index, reconstructed_states, connections, reconstructed_state_positions);

}

fn get_all_state_keys_connected_by_vec (
  state_positions: &HashMap<String, State>, 
  state_keys: &Vec<String>) -> HashMap<String, Vec<String>> {

  let mut connected_state_positions: HashMap<String, Vec<String>> = HashMap::new();

  for state_key in state_keys {

    let connected_state = state_positions
      .get(state_key)
      .expect("Could not retrieve the specified state");

    for (connection_character, states_connected_by_character) in &connected_state.states_connected_to {
      let binding = vec![];

      let previous_connections_by_character: HashSet<String> = HashSet::from_iter(connected_state_positions
        .get(connection_character)
        .unwrap_or_else(|| {
          return &binding;
        })
        .iter()
        .cloned());

      let states_connected_by_character: HashSet<String> = HashSet::from_iter(
        states_connected_by_character
        .iter()
        .cloned());

      let unionized_state_connections_by_character = previous_connections_by_character.to_owned()
        .union(&states_connected_by_character)
        .cloned()
        .collect();

      connected_state_positions.insert(connection_character.to_owned(), unionized_state_connections_by_character);
      
    };

  };

  return connected_state_positions.into();

}

fn does_vec_contain_final_state(state_positions: &HashMap<String, State>, state_keys: &Vec<String>) -> bool {
  for state_key in state_keys {

    let is_final_state = state_positions
      .get(state_key)
      .expect("There was a problem retrieving the state");

    if is_final_state.is_final {
      return true;
    }

  }
  return false;
}

fn create_unique_state_coordinates(state_positions: &HashMap<String, RefCell<State>>) -> Coordinate {
  
  let mut x_position = 300;
  let mut y_position = 300;

  let mut hashed_position = x_position.to_string() + "," + y_position.to_string().as_str();

  while state_positions.contains_key(&hashed_position) {

    if x_position < 800 {
      x_position += 200;
    } else {
      x_position = 100;
      y_position += 200;
    }

    hashed_position = x_position.to_string() + "," + y_position.to_string().as_str();

  };
  return Coordinate {
    x: x_position,
    y: y_position
  };
}

fn create_connections_from_state_positions(state_positions: &HashMap<String, RefCell<State>>) -> Vec<Connection> {

  let mut connections = vec![];

  // quite slow but okay as most graphs used will be quite parsed
  for (current_state_key, current_state) in state_positions {

    let current_state = current_state.borrow();

    for (connection_character, connected_state_keys) in &current_state.states_connected_to {
      for connected_state_key in connected_state_keys {
        let is_connected_to_self: bool = current_state_key == connected_state_key;

        let end_point: Coordinate = connected_state_key
          .try_into()
          .expect("Could not parse given key to coordinates");

        let new_bezier_curve = BezierCurve {
          start_point: 
          current_state.position,

          control_point_one: 
            if is_connected_to_self 
            { Coordinate {
              x: current_state.position.x - 200,
              y: current_state.position.y + 200

            } } else 
            {current_state.position},

          control_point_two:
            if is_connected_to_self 
            { Coordinate {
              x: end_point.x - 200,
              y: end_point.y - 200
            } } else 
            {end_point},
          
          end_point
        };

        let new_connection = Connection {
          connection_character: connection_character
            .to_owned(),
          curve: new_bezier_curve,
          element: String::from("Connection")
        };

        connections.push(new_connection)
      }

    }

  };

  return connections;

}