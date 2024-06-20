import type { State } from "./interfaces";

export const parseTauriResponseToTSTypes = (
  tauri_states: Array<State>,
  tauri_state_connections: { [key: string]: State }
  ):  [Array<State>, Map<string, State>] => {
  
  const states = tauri_states;

  // Needed as hashmaps get parsed into an object instead of a map when coming from backend
  const state_connections = new Map<string, State>(
      Object.entries(tauri_state_connections),
  );

  states.forEach((state) => {
    state.states_connected_to = new Map<string, Array<string>>(
      Object.entries(state.states_connected_to),
    );
  });

  state_connections.forEach((state) => {
    state.states_connected_to = new Map<string, Array<string>>(
      Object.entries(state.states_connected_to),
    );
  });

  return [states, state_connections];

}