import { writable, type Writable } from "svelte/store";
import type { Connection, State } from "./interfaces";
import { Action, Automata } from "./enums";

export const start_state_index: Writable<number | null> = writable(null);
export const start_state_position: Writable<string | null> = writable(null);

// hashing every coordinate to a state for use when user click on a given coordinate point
// Allows for O(1) access without having to search for the state which was clicked in the State array
export const state_positions: Writable<Map<string, State>> = writable(new Map());

// Used to function as a "stack" so the most recently added states or connections can be removed by the user
export const list_of_all_elements: Writable<Array<State | Connection>> = writable(new Array());

// Serves a similar function to list of all elements except for undoing
export const recently_removed_elements: Writable<Array<State | Connection>> = writable(new Array());

export const list_of_states: Writable<Array<State>> = writable(new Array());
export const list_of_connections: Writable<Array<Connection>> = writable(new Array());
export const input_alphabet: Writable<Array<string>> = writable(new Array("a", "b"));
export const type_of_automata: Writable<Automata> = writable(Automata.DFA);
export const dialogue_to_user: Writable<string | null> = writable(null);
export const current_action: Writable<Action> = writable(Action.ADDING_REGULAR_STATE);
export const selected_connection_index: Writable<number | null> = writable(null);