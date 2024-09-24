use std::collections::{HashMap, HashSet};
use crate::regular_automata_funcs::regular_automata_models::{RegularAutomataTransition, RegularAutomataTransitions, RegularAutomatonConnection};
use super::common_models::{BezierCurve, Coordinate, State};

pub fn create_unique_state_coordinates(state_positions: &HashSet<String>) -> Coordinate {
  
  let mut x_position = 300;
  let mut y_position = 300;

  let mut hashed_position = x_position.to_string() + "," + y_position.to_string().as_str();

  while state_positions.contains(&hashed_position) {

    if x_position < 800 {
      x_position += 200;
    } else {
      x_position = 300;
      y_position += 200;
    }

    hashed_position = x_position.to_string() + "," + y_position.to_string().as_str();

  };
  return Coordinate {
    x: x_position,
    y: y_position
  };
}

pub fn create_connections_from_state_positions(state_positions: &HashMap<String, State>) -> Vec<RegularAutomatonConnection> {

  let mut connections = vec![];

  // quite slow but okay as most graphs used will be quite sparse
  for (current_state_key, current_state) in state_positions {

    for (connection_character, connected_state_keys) in current_state.get_all_connections() {
      for connected_state_key in connected_state_keys {
        let is_connected_to_self: bool = current_state_key == connected_state_key;

        let end_point: Coordinate = connected_state_key
          .try_into()
          .expect("Could not parse given key to coordinates");

        let current_position = current_state.get_position();

        let new_bezier_curve = BezierCurve {
          start_point: 
          current_position,

          control_point_one: 
            if is_connected_to_self { 
              Coordinate {
              x: current_position.x - 200,
              y: current_position.y + 200
              } 
            } else {
              current_position
            },

          control_point_two:
            if is_connected_to_self 
            { Coordinate {
              x: end_point.x - 200,
              y: end_point.y - 200
            } } else 
            {end_point},
          
          end_point
        };

        let new_connection = RegularAutomatonConnection {
          connection_character: connection_character
            .to_owned(),
          curve: new_bezier_curve,
          element: String::from("RegularAutomatonConnection")
        };

        connections.push(new_connection)
      }

    }

  };

  return connections;

}

pub fn remove_all_epsilon_transitions(state_positions: &mut HashMap<String, State>) {

  let mut make_final;
  let mut finished = false;

  while !finished {
    finished = true;
    let cloned_state_positions = state_positions.clone();
    for (_, state) in &mut *state_positions {

      make_final = false;

      let connections = state
        .get_all_connections_mut();

      if let Some(epsilon_state_keys) = connections.clone().get("ϵ") {
        if epsilon_state_keys.len() == 0 {
          connections.remove("ϵ");
          continue;
        }

        finished = false;
        for epsilon_state_key in epsilon_state_keys {
          let epsilon_state = cloned_state_positions
            .get(epsilon_state_key)
            .expect("Could not retrieve the requested state");

          if epsilon_state.is_final() {
            make_final = true;
          }

          let connections_from_epsilon_state = epsilon_state.get_all_connections();

          for (character, keys) in connections_from_epsilon_state {
            connections
              .entry(character.to_owned())
              .and_modify(|current_set| {
                for key in keys {
                  current_set.insert(key.to_owned());
                }
              })
              .or_insert(keys.to_owned());
          }
          connections
            .entry("ϵ".to_owned())
            .and_modify(|current_set| {
              current_set.remove(epsilon_state_key);
          });

          if make_final {
            state.make_final();
          }
          break;
        }
      }
    }
  }
}

// An input alphabet must consist of entirely unique characters and should be at most a single character long
// We want to preserve the order of the alphabet for ease of use thus hashsets are not an option, thus we will iterate
// Over everything and ensure it fits our requirements

pub fn sanitize_input_alphabet(alphabet: Vec<&str>) -> Vec<String> {
  let mut previously_seen_input_characters = HashSet::new();
  let sanitized_alphabet = alphabet.into_iter().filter(|input_character| {
    if !previously_seen_input_characters.contains(input_character) && input_character.len() == 1 {
      previously_seen_input_characters.insert(input_character.to_owned());
      return true;
    }
    return false;
  }).map(|input_characters| {
    return input_characters.to_owned();
  }).collect();

  return sanitized_alphabet;
}

pub fn create_state_positions_from_states(states: Vec<&State>) -> HashMap<String, State> {

  let mut state_positions = HashMap::new();

  for state in states {
    
    state_positions.insert(state.get_position_as_string(), state.to_owned());

  }
  state_positions
}

// Multiple loops can exist between the same states due to different connection characters
// We need to account for this
#[tauri::command(rename_all = "snake_case")]
pub fn find_all_loops_in_state_positions(state_positions: HashMap<String, State>) {

  let mut all_unique_loops = HashSet::new();
  let mut unique_loops_from_state_keys = HashMap::new();

  // We perform DFS on all states and add any path which leads back to the current state
  for (state_key, _) in &state_positions {
    // If we have already taken a specific transition before, we should not take it again
    let transitions_taken = Vec::new();
    find_loops(&state_key, &state_positions, state_key, transitions_taken, &mut all_unique_loops, &mut unique_loops_from_state_keys);
  }

  println!("{unique_loops_from_state_keys:?}");
}

fn find_loops(
  current_state_key: &str, 
  state_positions: &HashMap<String, State>, 
  state_key_to_check_for_loops: &str, 
  // Represents first state key, connection character, and final state key respectively
  mut transitions_taken: RegularAutomataTransitions,
  // All of our unique loops are stored here, to avoid having duplicate loops for every single state involved in a given loop
  // We know that for the same loops originating from two different states in that loop, the vectors will be some multiple of
  // left or right shifts away from each other. If that is the case then we do not want to add it to our hashmap
  all_unique_loops: &mut HashSet<RegularAutomataTransitions>,
  all_loops_to_given_state: &mut HashMap<String, HashSet<RegularAutomataTransitions>>
) {

  let current_state = state_positions
    .get(current_state_key)
    .expect("Could not retrieve the requested state");

  // Have to take at least one transition to qualify even if the transition is a self loop
  if current_state_key == state_key_to_check_for_loops && !transitions_taken.is_empty() {
    all_loops_to_given_state.entry(current_state_key.to_owned())
      .and_modify(|current_loops| {
        current_loops.insert(transitions_taken.to_owned());
      })
      .or_insert(HashSet::from([transitions_taken]));
  } else {
    for (connection_character, connected_state_keys) in current_state.get_all_connections() {
      for connected_state_key in connected_state_keys {
        let transition_to_take = RegularAutomataTransition::new(
          current_state_key,
          connection_character, 
          connected_state_key);
          
        if !transitions_taken.contains(&transition_to_take) {
          transitions_taken.push(transition_to_take);
          find_loops(
            connected_state_key,
            state_positions,
            state_key_to_check_for_loops,
            transitions_taken.to_owned(),
            all_unique_loops,
            all_loops_to_given_state
          );
        }
      }
    }
  }
}