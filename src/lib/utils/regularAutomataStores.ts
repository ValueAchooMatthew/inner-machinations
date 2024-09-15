import { Action, Automata } from "$lib/types/enums";
import type { RegularAutomataConnection, State } from "$lib/types/interfaces";
import { writable, type Writable } from "svelte/store";


export const start_state_index: Writable<number | null> = writable(null);
export const start_state_position: Writable<string | null> = writable(null);

// hashing every coordinate to a state for use when user click on a given coordinate point
// Allows for O(1) access without having to search for the state which was clicked in the State array
export const state_positions: Writable<Map<string, State>> = writable(new Map());

// Used to function as a "stack" so the most recently added states or connections can be removed by the user
// Need to reimplement
export const list_of_all_elements: Writable<Array<State | RegularAutomataConnection>> = writable(new Array());

// Serves a similar function to list of all elements except for undoing
// Need to implement later
export const recently_removed_elements: Writable<Array<State | RegularAutomataConnection>> = writable(new Array());

export const current_action: Writable<Action> = writable(Action.ADDING_REGULAR_STATE);
export const list_of_states: Writable<Array<State>> = writable(new Array());
export const list_of_regular_automata_connections: Writable<Array<RegularAutomataConnection>> = writable(new Array());
export const input_alphabet: Writable<Array<string>> = writable(new Array("a", "b"));
export const type_of_automata: Writable<Automata> = writable(Automata.DFA);
export const selected_connection_index: Writable<number | null> = writable(null);
export const should_strict_check: Writable<boolean> = writable(false);
export const should_show_string_traversal: Writable<boolean> = writable(false);
export const default_connection_character: Writable<string> = writable("a");