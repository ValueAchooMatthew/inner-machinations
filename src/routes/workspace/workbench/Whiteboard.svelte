<script lang="ts">
  import { onMount } from "svelte";
  import { draw } from "$lib/utils/drawingFuncs";
  import { roundToNearest, getClosestPointIndex, indexOfClosestBezierCurveToPoint } from "$lib/utils/mathFuncs";
  import type { State, Connection, Coordinate, BezierCurve } from "$lib/types/interfaces";
  import { Action } from "$lib/types/enums";
  import { saveWorkspace } from "$lib/utils/savingWorkspaceFuncs";
  import { convertCoordinateToString, removeFirstElementFromArray } from "$lib/utils/miscUtils";
  import { list_of_states, list_of_connections, selected_connection_index, 
  state_positions, current_action, email, workspace_name,
  input_alphabet} from "$lib/utils/automataStores";
  import { undo } from "$lib/utils/deletionFuncs";
  import { handleUserClickingCanvas } from "$lib/utils/userEvents";

  export let default_connection_character: string = "a";
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
    handleUserClickingCanvas(cursor_x_pos, cursor_y_pos, default_connection_character);
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

  const handleKeyDownEvent = async (event: KeyboardEvent): Promise<void> => {
    // if (event.ctrlKey === true && event.key === "z") {
    //   undo();
    // } else 
    if (event.ctrlKey === true && event.key === "s") {
      saveWorkspace();
    }
  };

  const clearCursor = (): void => {
    current_action.set(Action.CLICKING);
    selected_connection_index.set(null);
  };

  // Used when an arrow is selected and the character of its transition is being changed by the user
  const handleCharChange = (event: KeyboardEvent): void => {
    if ($selected_connection_index === null || event.key.length !== 1) {
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

    if (start_state === undefined) {
      return;
    }

    let state_connections_of_previous_character = start_state.states_connected_to
      .get(old_character);

    let state_connections_of_new_character = start_state.states_connected_to
      .get(new_character);

    if (state_connections_of_previous_character === undefined) {
      return;
    }
    // Removing the end state from the old connection character's hashset
    state_connections_of_previous_character = removeFirstElementFromArray(state_connections_of_previous_character, end_state_key)

    // Adding end state to new connection character's hashset
    if (state_connections_of_new_character === undefined) {

      start_state.states_connected_to.set(
        new_character,
        new Array(end_state_key)
      );
    } else {
      state_connections_of_new_character.push(end_state_key);
    }

    list_of_connections.update((connections)=>{
      // same issue
      connections[$selected_connection_index] = selected_connection;
      return connections;
    })

    state_positions.update((previous_state_positions)=>{
      previous_state_positions.set(start_state_key, start_state);
      return previous_state_positions;
    });

    selected_connection_index.set(null);
    current_action.set(Action.CLICKING);
    if(!$input_alphabet.includes(new_character) && new_character.length === 1) {
      input_alphabet.update((previous_input_alphabet) => {
        previous_input_alphabet.push(new_character);
        return previous_input_alphabet;
      })
    }

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
  on:keydown={handleKeyDownEvent}
  on:mousedown={handleDragStart}
  on:mouseup={handleDragEnd}
  on:mousemove={handleDrag}/>
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
    on:mouseup={handleDragEnd}>
  </canvas>

