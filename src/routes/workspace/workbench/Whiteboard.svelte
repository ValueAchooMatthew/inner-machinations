<script lang="ts">
  import { onMount } from "svelte";
  import { draw } from "$lib/drawingFuncs";
  import { roundToNearest, getClosestPointIndex, indexOfClosestBezierCurveToPoint } from "$lib/mathFuncs";
  import type { State, Connection, Coordinate, BezierCurve } from "$lib/interfaces";
  import { Action } from "$lib/enums";
  import Sidebar from "./Sidebar.svelte";
  import TestFeedback from "./TestFeedback.svelte";
  import { saveWorkspace } from "$lib/savingWorkspaceFuncs";
  import { convertCoordinateToString, removeFirstElementFromArray } from "$lib/miscUtils";
  import AdvancedAutomataFunctions from "./AdvancedAutomataFunctions.svelte";
  import { list_of_states, list_of_connections, selected_connection_index, 
  state_positions, start_state_index, start_state_position, list_of_all_elements, current_action } from "$lib/automataStores";
  import { undo } from "$lib/deletionFuncs";
  import { handleUserClickingCanvas } from "$lib/userEvents";

  export let default_connection_char: string = "a";
  export let is_string_accepted: boolean | null;
  export let workspace_name: string | undefined;
  export let email: string | undefined;
  export let highlighted_state: State | null;

  // DO NOT CHANGE ANY CODE IN FORM FOO = [...FOO, BAR]
  // Necessary to trigger sveltekit rerender of dynamic variables and draw to screen

  $: {
    if (context) {
      draw(
        context,
        width,
        height,
        $list_of_states,
        $list_of_connections,
        $selected_connection_index,
        highlighted_state,
        1
      );
    }
  }

  // Necessary for removing ghost image when dragging canvas
  const img = new Image();
  img.src =
    "data:image/gif;base64,R0lGODlhAQABAIAAAAUEBAAAACwAAAAAAQABAAACAkQBADs=";

  // Consider refactoring into rust backend for faster performance in future

  $: width = 900;
  $: height = 900;

  let canvas: HTMLCanvasElement | null;
  let control_point_index: number | null = null;
  let context: CanvasRenderingContext2D;

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

  });

  const handleClick = (event: MouseEvent): void => {
    const cursor_x_pos = roundToNearest(event.offsetX, 100);
    const cursor_y_pos = roundToNearest(event.offsetY, 100);
    handleUserClickingCanvas(cursor_x_pos, cursor_y_pos, default_connection_char);
  };

  // Decent start
  // Try and draw without redrawing whole canvas
  const handleMove = (event: MouseEvent) => {
    
    if($current_action !== Action.PLACING_END_OF_LINE){
      return;
    }

    const cursor_x_pos = roundToNearest(event.offsetX, 20);
    const cursor_y_pos = roundToNearest(event.offsetY, 20);
    const cursor_coords: Coordinate = { x: cursor_x_pos, y: cursor_y_pos };

    list_of_connections.update((connections)=>{
      const connection = connections.pop();
      if(!connection){
        return connections;
      }
      connection.curve.end_point = cursor_coords;
      connections.push(connection);
      return connections;
    });

  };

  const handleUndoEvent = async (event: KeyboardEvent): Promise<void> => {
    if (event.ctrlKey === true && event.key === "z") {
      undo();
    } else if (event.ctrlKey === true && event.key === "s") {
      saveWorkspace(email, workspace_name);
    }
  };

  const clearCursor = (): void => {
    current_action.set(Action.CLICKING);
    selected_connection_index
  };

  // Used when an arrow is selected and the character of its transition is being changed by the user
  const handleCharChange = (event: KeyboardEvent): void => {
    if (
      $selected_connection_index === null
      || event.ctrlKey
      || event.altKey
      || event.shiftKey
      || event.key == "Tab"
    ) {
      return;
    }


    const selected_connection = $list_of_connections[$selected_connection_index];
    const old_character = selected_connection.connection_character;
    const new_character = event.key;
    selected_connection.connection_character = new_character;

    const start_state_key = convertCoordinateToString(
      selected_connection.curve.start_point,
    );

    const end_state_key = convertCoordinateToString(
      selected_connection.curve.end_point,
    );

    const start_state = $state_positions
      .get(start_state_key);

    const end_state = $state_positions
      .get(end_state_key);

    if (start_state === undefined || end_state === undefined) {
      return;
    }

    let state_connections_of_previous_character = start_state.states_connected_to
      .get(old_character);

    let state_connections_of_new_character = start_state.states_connected_to
      .get(new_character);

    if (state_connections_of_previous_character === undefined) {
      return;
    }
    // Removing the end state from the old connection character's hashmap
    state_connections_of_previous_character = removeFirstElementFromArray(state_connections_of_previous_character, end_state_key);

    // Adding end state to new connection character's hashmap
    if (state_connections_of_new_character === undefined) {
      start_state.states_connected_to.set(
        new_character,
        new Array<String>(end_state_key),
      );
    } else {
      state_connections_of_new_character.push(end_state_key);
    }

    list_of_connections.update((connections)=>{
      // same issue
      connections[$selected_connection_index] = selected_connection;
      return connections;
    })
    list_of_states

    $state_positions = $state_positions;
    selected_connection_index.set(null);
    current_action.set(Action.CLICKING)
  };

  const handleDragStart = (event: MouseEvent): void => {
    if ($selected_connection_index === null) {
      return;
    }
    current_action.set(Action.DRAGGING_LINE);
    const cursor_coordinates = { x: event.offsetX, y: event.offsetY };
    const curve: BezierCurve = $list_of_connections[$selected_connection_index].curve;
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
      $current_action !== Action.DRAGGING_LINE ||
      $selected_connection_index === null ||
      control_point_index === null
    ) {
      return;
    }
    const connection: Connection = $list_of_connections[$selected_connection_index];
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
    list_of_connections.update((connections)=>{
      connections[$selected_connection_index] = connection;
      return connections;
    });
  };

  const handleDragEnd = (): void => {
    if ($selected_connection_index === null) {
      return;
    }
    current_action.set(Action.DRAGGING_LINE);
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
      selected_connection_index.set(indexOfClosestBezierCurveToPoint(
        { x: event.offsetX, y: event.offsetY },
      ));
    }}
    on:keyup={handleCharChange}
    on:mousedown={handleDragStart}
    on:mouseup={handleDragEnd}
  >
  </canvas>
  <div class="flex flex-col justify-start gap-3 py-3">
    <TestFeedback {is_string_accepted} />
    <Sidebar
      {clearCursor}
      {email}
      {workspace_name}
    />
    <AdvancedAutomataFunctions />
  </div>

</div>