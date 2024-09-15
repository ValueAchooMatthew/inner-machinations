import { get } from "svelte/store";
import type { RegularAutomataConnection, State } from "../types/interfaces";
import { convertCoordinateToString, removeFirstElementFromArray } from "./miscUtils";
import { current_action, list_of_all_elements, list_of_regular_automata_connections, list_of_states, recently_removed_elements, start_state_index, start_state_position, state_positions } from "./regularAutomataStores";
import { Action } from "$lib/types/enums";

export const removeStateFromConnections = (given_state: State, state_to_remove: State, connection_character: string) => {

  let states_connected_by_character = given_state.states_connected_to.get(connection_character);
  if(!states_connected_by_character) {
    // If there are no connections by the given char, we just return the original state
    return given_state;
  }

  const updated_states_connected_by_character = removeFirstElementFromArray(states_connected_by_character, convertCoordinateToString(state_to_remove.position));
  given_state.states_connected_to.set(connection_character, updated_states_connected_by_character);

  state_positions.update((positions) => {
    positions.set(convertCoordinateToString(given_state.position), given_state);
    return positions;
  });

  list_of_states.update((states) => {
    states.forEach((state, index) => {
      if (state.position === given_state.position) {
        states[index] = given_state;
      }
    })
    return states;
  });
}

export function undo() {

  let removed_element: State | RegularAutomataConnection | undefined;
  
  list_of_all_elements.update((elements)=>{
    removed_element = elements.pop();
    return elements;
  });

  
  if (!removed_element) {
    // The removed element is undefined, nothing more needs to be done
    return;
  }
  
  recently_removed_elements.update((elements) => {
    // Type checking with ts doesn't seem to work properly with update calls
    // Thus additional unnecessary typecheck performed here
    if(removed_element) {
      elements.push(removed_element);
    }
    return elements;
  });

  if (removed_element.element === "State") {
    // The removed element is a state. The list of states we have stored must be updated and so must the state positions.
    // As well, if the removed state was a start state, information relating to start states must also be updated.
    if (removed_element.is_start) {
      start_state_index.set(null);
      start_state_position.set(null);
    }

    list_of_states.update((states) => {
      states.pop();
      return states;
    });

    const removed_state_coordinates_as_string = convertCoordinateToString(removed_element.position);

    state_positions.update((positions) => {
      // Will have to look into this later, but for some reason type checking fails to work properly
      // inside an update callback even though we have properly type checked that removed_element
      // must be of type state
      positions.delete(removed_state_coordinates_as_string);
      return positions;
    })
    return;
  }
  // The removed element must be a connection. The list of connections must be updated and so must the connections of the state
  // the connection is starting from
  list_of_regular_automata_connections.update((connections) => {
    connections.pop();
    return connections;
  });

  const connection_starting_state: State | undefined = get(state_positions).get(
    convertCoordinateToString(removed_element.curve.start_point),
  );

  const connection_ending_state: State | undefined = get(state_positions).get(
    convertCoordinateToString(removed_element.curve.end_point)
  );
  
  if (!connection_starting_state || !connection_ending_state) {
    return;
  }

  removeStateFromConnections(connection_starting_state, connection_ending_state, removed_element.connection_character);
}

export function redo() {

  let element_to_readd: State | RegularAutomataConnection | undefined;
  
  recently_removed_elements.update((elements) => {
    element_to_readd = elements.pop();
    return elements;
  });

  if(!element_to_readd) {
    return;
  }

  list_of_all_elements.update((elements) => {
    if(element_to_readd) {
      elements.push(element_to_readd);
    }
    return elements;
  });

  if(element_to_readd.element === "State") {

    const state_to_readd = element_to_readd;
    const state_to_readd_key = convertCoordinateToString(state_to_readd.position);

    // Handling checking if state is start state here for easier access to current value of list_of_states
    list_of_states.update((states) => {
      states.push(state_to_readd);
      if(state_to_readd.is_start) {
        start_state_index.set(states.length - 1);
        start_state_position.set(state_to_readd_key);
      }
      return states;
    });

    state_positions.update((state_positions) => {
      state_positions.set(state_to_readd_key, state_to_readd);
      return state_positions;
    });

  } else if(element_to_readd.element === "RegularAutomataConnection") {
    const connection = element_to_readd;
    const connection_character = connection.connection_character;
    const connection_starting_state_key = convertCoordinateToString(connection.curve.start_point);
    const connection_ending_state_key = convertCoordinateToString(connection.curve.end_point);

    list_of_regular_automata_connections.update((connections) => {
      connections.push(connection);
      return connections;
    });

    const connection_starting_state = get(state_positions).get(
      connection_starting_state_key
    );

    if(!connection_starting_state) {
      return;
    }

    let state_keys_connected_by_connection_character = connection_starting_state.states_connected_to.get(
      connection_character
    );
    
    if(!state_keys_connected_by_connection_character) {
      state_keys_connected_by_connection_character = new Array(connection_ending_state_key);
    } else {
      state_keys_connected_by_connection_character.push(connection_ending_state_key);
    }

    connection_starting_state.states_connected_to.set(
      connection_character, state_keys_connected_by_connection_character
    );

    state_positions.update((state_positions) => {
      state_positions.set(connection_starting_state_key, connection_starting_state);
      return state_positions;
    });

    list_of_states.update((states) => {
      states.map((state) => {
        if(state.position.x === connection.curve.start_point.x && state.position.y === connection.curve.start_point.y) {
          state.states_connected_to.set(
            connection_character,
            state_keys_connected_by_connection_character
          );
        }
      });
      return states;
    });
  }
}

export function handleTrash() {
  recently_removed_elements.set(
    get(list_of_all_elements).reverse()
  );
  state_positions.set(new Map());
  start_state_index.set(null);
  start_state_position.set(null);
  list_of_states.set(new Array());
  list_of_regular_automata_connections.set(new Array());
  list_of_all_elements.set(new Array());
  current_action.set(Action.CLICKING);
}
