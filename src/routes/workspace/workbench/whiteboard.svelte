<script lang="ts">
    import { onMount } from "svelte";
    import { draw } from "../../../lib/drawingFuncs";
    import { roundToNearest, getClosestPointIndex, indexOfClosestBezierCurveToPoint } from "$lib/mathFuncs";
    import type { State, Connection, Coordinate, BezierCurve } from "$lib/interfaces";
    import { Action } from "$lib/enums";
    import Sidebar from "./sidebar.svelte";

    export let start_state_coordinates: string | null;
    export let state_connections: {[key: string]: State | undefined};
    export let dialogue: string;
    export let start_state_index: number;
 

    $: {if(context){
        draw(context, width, height, states, connections, start_state_index, selected_connection_index);
    }};

    // DO NOT CHANGE ANY CODE IN FORM FOO = [...FOO, BAR]
    // Necessary to trigger sveltekit rerender of dynamic variables and draw to screen

    // Necessary for removing ghost image when dragging canvas
    const img = new Image();
    img.src = 'data:image/gif;base64,R0lGODlhAQABAIAAAAUEBAAAACwAAAAAAQABAAACAkQBADs=';

    // Consider refactoring into rust backend for faster performance in future
    let states: Array<State> = [];
    let connections: Array<Connection> = [];
    let elements: Array<State | Connection> = [];

    $: width = 900;
    $: height = 900;

    let canvas: HTMLCanvasElement | null;
    let selected_connection_index: number | null = null;
    let control_point_index: number | null = null;
    let context: CanvasRenderingContext2D;
    let current_action: Action = Action.ADDING_REGULAR_STATE;


    onMount(()=>{
        width = window.screen.availWidth;
        height = window.screen.height - 300;
        const ctx = canvas?.getContext("2d");
        if(ctx){
            context = ctx;
            context.strokeStyle = "black";
            context.imageSmoothingQuality = "high";
        }
    })

    const undo = (): void => {
        const element: State | Connection | undefined = elements.pop();
        if(!element){
            return;
        }
        if(element.element === "State"){
            const state = states.pop();
            if(!state){
                return;
            }
            if(states.length == start_state_index){
                start_state_index = -1;
                start_state_coordinates = null;
                state_connections[`${state.position.x},${state.position.y}`] = undefined;
            }else{
                state_connections[`${state.position.x},${state.position.y}`] = undefined;
            }
            states = states;
            return;
        }

        connections.pop();
        const node_one_key = `${element.curve.start_point.x},${element.curve.start_point.y}`;
        const node_two_key = `${element.curve.end_point.x},${element.curve.end_point.y}`

        const node_one: State | undefined = state_connections[node_one_key];
        const node_two: State | undefined = state_connections[node_two_key];
        if(!node_one || !node_two){
            return;
        }
        node_one.nodes_connected_to = node_one.nodes_connected_to.filter((connected_node_position)=>{
            return !(connected_node_position === node_two_key);
        });

        node_two.nodes_connected_from = node_two.nodes_connected_from.filter((connected_node_position)=>{
            return !(connected_node_position === node_one_key);
        });
        node_one.connection_chars.pop();

        state_connections[node_one_key] = node_one;
        state_connections[node_two_key] = node_two;

        states = states;
        }

    const handleTrash = () => {
        states = [];
        connections = [];
        elements = [];
        start_state_index = -1;
        state_connections = {};
        start_state_coordinates = null;
        current_action = Action.CLICKING;
    }

    const handleClick = (event: MouseEvent): void => {
        const cursor_x_pos = roundToNearest(event.offsetX, 100);
        const cursor_y_pos = roundToNearest(event.offsetY, 100);
        const cursor_coords: Coordinate = {x: cursor_x_pos, y: cursor_y_pos};

        let selected_state: State | undefined = state_connections[`${cursor_x_pos},${cursor_y_pos}`];
        dialogue = "";
        // Really needs to be refactored
        switch(current_action){
            case Action.ADDING_REGULAR_STATE:
                if(selected_state !== undefined){
                    dialogue = "You cannot place a Node on top of another Node";
                    return;
                }
                selected_state = {position: cursor_coords, nodes_connected_to: new Array<string>(), 
                nodes_connected_from: new Array<string>(), connection_chars: new Array<string>(), element: "State", is_final: false};
                elements.push(selected_state);
                states.push(selected_state);
                state_connections[`${cursor_x_pos},${cursor_y_pos}`] = selected_state;
                break;

            case Action.ADDING_FINAL_STATE:
                if(selected_state === undefined){
                    dialogue = "You must make an existing Node a final Node";
                    return;
                }else if(selected_state.is_final){
                    dialogue = "The Node is already a final Node";
                    return;
                }
                selected_state.is_final = true;
                break;
            
            case Action.ADDING_START_STATE:
                if(selected_state !== undefined){
                    dialogue = "You cannot place a Node on top of another Node";
                    return;
                }
                start_state_index = states.length;
                start_state_coordinates = `${cursor_x_pos},${cursor_y_pos}`;
                selected_state = {position: cursor_coords, nodes_connected_to: new Array<string>(), 
                nodes_connected_from: new Array<string>(), connection_chars: new Array<string>(), element: "State", is_final: false};
                elements.push(selected_state);
                states.push(selected_state);
                state_connections[`${cursor_x_pos},${cursor_y_pos}`] = selected_state;
                break;

            case Action.PLACING_LINE:
                if(selected_state === undefined){
                    dialogue = "You must place an arrow on top of another Node";
                    return;
                }
                const curve: BezierCurve = {start_point: cursor_coords, control_point_one: cursor_coords, 
                control_point_two: cursor_coords, end_point: cursor_coords};

                const connection: Connection = {curve: curve, element: "Connection", character: "a"};
                connections.push(connection);
                current_action = Action.DRAWING_LINE;
                break;

            case Action.DRAWING_LINE:
                if(selected_state === undefined){
                    dialogue = "The arrow must point to a valid Node";
                    return;
                }
                const last_connection = connections.pop();
                // Starting node's key will be at the x, y coordinates of the connection's start point
                // The selected node will treated as our "ending" node
                const starting_state_key = last_connection?.curve.start_point.x + "," + last_connection?.curve.start_point.y;
                const starting_state = state_connections[starting_state_key];
                if(last_connection === undefined || starting_state === undefined){
                    return;
                }
                starting_state.nodes_connected_to.push(cursor_x_pos + "," + cursor_y_pos);
                starting_state.connection_chars.push("a");
                selected_state.nodes_connected_from.push(starting_state_key);

                // First control point starts at the start coordinate, the second control point moves to follow the end coordinates
                // Makes drawing for user easier if control points are spread apart
                last_connection.curve.end_point = selected_state.position;
                last_connection.curve.control_point_two = selected_state.position;
                if(selected_state === starting_state){
                    // If the connection is supposed to be a loop, the control points are automatically changed so it doesn't look like
                    // A single point when drawn and instead forms a circle like shape
                    last_connection.curve.control_point_one = {x: cursor_coords.x - 200, y: cursor_coords.y + 200};
                    last_connection.curve.control_point_two = {x: cursor_coords.x - 200, y: cursor_coords.y - 200};
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
    }

    // Decent start
    // Try and draw without redrawing whole canvas
    const handleMove = (event: MouseEvent) =>{
        switch(current_action){
            case Action.DRAWING_LINE:
                const cursor_x_pos = roundToNearest(event.offsetX, 20);
                const cursor_y_pos = roundToNearest(event.offsetY, 20);
                const cursor_coords: Coordinate = {x: cursor_x_pos, y: cursor_y_pos};
                const connection = connections.pop();
                if(connection === undefined){
                    return;
                }
                connection.curve.end_point = cursor_coords;
                connections = [...connections, connection];
                break;

            default:
                break;
        }

    }

    const handleUndoEvent = (event: KeyboardEvent): void =>{
        if(event.ctrlKey === true && event.key === "z"){
            undo();
        }
    }

    const clearCursor = (): void => {
        selected_connection_index = null;
        current_action = Action.CLICKING;
    }

    // Used when an arrow is selected and the character of its transition is being changed by the user
    const handleCharChange = (event: KeyboardEvent): void => {
        if(selected_connection_index == null || event.ctrlKey || event.altKey || event.shiftKey || event.key == "Tab"){
            return;
        }
        const selected_connection = connections[selected_connection_index];
        selected_connection.character = event.key;

        const start_node_key = selected_connection.curve.start_point.x + "," + selected_connection.curve.start_point.y;
        const end_node_key = selected_connection.curve.end_point.x + "," + selected_connection.curve.end_point.y;
        const start_state: State | undefined = state_connections[start_node_key];
        const end_state: State | undefined = state_connections[end_node_key];

        if(!start_state || !end_state){
            return;
        }

        const connectionIndex = start_state.nodes_connected_to.indexOf(end_node_key);
        start_state.connection_chars[connectionIndex] = event.key;

        state_connections = state_connections;
        selected_connection_index = null;
        current_action = Action.CLICKING;
    }

    const handleDragStart = (event: MouseEvent): void => {
        if(selected_connection_index === null){
            return;
        }
        current_action = Action.DRAGGING_LINE;
        const cursor_coordinates = {x: event.offsetX, y: event.offsetY};
        const curve: BezierCurve = connections[selected_connection_index].curve;
        const control_points: Array<Coordinate> = [curve.control_point_one, curve.control_point_two];
        const index_of_closest_control_point = getClosestPointIndex(control_points, cursor_coordinates);
        control_point_index = index_of_closest_control_point;
    }

    const handleDrag = (event: MouseEvent) =>{
        if(current_action !== Action.DRAGGING_LINE || selected_connection_index === null || control_point_index === null){
            return;
        }
        const connection: Connection = connections[selected_connection_index];
        const curve: BezierCurve = connection.curve;
        const cursor_coords: Coordinate = {x: event.offsetX, y: event.offsetY};
        if(control_point_index === 0){
            // First control point is closest
            curve.control_point_one = cursor_coords;
        }else if(control_point_index === 1){
            // Second control point is closest
            curve.control_point_two = cursor_coords
        }else{
            // unreachable
            return;
        }
        connection.curve = curve;
        connections[selected_connection_index] = connection;
    }

    const handleDragEnd = (event: MouseEvent): void => {
        if(selected_connection_index === null){
            return;
        }
        current_action = Action.DRAGGING_LINE;
    }

</script>

<svelte:window on:keydown={handleUndoEvent} on:mousedown={handleDragStart} on:mouseup={handleDragEnd} on:mousemove={handleDrag} /> 
<div class="w-fit h-fit relative font-semibold overflow-scroll">

    <!-- Setting tabindex is necessary so element is focusable and can thus listen to keydown events -->
    <!-- svelte-ignore a11y-positive-tabindex -->
    <canvas tabindex="1" draggable="false"
    style={`width: ${width}px; height: ${height}px;`} width={width} height={height}
    bind:this={canvas} 
    on:mousemove={handleMove} 
    on:click={handleClick}
    on:dblclick={(event)=>{clearCursor(); 
        selected_connection_index = indexOfClosestBezierCurveToPoint({x: event.offsetX, y: event.offsetY}, connections)}}
    on:keyup={handleCharChange}
    on:mousedown={handleDragStart}
    on:mouseup={handleDragEnd}>
    </canvas>
    <Sidebar bind:current_action={current_action} undo={undo} 
    handleTrash={handleTrash} clearCursor={clearCursor} bind:selected_connection_index={selected_connection_index}/>

    
</div>
