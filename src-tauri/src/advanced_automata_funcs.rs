use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::automaton::{parse_input_alphabet, Automaton, Dfa};
use crate::create_unique_state_coordinates;
use crate::error::{CommandResult, Error};
use crate::models::{AutomatonData, ConnectionData, Coordinate, StateData};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MinimizedAutomaton {
  pub automaton: AutomatonData,
  // When it is, the frontend leaves the automaton as the user drew it
  pub was_already_minimal: bool
}

// DFA's are typically required to have a connection for each state for each character in the input alphabet.
// This however, is very unuser friendly as it is very easy to infer that an unspecified connection results in automatic unacceptance.
// So a missing connection counts as one to a dead state, and the minimized DFA leaves out the dead state and the connections
// into it. A DFA that specifies every connection keeps its dead state, if it has one, so it still passes strict checking.
// Nothing is saved; the frontend shows the result, and saves it like any other change
#[tauri::command]
pub fn minimize_dfa(automaton: AutomatonData, input_alphabet: Vec<String>) -> CommandResult<MinimizedAutomaton> {

  let (nfa, state_ids) = Automaton::from_data(&automaton);

  // Determinizing first means a DFA drawn with two connections on one character is minimized as the NFA it is
  let dfa = nfa
    .determinize(&parse_input_alphabet(&input_alphabet))
    .ok_or_else(|| Error::new("The DFA needs a start state to be minimized"))?;
  let minimized = dfa.minimize();

  // A DFA whose states are all reachable comes out of the subset construction with as many states as it went in,
  // and is minimal if minimizing then merges none of them
  let was_already_minimal = nfa.is_deterministic()
    && dfa.state_count() == nfa.state_count()
    && minimized.state_count() == dfa.state_count();

  Ok(MinimizedAutomaton {
    was_already_minimal,
    automaton: create_automaton_data_from_dfa(&minimized, &nfa, &state_ids, &automaton)
  })

}

// The subset construction. The resulting DFA specifies every connection, using a dead state where needed,
// so it passes strict checking. Nothing is saved
#[tauri::command]
pub fn convert_nfa_to_dfa(automaton: AutomatonData, input_alphabet: Vec<String>) -> CommandResult<AutomatonData> {

  let (nfa, state_ids) = Automaton::from_data(&automaton);

  let dfa = nfa
    .determinize(&parse_input_alphabet(&input_alphabet))
    .ok_or_else(|| Error::new("The NFA needs a start state to be converted"))?
    .complete();

  Ok(create_automaton_data_from_dfa(&dfa, &nfa, &state_ids, &automaton))

}

// Turns a DFA made from the automaton back into one the frontend can draw. Each state goes where one of the
// original states it stands for was, if that spot is still free, so a DFA that needed no changes keeps its
// layout; the start state goes where the original start state was. A connection the original had between the
// same two places, on the same character, keeps its curve
fn create_automaton_data_from_dfa(
  dfa: &Dfa,
  nfa: &Automaton,
  original_state_ids: &[i32],
  original: &AutomatonData
) -> AutomatonData {

  let original_positions: HashMap<i32, Coordinate> = original.states
    .iter()
    .map(|state| (state.id, state.position))
    .collect();

  let mut taken_positions: HashSet<Coordinate> = HashSet::new();
  let mut positions: Vec<Coordinate> = vec![];

  for state in 0..dfa.state_count() {

    let original_start_state = if state == 0 { nfa.start() } else { None };

    let free_original_position = original_start_state
      .into_iter()
      .chain(dfa.members(state).iter().copied())
      .map(|original_state| original_positions[&original_state_ids[original_state]])
      .find(|position| !taken_positions.contains(position));

    let position = free_original_position
      .unwrap_or_else(|| create_unique_state_coordinates(&taken_positions));

    taken_positions.insert(position);
    positions.push(position);
  }

  let states = (0..dfa.state_count())
    .map(|state| StateData {
      id: state as i32 + 1,
      position: positions[state],
      is_start: state == 0,
      is_final: dfa.is_accepting(state)
    })
    .collect();

  let mut connections: Vec<ConnectionData> = vec![];
  // How many connections join each pair of states so far, in either direction, so the next one can be drawn apart
  let mut connection_counts: HashMap<(usize, usize), usize> = HashMap::new();

  for state in 0..dfa.state_count() {
    for (character, target) in dfa.transitions_from(state) {

      let character = character.to_string();
      let (from, to) = (positions[state], positions[target]);

      let previous_control_points = original.connections
        .iter()
        .find(|connection| {
          connection.character == character
            && original_positions.get(&connection.from) == Some(&from)
            && original_positions.get(&connection.to) == Some(&to)
        })
        .map(|connection| connection.control_points);

      let connection_count = connection_counts.entry((state.min(target), state.max(target))).or_insert(0);
      let control_points = previous_control_points
        .unwrap_or_else(|| default_control_points(from, to, state == target, *connection_count));
      *connection_count += 1;

      connections.push(ConnectionData {
        id: connections.len() as i32 + 1,
        from: state as i32 + 1,
        to: target as i32 + 1,
        character,
        control_points
      });
    }
  }

  AutomatonData { states, connections }

}

// The first connection between two states is straight, and each one after it bends further away, alternating sides,
// so their characters are drawn apart. A state's first loop to itself goes to the left, as one the user draws does,
// and later loops go up, down and right, getting bigger after that. The frontend lays out connections the same way
pub fn default_control_points(from: Coordinate, to: Coordinate, is_loop: bool, index: usize) -> [Coordinate; 2] {

  if is_loop {
    let size = 200 + 100 * (index / 4) as i32;
    let ((first_x, first_y), (second_x, second_y)) = match index % 4 {
      0 => ((-size, size), (-size, -size)),
      1 => ((-size, -size), (size, -size)),
      2 => ((size, size), (-size, size)),
      _ => ((size, -size), (size, size))
    };
    return [
      Coordinate { x: from.x + first_x, y: from.y + first_y },
      Coordinate { x: to.x + second_x, y: to.y + second_y }
    ];
  }

  // The bend is measured from the same side whichever way the connection goes
  let (first_point, second_point) = if (from.x, from.y) <= (to.x, to.y) { (from, to) } else { (to, from) };
  let x_distance = (second_point.x - first_point.x) as f64;
  let y_distance = (second_point.y - first_point.y) as f64;
  let length = x_distance.hypot(y_distance);

  if index == 0 || length == 0.0 {
    return [from, to];
  }

  // 100 to one side, then 100 to the other, then 200 to the first side, and so on
  let bend = 100.0 * ((index + 1) / 2) as f64 * if index % 2 == 1 { 1.0 } else { -1.0 };
  let point_along_curve = |fraction: f64| Coordinate {
    x: (from.x as f64 + (to.x - from.x) as f64 * fraction - y_distance / length * bend).round() as i32,
    y: (from.y as f64 + (to.y - from.y) as f64 * fraction + x_distance / length * bend).round() as i32
  };

  [point_along_curve(1.0 / 3.0), point_along_curve(2.0 / 3.0)]

}

#[cfg(test)]
mod tests;
