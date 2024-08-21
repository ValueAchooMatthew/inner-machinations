import { get } from "svelte/store";
import { default_connection_character, input_alphabet, list_of_connections, list_of_states, should_show_string_traversal, should_strict_check, start_state_index, start_state_position, state_positions, type_of_automata } from "$lib/utils/automataStores";
import type { State, WorkspaceData } from "$lib/types/interfaces";
import { convertCoordinateToString } from "$lib/utils/miscUtils";
import { Automata } from "../types/enums";

export const parseListOfStates = (
  json_states: Array<State>,
  ): Array<State> => {
  
  const states = json_states;

  states.forEach((state) => {
    state.states_connected_to = new Map<string, Array<string>>(
      Object.entries(state.states_connected_to),
    );
  });

  return states;
}

export const parseStatePositions = (
  json_state_positions: { [key: string]: State }
): Map<string, State> => {

  // Needed as hashmaps get parsed into an object instead of a map when coming from backend
  const state_positions = new Map<string, State>(
    Object.entries(json_state_positions),
  );
  
  state_positions.forEach((state) => {
    state.states_connected_to = new Map<string, Array<string>>(
      Object.entries(state.states_connected_to),
    );
  });

  return state_positions;
}

export const setTauriResponses = (tauri_response: WorkspaceData): void => {

  start_state_index.set(
    tauri_response.start_state_index
  );

  start_state_position.set(
    tauri_response.start_state_position
  );

  list_of_states.set(
    parseListOfStates(tauri_response.list_of_states)
  );

  list_of_connections.set(
    tauri_response.list_of_connections
  );

  state_positions.set(
    parseStatePositions(tauri_response.state_positions)
  );

  const enum_representation_of_response = tauri_response.type_of_automata == "DFA"? Automata.DFA:Automata.NFA
  type_of_automata.set(
    enum_representation_of_response
  );

  input_alphabet.set(
    tauri_response.alphabet
  );

  should_strict_check.set(
    tauri_response.should_strict_check
  );

  should_show_string_traversal.set(
    tauri_response.should_show_string_traversal
  );

  default_connection_character.set(
    tauri_response.default_connection_character
  );


}