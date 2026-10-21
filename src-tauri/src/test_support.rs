// Builds automata for tests. A state is identified by its x position, which is also its id, and sits at
// y = 100. Connections get ids in the order they're listed, and straight curves

use crate::models::{AutomatonData, ConnectionData, Coordinate, StateData};

pub struct TestState {
  x: i32,
  is_start: bool,
  is_final: bool,
  connections: Vec<(String, i32)>
}

// A state, with the character and target x position of each connection from it
pub fn state(x: i32, is_start: bool, is_final: bool, connections: &[(&str, i32)]) -> TestState {
  TestState {
    x,
    is_start,
    is_final,
    connections: connections.iter().map(|(character, target)| (character.to_string(), *target)).collect()
  }
}

pub fn automaton(states: impl IntoIterator<Item = TestState>) -> AutomatonData {
  let states: Vec<TestState> = states.into_iter().collect();
  let mut connections = vec![];

  for state in &states {
    for (character, target) in &state.connections {
      connections.push(ConnectionData {
        id: connections.len() as i32 + 1,
        from: state.x,
        to: *target,
        character: character.clone(),
        control_points: [Coordinate { x: state.x, y: 100 }, Coordinate { x: *target, y: 100 }]
      });
    }
  }

  AutomatonData {
    states: states
      .iter()
      .map(|state| StateData {
        id: state.x,
        position: Coordinate { x: state.x, y: 100 },
        is_start: state.is_start,
        is_final: state.is_final
      })
      .collect(),
    connections
  }
}

// Each transition as "from -character-> to", by state id, sorted
pub fn transitions(automaton: &AutomatonData) -> Vec<String> {
  let mut transitions: Vec<String> = automaton
    .connections
    .iter()
    .map(|connection| format!("{} -{}-> {}", connection.from, connection.character, connection.to))
    .collect();
  transitions.sort();
  transitions
}
