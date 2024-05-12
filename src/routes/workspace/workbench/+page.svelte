<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { draw } from "../../../lib/drawingFuncs";
    import { roundToNearest, getClosestPointIndex, indexOfClosestLineToPoint } from "$lib/mathFuncs";
    import type { State, Connection, Coordinate, BezierCurve } from "$lib/interfaces";
    import { Action } from "$lib/enums";

    $: {if(start_state_coordinates && stringToCheck){
        const check_string = async () => {
            isStringAccepted = await invoke("test_string", {stateConnections: state_connections, 
                startStateCoordinates: start_state_coordinates, stringToCheck: stringToCheck});
        };
        check_string().catch((e)=>{
            console.log(e);
        });
    }};

    $: {if(context){
        draw(context, width, height, states, connections, start_state_index, selected_connection_index);
    }} 


    // Spaghettiest spaghetti code to every spaghetti, must refactor

    // DO NOT CHANGE ANY CODE IN FORM FOO = [...FOO, BAR]
    // Necessary to trigger sveltekit rerender of dynamic variables and draw to screen

    // Necessary for removing ghost image when dragging canvas
    const img = new Image();
    img.src = 'data:image/gif;base64,R0lGODlhAQABAIAAAAUEBAAAACwAAAAAAQABAAACAkQBADs=';

    // Consider refactoring into rust backend for faster performance in future
    let states: Array<State> = [];
    let connections: Array<Connection> = [];
    let elements: Array<State | Connection> = [];
    let start_state_index: number = -1;
    // hashing every coordinate to a state for use when user click on a given coordinate point
    // Allows for O(1) access without having to search for the state which was clicked in the State array
    let state_connections: {[key: string]: State | undefined} = {};
    let start_state_coordinates: string | null = null;
    let dialogue = "";
    let stringToCheck: String;
    
    $: width = 900;
    $: height = 900;
    
    let canvas: HTMLCanvasElement | null;
    let selected_connection_index: number | null = null;
    let control_point_index: number | null = null;
    let context: CanvasRenderingContext2D;
    let current_action: Action = Action.ADDING_REGULAR_STATE;

    let isStringAccepted: boolean;

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
        }else if(element.element === "State"){
            const state = states.pop();
            if(!state){
                return;
            }
            if(state.is_final == true){
                state.is_final = false;
                states = [...states, state];
                elements = [...elements, element];
            }else if(states.length == start_state_index){
                start_state_index = -1;
                start_state_coordinates = null;
                state_connections[`${state.position.x},${state.position.y}`] = undefined;
            }else{
                state_connections[`${state.position.x},${state.position.y}`] = undefined;
            }
        }else{
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

            }
            states = states;
        }
    
    const handleTrash = () => {
        states = [];
        connections = [];
        elements = [];
        start_state_index = -1;
        state_connections = {};
        start_state_coordinates = null;
        current_action = Action.ADDING_REGULAR_STATE;
    }

    const handleSubmit = (event: SubmitEvent)=> {
        if(!(event.target instanceof HTMLFormElement)){
            return;
        }
        const data = new FormData(event.target);
        const inputted_string = data.get("string");
        if(!inputted_string){
            return;
        }
        if(start_state_index === -1){
            dialogue = "You must specify at least one start state"
            return;
        }
        stringToCheck = inputted_string.toString();
    }

    const handleClick = (event: MouseEvent): void => {
        const cursor_x_pos = roundToNearest(event.x + window.scrollX, 100);
        const cursor_y_pos = roundToNearest(event.y + window.scrollY, 100);
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
                const starting_state_key = last_connection?.curve.start_point.x + "," + last_connection?.curve.start_point.y
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
                console.log(selected_state.position)
                console.log(last_connection.curve.end_point);
                if(selected_state === starting_state){
                    // If the connection is supposed to be a loop, the control points are automatically changed so it doesn't look like
                    // A single point when drawn and instead forms a circle like shape
                    last_connection.curve.control_point_one = {x: cursor_coords.x + 150, y: cursor_coords.y + 150};
                    last_connection.curve.control_point_two = {x: cursor_coords.x - 150, y: cursor_coords.y + 150};
                }
                connections.push(last_connection);
                elements.push(last_connection);
                current_action = Action.CLICKING;
                break;
            case Action.DRAGGING_LINE:
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
                const cursor_x_pos = roundToNearest(event.x + window.scrollX, 20);
                const cursor_y_pos = roundToNearest(event.y + window.scrollY, 20);
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
        if(selected_connection_index == null || event.ctrlKey || event.altKey || event.shiftKey){
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
        const cursor_coordinates = {x: event.x, y: event.y};
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
        const cursor_coords: Coordinate = {x: event.x, y: event.y};
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
    {#if isStringAccepted}
        <div class="text-center flex flex-col justify-center absolute top-5 right-5 bg-green-800 rounded-full border-black border-2 w-28 h-28">
            <div class="text-sm">
                The string was accepted!!
            </div>
        </div>
    {:else if isStringAccepted !== undefined}
        <div class="text-center flex flex-col justify-center absolute top-5 right-5 bg-[#e03c3c] rounded-full border-black border-2 w-28 h-28">
            <div class="text-sm">
                The string was not accepted
            </div>
        </div>
    {/if}
    <!-- Setting tableindex is necessary so element is focusable and can thus listen to keydown events -->
    <!-- svelte-ignore a11y-positive-tabindex -->
    <canvas tabindex="1" draggable="false"
    style={`width: ${width}px; height: ${height}px;`} width={width} height={height}
    bind:this={canvas} 
    on:mousemove={handleMove} 
    on:click={handleClick}
    on:dblclick={(event)=>{clearCursor(); selected_connection_index = indexOfClosestLineToPoint(event.clientX, event.clientY, connections)}}
    on:keyup={handleCharChange}
    on:mousedown={handleDragStart}
    on:mouseup={handleDragEnd}
    >
    </canvas>
    <div class="text-center select-none flex flex-col justify-between gap-3 bg-opacity-100 w-32 h-fit absolute right-4 top-0 bottom-0 my-auto border-black border-2 rounded-md px-2 py-4 mr-0.5 z-50">
        <div class="flex flex-col gap-2">
            <button on:click={()=>{clearCursor(); current_action = Action.ADDING_START_STATE;}} class="flex flex-col self-center" style="line-height: 15px;">
                New Start State
                <div class="mt-2 self-center bg-green-600 rounded-full w-14 h-14 border-black border-[1px]">
                </div>
            </button>
            <button on:click={()=>{clearCursor(); current_action = Action.ADDING_REGULAR_STATE;}}  class="flex flex-col self-center">
                New State
                <div class="self-center bg-orange-600 rounded-full w-14 h-14 border-black border-[1px]">
                </div>
            </button>
            <button on:click={()=>{clearCursor(); current_action = Action.ADDING_FINAL_STATE;}} 
                class="flex flex-col self-center" style="line-height: 15px;">
                New Final State
                <div  class="mt-2 self-center  rounded-full w-[4.5rem] h-[4.5rem] border-black border-[1px]">
                </div>
            </button>
            <button on:click={()=>{clearCursor(); current_action = Action.PLACING_LINE;}} class="flex flex-col " style="line-height: 15px;">
                New Connection
                <svg class="hover:cursor-pointer w-10 self-center" data-slot="icon" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3"></path>
                </svg>
            </button>

        </div>
        <div class="flex justify-center mt-2">
            <svg on:click={()=>{clearCursor(); undo(); selected_connection_index = null;}} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor"
                 viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M2.515 10.674a1.875 1.875 0 0 0 0 2.652L8.89 19.7c.352.351.829.549 1.326.549H19.5a3 3 0 0 0 3-3V6.75a3 3 0 0 0-3-3h-9.284c-.497 0-.974.198-1.326.55l-6.375 6.374ZM12.53 9.22a.75.75 0 1 0-1.06 1.06L13.19 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L15.31 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-1.72 1.72-1.72-1.72Z"></path>
              </svg>
            <svg on:click={()=>{clearCursor(); handleTrash(); selected_connection_index = null;}} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1 1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991 2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 0 1-.256-1.478A48.567 48.567 0 0 1 7.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0 1 3.369 0c1.603.051 2.815 1.387 2.815 2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6 0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0 1 0-1.5.058l.347 9a.75.75 0 1 0 1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347 9a.75.75 0 0 0 1.5.058l.345-9Z"></path>
            </svg>
            <svg on:click={()=>{clearCursor(); }} aria-hidden="true" class="hover:cursor-pointer w-6 mb-0.5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
                <path clip-rule="evenodd" d="M6.672 1.911a1 1 0 10-1.932.518l.259.966a1 1 0 001.932-.518l-.26-.966zM2.429 4.74a1 1 0 10-.517 1.932l.966.259a1 1 0 00.517-1.932l-.966-.26zm8.814-.569a1 1 0 00-1.415-1.414l-.707.707a1 1 0 101.415 1.415l.707-.708zm-7.071 7.072l.707-.707A1 1 0 003.465 9.12l-.708.707a1 1 0 001.415 1.415zm3.2-5.171a1 1 0 00-1.3 1.3l4 10a1 1 0 001.823.075l1.38-2.759 3.018 3.02a1 1 0 001.414-1.415l-3.019-3.02 2.76-1.379a1 1 0 00-.076-1.822l-10-4z" fill-rule="evenodd"></path>
              </svg>
        </div>
    </div>
    {#if (dialogue)}
        <div class="absolute top-0 right-0 left-0 w-fit h-fit mx-auto transition-all duration-300 bg-pink-400 px-5 py-1 rounded-md text-center">
            {dialogue}
        </div>
    {/if}

    <div class="flex flex-col justify-center">
        <form class="flex self-center" on:submit|preventDefault={handleSubmit}>
            <label for="string">
                Check String:
                <input class="border-black border-2 text-3xl rounded-md px-2 py-1" type="text" name="string">
            </label>
        </form>
    </div>

</div>
<div class="p-4 flex justify-center font-semibold">
    <a class="text-4xl " href="/">Home</a>
</div>
