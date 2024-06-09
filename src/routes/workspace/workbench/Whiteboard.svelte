<script lang="ts">
  import { onMount } from "svelte";
  import { draw } from "$lib/drawingFuncs";
  import {
    roundToNearest,
    getClosestPointIndex,
    indexOfClosestBezierCurveToPoint,
  } from "$lib/mathFuncs";
  import type {
    State,
    Connection,
    Coordinate,
    BezierCurve,
  } from "$lib/interfaces";
  import { Action, Automata } from "$lib/enums";
  import Sidebar from "./Sidebar.svelte";
  import TestFeedback from "./TestFeedback.svelte";
  import { invoke } from "@tauri-apps/api";
  import { convertCoordinateToString } from "$lib/miscUtils";

  export let start_state_coordinates: string | null;
  export let state_connections: Map<string, State>;
  export let connections: Array<Connection> = [];
  export let dialogue: string;
  export let start_state_index: number | null;
  export let default_connection_char: string = "a";
  export let is_string_accepted: boolean | null;
  export let workspace_name: string | undefined;
  export let email: string | undefined;

  $: {
    if (context) {
      draw(
        context,
        width,
        height,
        states,
        connections,
        selected_connection_index,
        1,
      );
    }
  }

  // DO NOT CHANGE ANY CODE IN FORM FOO = [...FOO, BAR]
  // Necessary to trigger sveltekit rerender of dynamic variables and draw to screen

  // Necessary for removing ghost image when dragging canvas
  const img = new Image();
  img.src =
    "data:image/gif;base64,R0lGODlhAQABAIAAAAUEBAAAACwAAAAAAQABAAACAkQBADs=";

  // Consider refactoring into rust backend for faster performance in future
  let states: Array<State> = [];
  let elements: Array<State | Connection> = [];

  $: width = 900;
  $: height = 900;

  let canvas: HTMLCanvasElement | null;
  let selected_connection_index: number | null = null;
  let control_point_index: number | null = null;
  let context: CanvasRenderingContext2D;
  let current_action: Action = Action.ADDING_REGULAR_STATE;

  const saveWorkspace = async () => {
    if (!email || !workspace_name) {
      return;
    }
    if (workspace_name === "Untitled Project") {
      dialogue = "You must change the name of the project to save.";
      return;
    }
    dialogue = "";

    await invoke("save_workspace", {
      states: state_connections,
      workspaceName: workspace_name,
      email: email,
      connections: connections,
    });
  };

  onMount(async () => {
    width = window.screen.availWidth - 200;
    height = window.screen.height - 300;
    const ctx = canvas?.getContext("2d");
    if (!ctx) {
      return;
    }
    context = ctx;
    context.strokeStyle = "black";
    context.imageSmoothingQuality = "high";

    // For retrieving workspace
    if (!email || !workspace_name) {
      return;
    }
    const retrieved_data: [
      number | null,
      Array<State>,
      Array<Connection>,
      { [key: string]: State },
    ] = await invoke("retrieve_workspace_data", {
      email: email,
      workspaceName: workspace_name,
    });

    states = retrieved_data[1];
    connections = retrieved_data[2];
    // If the some value exists, there is a specified start state
    if (retrieved_data[0] !== null) {
      start_state_index = retrieved_data[0];
      start_state_coordinates = convertCoordinateToString(
        states[start_state_index].position,
      );
    }

    // Needed as hashmaps get parsed into an object instead of a map when coming from backend
    state_connections = new Map<string, State>(
      Object.entries(retrieved_data[3]),
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
  });

  const undo = (): void => {
    const element: State | Connection | undefined = elements.pop();
    if (!element) {
      return;
    }
    if (element.element === "State") {
      const state = states.pop();
      if (!state) {
        return;
      }
      if (states.length === start_state_index) {
        start_state_index = null;
        start_state_coordinates = null;
      }
      state_connections.delete(convertCoordinateToString(element.position));
      states = states;
      return;
    }

    connections.pop();
    const node_one_coords = element.curve.start_point;

    const node_one: State | undefined = state_connections.get(
      convertCoordinateToString(node_one_coords),
    );
    if (!node_one) {
      return;
    }
    let connected_states = node_one.states_connected_to.get(
      element.connection_character,
    );
    const end_state_coords = element.curve.end_point;
    if (connected_states === undefined) {
      return;
    }
    const index = connected_states.indexOf(
      convertCoordinateToString(end_state_coords),
    );
    connected_states.splice(index, 1);
    node_one.states_connected_to.set(
      element.connection_character,
      connected_states,
    );
    state_connections.set(
      convertCoordinateToString(end_state_coords),
      node_one,
    );

    states = states;
  };

  const handleTrash = () => {
    states = [];
    connections = [];
    elements = [];
    start_state_index = null;
    state_connections.clear();
    start_state_coordinates = null;
    current_action = Action.CLICKING;
  };

  const handleClick = (event: MouseEvent): void => {
    const cursor_x_pos = roundToNearest(event.offsetX, 100);
    const cursor_y_pos = roundToNearest(event.offsetY, 100);
    const cursor_coords: Coordinate = { x: cursor_x_pos, y: cursor_y_pos };
    const cursor_coords_string: string =
      convertCoordinateToString(cursor_coords);
    let selected_state: State | undefined =
      state_connections.get(cursor_coords_string);
    dialogue = "";
    switch (current_action) {
      case Action.ADDING_REGULAR_STATE:
        if (selected_state) {
          dialogue = "You cannot place a Node on top of another Node.";
          return;
        }
        selected_state = {
          position: cursor_coords,
          states_connected_to: new Map<string, Array<String>>(),
          is_start: false,
          is_final: false,
          element: "State",
        };
        elements.push(selected_state);
        states.push(selected_state);
        state_connections.set(cursor_coords_string, selected_state);
        break;

      case Action.ADDING_FINAL_STATE:
        if (!selected_state) {
          dialogue = "You must make an existing Node a final Node.";
          return;
        } else if (selected_state.is_final) {
          dialogue = "The Node is already a final Node.";
          return;
        }
        selected_state.is_final = true;
        states.forEach((state) => {
          if (
            state.position.x === cursor_x_pos &&
            state.position.y === cursor_y_pos
          ) {
            state.is_final = true;
          }
        });
        state_connections.set(cursor_coords_string, selected_state);
        break;

      case Action.ADDING_START_STATE:
        if (selected_state) {
          dialogue = "You cannot place a Node on top of another Node.";
          return;
        }
        start_state_coordinates = cursor_coords_string;
        // Only one start state is allowed, thus when the start state coordinates change, the old one becomes a regular state

        if (start_state_index !== null) {
          const old_start_state = states[start_state_index];
          old_start_state.is_start = false;
          states[start_state_index] = old_start_state;
        }

        selected_state = {
          position: cursor_coords,
          states_connected_to: new Map<string, Array<String>>(),
          is_start: true,
          is_final: false,
          element: "State",
        };
        elements.push(selected_state);
        states.push(selected_state);
        start_state_index = states.length - 1;
        state_connections.set(cursor_coords_string, selected_state);
        current_action = Action.ADDING_REGULAR_STATE;
        break;

      case Action.PLACING_LINE:
        if (!selected_state) {
          dialogue = "You must place an arrow on top of another Node.";
          return;
        }
        const curve: BezierCurve = {
          start_point: cursor_coords,
          control_point_one: cursor_coords,
          control_point_two: cursor_coords,
          end_point: cursor_coords,
        };

        const connection: Connection = {
          curve: curve,
          element: "Connection",
          connection_character: default_connection_char,
        };
        connections.push(connection);
        current_action = Action.DRAWING_LINE;
        break;

      case Action.PLACING_EPSILON_LINE:
        if (!selected_state) {
          dialogue = "You must place an arrow on top of another Node.";
          return;
        }
        const ep_curve: BezierCurve = {
          start_point: cursor_coords,
          control_point_one: cursor_coords,
          control_point_two: cursor_coords,
          end_point: cursor_coords,
        };

        const ep_connection: Connection = {
          curve: ep_curve,
          element: "Connection",
          connection_character: "ϵ",
        };
        connections.push(ep_connection);
        current_action = Action.DRAWING_LINE;
        break;

      case Action.DRAWING_LINE:
        if (!selected_state) {
          dialogue = "The arrow must point to a valid Node.";
          return;
        }
        const last_connection = connections.pop();
        if (!last_connection) {
          return;
        }
        // Starting node's key will be at the x, y coordinates of the connection's start point
        // The selected node will treated as our "ending" node
        const starting_state_key = last_connection.curve.start_point;
        const starting_state = state_connections.get(
          convertCoordinateToString(starting_state_key),
        );
        if (!starting_state) {
          return;
        }
        const previous_connections = starting_state.states_connected_to.get(
          last_connection.connection_character,
        );
        if (previous_connections === undefined) {
          starting_state.states_connected_to.set(
            last_connection.connection_character,
            new Array<String>(cursor_coords_string),
          );
        } else {
          previous_connections.push(cursor_coords_string);
          starting_state.states_connected_to.set(
            last_connection.connection_character,
            previous_connections,
          );
        }

        // First control point starts at the start coordinate, the second control point moves to follow the end coordinates
        // Makes drawing for user easier if control points are spread apart
        last_connection.curve.end_point = selected_state.position;
        last_connection.curve.control_point_two = selected_state.position;
        if (selected_state === starting_state) {
          // If the connection is supposed to be a loop, the control points are automatically changed so it doesn't look like
          // A single point when drawn and instead forms a circle like shape
          last_connection.curve.control_point_one = {
            x: cursor_coords.x - 200,
            y: cursor_coords.y + 200,
          };
          last_connection.curve.control_point_two = {
            x: cursor_coords.x - 200,
            y: cursor_coords.y - 200,
          };
        }
        connections.push(last_connection);
        elements.push(last_connection);
        current_action = Action.CLICKING;
        break;

      case Action.DRAGGING_LINE:
        selected_connection_index = null;
        current_action = Action.CLICKING;
        break;

      default:
        return;
    }
    states = states;
    state_connections = state_connections;
  };

  // Decent start
  // Try and draw without redrawing whole canvas
  const handleMove = (event: MouseEvent) => {
    switch (current_action) {
      case Action.DRAWING_LINE:
        const cursor_x_pos = roundToNearest(event.offsetX, 20);
        const cursor_y_pos = roundToNearest(event.offsetY, 20);
        const cursor_coords: Coordinate = { x: cursor_x_pos, y: cursor_y_pos };
        const connection = connections.pop();
        if (connection === undefined) {
          return;
        }
        connection.curve.end_point = cursor_coords;
        connections = [...connections, connection];
        break;

      default:
        break;
    }
  };

  const handleUndoEvent = async (event: KeyboardEvent): Promise<void> => {
    if (event.ctrlKey === true && event.key === "z") {
      undo();
    } else if (event.ctrlKey === true && event.key === "s") {
      saveWorkspace();
    }
  };

  const clearCursor = (): void => {
    current_action = Action.CLICKING;
    selected_connection_index = null;
  };

  // Used when an arrow is selected and the character of its transition is being changed by the user
  const handleCharChange = (event: KeyboardEvent): void => {
    if (
      selected_connection_index == null ||
      event.ctrlKey ||
      event.altKey ||
      event.shiftKey ||
      event.key == "Tab"
    ) {
      return;
    }

    const selected_connection = connections[selected_connection_index];
    const old_character = selected_connection.connection_character;
    const new_character = event.key;
    selected_connection.connection_character = new_character;

    const start_state_key = convertCoordinateToString(
      selected_connection.curve.start_point,
    );
    const end_state_key = convertCoordinateToString(
      selected_connection.curve.end_point,
    );
    const start_state: State | undefined =
      state_connections.get(start_state_key);
    const end_state: State | undefined = state_connections.get(end_state_key);

    if (!start_state || !end_state) {
      return;
    }
    let previous_state_connection =
      start_state.states_connected_to.get(old_character);
    let new_state_connection =
      start_state.states_connected_to.get(new_character);
    if (previous_state_connection === undefined) {
      return;
    }
    // Removing the end state from the old connection character's hashmap
    const index = previous_state_connection.indexOf(end_state_key);
    previous_state_connection = previous_state_connection.splice(index, 1);

    // Adding end state to new connection character's hashmap
    if (new_state_connection === undefined) {
      start_state.states_connected_to.set(
        new_character,
        new Array<String>(end_state_key),
      );
    } else {
      new_state_connection.push(end_state_key);
    }

    state_connections = state_connections;
    selected_connection_index = null;
    current_action = Action.CLICKING;
  };

  const handleDragStart = (event: MouseEvent): void => {
    if (selected_connection_index === null) {
      return;
    }
    current_action = Action.DRAGGING_LINE;
    const cursor_coordinates = { x: event.offsetX, y: event.offsetY };
    const curve: BezierCurve = connections[selected_connection_index].curve;
    const control_points: Array<Coordinate> = [
      curve.control_point_one,
      curve.control_point_two,
    ];
    const index_of_closest_control_point = getClosestPointIndex(
      control_points,
      cursor_coordinates,
    );
    control_point_index = index_of_closest_control_point;
  };

  const handleDrag = (event: MouseEvent) => {
    if (
      current_action !== Action.DRAGGING_LINE ||
      selected_connection_index === null ||
      control_point_index === null
    ) {
      return;
    }
    const connection: Connection = connections[selected_connection_index];
    const curve: BezierCurve = connection.curve;
    const cursor_coords: Coordinate = { x: event.offsetX, y: event.offsetY };
    if (control_point_index === 0) {
      // First control point is closest
      curve.control_point_one = cursor_coords;
    } else if (control_point_index === 1) {
      // Second control point is closest
      curve.control_point_two = cursor_coords;
    } else {
      // unreachable
      return;
    }
    connection.curve = curve;
    connections[selected_connection_index] = connection;
  };

  const handleDragEnd = (): void => {
    if (selected_connection_index === null) {
      return;
    }
    current_action = Action.DRAGGING_LINE;
  };
</script>

<svelte:window
  on:keydown={handleUndoEvent}
  on:mousedown={handleDragStart}
  on:mouseup={handleDragEnd}
  on:mousemove={handleDrag}
/>
<div class="w-full h-fit font-semibold flex align-middle justify-around">
  <!-- Setting tabindex is necessary so element is focusable and can thus listen to keydown events -->
  <!-- svelte-ignore a11y-positive-tabindex -->
  <canvas
    tabindex="1"
    draggable="false"
    class="border-black border-2 rounded-md mx-2 my-2 bg-white mr-0 flex-shrink-0"
    style={`width: ${width}px; height: ${height}px;`}
    {width}
    {height}
    bind:this={canvas}
    on:mousemove={handleMove}
    on:click={handleClick}
    on:dblclick={(event) => {
      clearCursor();
      selected_connection_index = indexOfClosestBezierCurveToPoint(
        { x: event.offsetX, y: event.offsetY },
        connections,
      );
    }}
    on:keyup={handleCharChange}
    on:mousedown={handleDragStart}
    on:mouseup={handleDragEnd}
  >
  </canvas>
  <div class="flex flex-col justify-start gap-3 py-3">
    <TestFeedback {is_string_accepted} />
    <Sidebar
      {saveWorkspace}
      bind:current_action
      {undo}
      {handleTrash}
      {clearCursor}
    />
  </div>
</div>
