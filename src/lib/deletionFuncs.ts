import { get } from "svelte/store";
import { list_of_all_elements, list_of_connections, list_of_states, start_state_index, start_state_position, state_positions } from "./automataStores";
import type { Connection, State } from "./interfaces";
import { convertCoordinateToString, removeFirstElementFromArray } from "./miscUtils";

export const removeStateFromConnections = (given_state: State, state_to_remove: State, connection_character: string): State => {

  let states_connected_by_char = given_state.states_connected_to.get(connection_character);
  if(!states_connected_by_char) {
    // If there are no connections by the given char, we just return the original state
    return given_state;
  }

  states_connected_by_char = removeFirstElementFromArray(states_connected_by_char, 
    convertCoordinateToString(state_to_remove.position));

  given_state.states_connected_to.set(connection_character, states_connected_by_char);

  state_positions.update((positions)=>{
    positions.set(convertCoordinateToString(given_state.position), given_state);
    return positions;
  });

  list_of_states.update((states)=> {
    return removeFirstElementFromArray(states, given_state);
  });

  list_of_states.update((states)=>{
    states.forEach((state, index) => {
      if (state.position === given_state.position) {
        states[index] = given_state;
      }
    })
    return states;
  });

  return given_state;

}

export const undo = (): void => {

  let removed_element: State | Connection | undefined;
  
  list_of_all_elements.update((elements)=>{
    removed_element = elements.pop();
    return elements;
  })

  if (!removed_element) {
    // The removed element is undefined, nothing more needs to be done
    return;
  }

  if (removed_element.element === "State") {
    // The removed element is a state. The list of states we have stored must be updated and so must the state positions.
    // As well, if the removed state was a start state, information relating to start states must also be updated.
    if (get(list_of_states).length === get(start_state_index)) {
      start_state_index.set(null);
      start_state_position.set(null);
    }

    list_of_states.update((states)=>{
      return states.slice(0, states.length - 1);
    })

    state_positions.update((positions)=>{
      // Will have to look into this later, but for some reason type checking fails to work properly
      // inside an update callback even though we have properly type checked that removed_element
      // must be of type state
      positions.delete(convertCoordinateToString((removed_element as State).position))
      return positions;
    })
    return;
  }
  // The removed element must be a connection. The list of connections must be updated and so must the connections of the state
  // the connection is starting from
  list_of_connections.update((connections) => {
    return connections.slice(0, connections.length - 1);
  });

  const starting_state: State | undefined = get(state_positions).get(
    convertCoordinateToString(removed_element.curve.start_point),
  );

  const ending_state: State | undefined = get(state_positions).get(
    convertCoordinateToString(removed_element.curve.end_point)
  );
  
  if (!starting_state || !ending_state) {
    return;
  }

  removeStateFromConnections(starting_state, ending_state, removed_element.connection_character);
};

