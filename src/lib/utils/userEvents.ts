import { get } from "svelte/store";
import { current_action, dialogue_to_user, list_of_all_elements, list_of_connections, list_of_states, selected_connection_index, start_state_index, start_state_position, state_positions } from "./automataStores";
import type { BezierCurve, Connection, Coordinate, State } from "../types/interfaces";
import { convertCoordinateToString } from "./miscUtils";
import { Action } from "../types/enums";

export const handleUserClickingCanvas = (cursor_x_pos: number, cursor_y_pos: number, default_connection_char: string) => {
  dialogue_to_user.set(null);
  const cursor_coords: Coordinate = { x: cursor_x_pos, y: cursor_y_pos };
  const cursor_coords_as_string: string = convertCoordinateToString(cursor_coords);
  const selected_state: State | undefined = get(state_positions).get(cursor_coords_as_string);
  const current_user_action = get(current_action);

  if(current_user_action === Action.ADDING_REGULAR_STATE) {
    if(selected_state !== undefined){
      dialogue_to_user.set("You cannot place a Node on top of another Node.")
      return;
    }
    addState(cursor_coords, cursor_coords_as_string, false);
  }else if(current_user_action === Action.ADDING_START_STATE){
    if(selected_state !== undefined) {
      dialogue_to_user.set("You cannot place a Node on top of another Node.")
      return;
    }
    addState(cursor_coords, cursor_coords_as_string, true);
    current_action.set(Action.ADDING_REGULAR_STATE);
  }else if(current_user_action === Action.ADDING_FINAL_STATE) {
    if(selected_state === undefined) {
      dialogue_to_user.set("You must make an existing Node a final Node.");
      return;
    }else if(selected_state.is_final) {
      dialogue_to_user.set("The Node is already a final Node.");
      return;
    }
    selected_state.is_final = true;
    make_final_state(selected_state, cursor_x_pos, cursor_y_pos, cursor_coords_as_string);
  } else if(current_user_action === Action.PLACING_START_OF_LINE) {
    if(selected_state === undefined) {
      dialogue_to_user.set("You must place an arrow on top of a Node.");
      return;
    }
    addConnection(cursor_coords, default_connection_char);
  } else if(current_user_action === Action.PLACING_START_OF_EPSILON_LINE) {
    if(selected_state === undefined) {
      dialogue_to_user.set("You must place an arrow on top of a Node.");
      return;
    }
    addConnection(cursor_coords, "ϵ");
  } else if(current_user_action === Action.PLACING_END_OF_LINE) {
    if(selected_state === undefined) {
      dialogue_to_user.set("The arrow must point to a valid Node.");
      return;
    }

    let connection: Connection | undefined;

    list_of_connections.update((connections) => {
      connection = connections.pop();
      return connections;
    })

    if(connection === undefined) {
      return;
    }

    placeEndOfLine(connection, selected_state, cursor_coords, cursor_coords_as_string);
    current_action.set(Action.PLACING_START_OF_LINE);
    
  } else if(current_user_action === Action.DRAGGING_LINE) {
    // If a user clicks when they are engaged in dragging, it's interpreted as the user trying to stop dragging a connection
    // And thus the current action is switched to clicking and the dragged line is updated
    selected_connection_index.set(null);
    current_action.set(Action.CLICKING);
  }else {
    // If a user is "just" clicking, no action should take place
    return;
  }

}

const placeEndOfLine = (connection: Connection, selected_state: State, cursor_coords: Coordinate, cursor_coords_as_string: string) => {

  connection.curve.end_point = selected_state.position;
  if(
    selected_state.position.x === connection.curve.start_point.x
    && selected_state.position.y === connection.curve.start_point.y
  ) {
    connection.curve.control_point_one = {
      x: cursor_coords.x - 200,
      y: cursor_coords.y + 200,
    };
    connection.curve.control_point_two = {
      x: cursor_coords.x - 200,
      y: cursor_coords.y - 200,
    };
  }else {
    connection.curve.control_point_two = selected_state.position;
  }

  const starting_state_of_connection: State | undefined = get(state_positions)
    .get(convertCoordinateToString(connection.curve.start_point));
  
  if(starting_state_of_connection === undefined) {
    return;
  }

  const connection_character = connection.connection_character;

  let states_connected_to_start_state_by_character = starting_state_of_connection
    .states_connected_to
    .get(connection_character);

  if(states_connected_to_start_state_by_character === undefined){
    states_connected_to_start_state_by_character = new Array();
  }

  states_connected_to_start_state_by_character.push(cursor_coords_as_string);
  starting_state_of_connection.states_connected_to.set(
    connection_character,
    states_connected_to_start_state_by_character
  )

  list_of_connections.update((connections)=> {
    // Once again for some reason the typing of connection isn't accurate hence the use of 'as'
    connections.push(connection);
    return connections;
  });

  list_of_states.update((states)=>{
    states.push(starting_state_of_connection);
    return states;
  });

  state_positions.update((positions)=>{
    positions.set(
      convertCoordinateToString(starting_state_of_connection.position),
      starting_state_of_connection
    )
    return positions;
  });

}

const addConnection = (cursor_coords: Coordinate, connection_char: string) => {

  const curve: BezierCurve = {
    start_point: cursor_coords,
    control_point_one: cursor_coords,
    control_point_two: cursor_coords,
    end_point: cursor_coords,
  };

  const connection: Connection = {
    curve: curve,
    element: "Connection",
    connection_character: connection_char,
  };

  list_of_connections.update((connections)=>{
    connections.push(connection);
    return connections;
  });
  current_action.set(Action.PLACING_END_OF_LINE);

}

const addState = (cursor_coords: Coordinate, cursor_coords_as_string: string, make_start: boolean) => {

  const new_state: State = {
    position: cursor_coords,
    states_connected_to: new Map<string, Array<String>>(),
    is_start: make_start,
    is_final: false,
    element: "State",
  };

  if(make_start) {
    update_start_state_information(new_state);
  }

  list_of_all_elements.update((elements)=>{
    elements.push(new_state);
    return elements;
  });

  list_of_states.update((states)=>{
    states.push(new_state);
    return states;
  });

  state_positions.update((positions)=>{
    positions.set(cursor_coords_as_string, new_state);
    return positions;
  });

}

const update_start_state_information = (new_start_state: State): void => {

  const previous_start_state_index = get(start_state_index);
  if(previous_start_state_index !== null) {
    list_of_states.update((states)=>{
      const previous_start_state = states[previous_start_state_index];
      previous_start_state.is_start = false;
      states[previous_start_state_index] = previous_start_state;
      return states;
    });
  }
  start_state_index.set(get(list_of_states).length - 1);
  start_state_position.set(convertCoordinateToString(new_start_state.position));

}

const make_final_state = (state_to_update: State, cursor_x_pos: number, cursor_y_pos: number, cursor_coords_as_string: string) => {

  list_of_states.update((states)=>{
    states.forEach((state, index)=> {
      if(state.position.x === cursor_x_pos && state.position.y === cursor_y_pos) {
        states[index] =  state_to_update
      }
    });
    return states;
  });

  state_positions.update((positions)=>{
    positions.set(cursor_coords_as_string, state_to_update);
    return positions;
  });

}
