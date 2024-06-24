import { get } from "svelte/store";
import { list_of_connections, list_of_states, start_state_index, start_state_position, state_positions } from "./automataStores";
import type { Connection, State } from "./interfaces";
import { convertCoordinateToString } from "./miscUtils";

export const parseListOfStates = (
  json_states: Array<State>,
  ):  Array<State> => {
  
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

export const setTauriResponses = (tauri_response: [
  number | null,
  Array<State>,
  Array<Connection>,
  { [key: string]: State }]): void => {

  start_state_index.set(
    tauri_response[0]
  );

  list_of_states.set(
    parseListOfStates(tauri_response[1])
  );

  list_of_connections.set(
    tauri_response[2]
  );

  state_positions.set(
    parseStatePositions(tauri_response[3])
  );

  // Ugly code but the best way to handle this since outside of .svelte files i cannot use the $ syntactic sugar
  const ss_index = get(start_state_index);
  start_state_position.set(
    ss_index !== null? convertCoordinateToString(get(list_of_states)[ss_index].position): 
    null
  );

}