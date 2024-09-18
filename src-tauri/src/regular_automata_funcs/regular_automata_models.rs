use std::collections::HashMap;
use crate::diesel::{QueryDsl, RunQueryDsl, ExpressionMethods, SqliteConnection};
use crate::{miscellaneous::{common_models::{BezierCurve, State}, 
database_models_and_utilities::{establish_connection, SavedConnection, SavedRegularAutomataWorkspace, SavedState, TypeOfAutomata}}, 
schema::{saved_regular_automata_connections, saved_states}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegularAutomatonConnection {
  pub curve: BezierCurve,
  pub connection_character: String,
  pub element: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegularAutomataWorkspaceData {
  start_state_index: Option<usize>,
  start_state_position: Option<String>,
  state_positions: HashMap<String, State>,
  list_of_states: Vec<State>,
  list_of_regular_automata_connections: Vec<RegularAutomatonConnection>,
  type_of_automata: TypeOfAutomata,
  date_of_last_update: String,
  alphabet: Vec<String>,
  should_strict_check: bool,
  should_show_string_traversal: bool,
  default_connection_character: String
}

impl RegularAutomataWorkspaceData {

  pub fn new(workspace: SavedRegularAutomataWorkspace) -> Self {

    let mut conn = establish_connection();

    let list_of_states = Self::get_list_of_states_from_saved_workspace(&workspace, &mut conn);
    let (start_state_index, start_state_position) = Self::get_start_state_information(&list_of_states);
    let list_of_regular_automata_connections = Self::get_list_of_regular_automata_connections_from_saved_workspace(&workspace, &mut conn);
    let state_positions = Self::get_state_positions_from_list_of_states(&list_of_states);
    let alphabet = Self::parse_alphabet(&workspace);

    let automata_type = workspace.type_of_automata;
    let date_of_last_update = workspace.date_of_last_update.format("%Y-%m-%d %H:%M:%S").to_string();
    let should_strict_check = workspace.should_strict_check;
    let should_show_string_traversal = workspace.should_show_string_traversal;
    let default_connection_character = workspace.default_connection_character;
    
    return RegularAutomataWorkspaceData {
      start_state_index,
      start_state_position,
      state_positions,
      list_of_states,
      list_of_regular_automata_connections,
      type_of_automata: automata_type,
      date_of_last_update,
      alphabet,
      should_strict_check,
      should_show_string_traversal,
      default_connection_character
    }

  }

  pub fn get_start_state_position(&self) -> &Option<String> {
    &self.start_state_position
  }

  pub fn get_state_positions(&self) -> &HashMap<String, State> {
    &self.state_positions
  }

  fn get_state_positions_from_list_of_states(list_of_states: &Vec<State>) -> HashMap<String, State> {

    let mut state_positions = HashMap::new();


    for state in list_of_states {

      let state_coords_as_string: String = state.get_position_as_string();

      state_positions.insert(state_coords_as_string.to_owned(), state.to_owned());

    }

    return state_positions;

  }

  fn get_list_of_states_from_saved_workspace(saved_workspace: &SavedRegularAutomataWorkspace, conn: &mut SqliteConnection) -> Vec<State> {
    // First get the states and connections from the database

    let mut list_of_states = Vec::new();

    let retrieved_states: Vec<SavedState> = saved_states::table
      .filter(saved_states::workspace_id.eq(&saved_workspace.id))
      .get_results::<SavedState>(conn)
      .expect("There was an issue getting the workspace's states");

    for retrieved_state in retrieved_states {
      let parsed_state = Self::parse_saved_state_to_regular_state(retrieved_state, saved_workspace, conn);
      list_of_states.push(parsed_state);
    }

    return list_of_states;

  }

  fn parse_saved_state_to_regular_state(state: SavedState, workspace: &SavedRegularAutomataWorkspace, conn: &mut SqliteConnection) -> State {
    
    let states_connected_to_given_state: Vec<SavedConnection> = saved_regular_automata_connections::table
      .filter(saved_regular_automata_connections::workspace_id.eq(&workspace.id))
      .filter(saved_regular_automata_connections::start_point.eq(&state.position))
      .get_results::<SavedConnection>(conn)
      .expect("There was an issue getting the workspace's states");

    let parsed_state_position = state.position.try_into()
      .expect("The string should be castable into Coordinate form");
  
    let mut parsed_state = State::new(parsed_state_position, state.is_start, state.is_final);
  
    for connected_state in states_connected_to_given_state {
      parsed_state.add_connection(&connected_state.connection_character, connected_state.end_point);
    }
    parsed_state
  }

  fn get_list_of_regular_automata_connections_from_saved_workspace(workspace: &SavedRegularAutomataWorkspace, conn: &mut SqliteConnection) -> Vec<RegularAutomatonConnection> {

    let mut list_of_regular_automata_connections = Vec::new();

    let retrieved_connections: Vec<SavedConnection> = saved_regular_automata_connections::table
      .filter(saved_regular_automata_connections::workspace_id.eq(&workspace.id))
      .get_results::<SavedConnection>(conn)
      .expect("There was an issue getting the workspace's states");

    for retrieved_connection in retrieved_connections {
      let parsed_connection = Self::parse_saved_connection_to_regular_connection(retrieved_connection);
      list_of_regular_automata_connections.push(parsed_connection);
    }

    list_of_regular_automata_connections

  }

  fn parse_saved_connection_to_regular_connection(connection: SavedConnection) -> RegularAutomatonConnection {

    let parsed_curve = BezierCurve {
      start_point: connection.start_point.try_into().expect("Could not parse string to coordinates"),
      control_point_one: connection.control_point_one.try_into().expect("Could not parse string to coordinates"),
      control_point_two: connection.control_point_two.try_into().expect("Could not parse string to coordinates"),
      end_point: connection.end_point.try_into().expect("Could not parse string to coordinates")
    };

    let parsed_connection = RegularAutomatonConnection {
      curve: parsed_curve,
      connection_character: connection.connection_character,
      element: String::from("RegularAutomatonConnection")
    };

    parsed_connection
  }
  
  // Return type corresponds to start state index and start state key respectively
  // Optional as saved workspace may not have start state
  fn get_start_state_information(list_of_states: &Vec<State>) -> (Option<usize>, Option<String>) {
    for (index, state_reference) in list_of_states.iter().enumerate() {
      if state_reference.is_start() {
        return (Some(index), Some(state_reference.get_position_as_string()));
      }
    }
    return (None, None);
  }

  fn parse_alphabet(saved_workspace: &SavedRegularAutomataWorkspace) -> Vec<String> {

    return saved_workspace.alphabet
      .split(',')
      .map(|s| s.to_string())
      .collect();
  }

}