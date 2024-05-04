<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { draw, roundToNearest} from "../../../lib/utils";
    import type { State, Arrow, StateConnection } from "../../../lib/interfaces";

    // DO NOT CHANGE ANY CODE IN FORM FOO = [...FOO, BAR]
    // Necessary to trigger sveltekit rerender of dynamic variables and draw to screen

    // Consider refactoring into rust backend for faster performance in future
    let states: Array<State> = [];
    let connections: Array<Arrow> = [];
    let elements: Array<State | Arrow> = [];
    let startStatePosition: number = -1;
    let finalStatePositions: Array<number> = []; 
    let stateConnections: {[key: string]: StateConnection | undefined} = {};
    let startStateCoordinates: string | null = null;
    let previouslySelectedNodeKey: string | null = null; 
    let dialogue = "";
    let stringToCheck: String;
    
    $: width = 900;
    $: height = 900;

    let canvas: HTMLCanvasElement | null;
    let context: CanvasRenderingContext2D;
    let lineSelected = false;
    let drawingLine = false;
    let linkStart: [number, number] = [0, 0];
    let isStartStateSelected = false;
    let isFinalStateSelected = false;

    let isStringAccepted: boolean;

    $: {if(startStateCoordinates && stringToCheck){
        const check_string = async () => {
            isStringAccepted = await invoke("test_string", {stateConnections: stateConnections, 
                startStateCoordinates: startStateCoordinates, stringToCheck: stringToCheck});
        }
        check_string();
    }};

    $: {if(context){
        draw(context, width, height, states, connections, startStatePosition, finalStatePositions);
    }} 

    onMount(()=>{
        width = window.innerWidth;
        height = window.innerHeight
        const ctx = canvas?.getContext("2d");
        if(ctx){
            context = ctx
            context.strokeStyle = "black";
            context.imageSmoothingQuality = "high";
        }
    })

    const undo = (): void =>{
        const element: State | Arrow | undefined = elements.pop();
        if(!element){
            return;
        }else{
            if(element.element === "State"){
                const state = states.pop();
                if(!state){
                    return;
                }else{
                    stateConnections[`${state.x_pos}${state.y_pos}`] = undefined;
                }
            }else{
                connections.pop();
            }
        }
        states = states
    }

    const handleTrash = () => {
        states = [];
        connections = [];
        elements = [];
        startStatePosition = -1;
        finalStatePositions = [];
        stateConnections = {};
        startStateCoordinates = null;
        previouslySelectedNodeKey = null;
        lineSelected = false;
        drawingLine = false;
        linkStart = [0, 0];
        isFinalStateSelected = false;
        isStartStateSelected = false;
    }

    const handleSubmit = (event: SubmitEvent)=> {
        if(!(event.target instanceof HTMLFormElement)){
            return;
        }
        const data = new FormData(event.target);
        const inputtedString = data.get("string");
        if(!inputtedString){
            return;
        }
        if(startStatePosition === -1){
            dialogue = "You must specify at least one start state"
            return;
        }
        stringToCheck = inputtedString.toString();
    }


    const handleClick = (event: MouseEvent): void => {
        const cursor_x_pos = roundToNearest(event.x + window.scrollX, 100);
        const cursor_y_pos = roundToNearest(event.y + window.scrollY, 100);

        const selectedState: StateConnection | undefined = stateConnections[`${cursor_x_pos}${cursor_y_pos}`];

        if(!lineSelected){
            if(selectedState){
                dialogue = "You cannot place a Node on top of another Node";
                return;
            }   
            dialogue = "";
            const node: State = {x_pos: cursor_x_pos, y_pos: cursor_y_pos, element: "State"};
            let nodeConnection: StateConnection = {nodes_connected_from: [], nodes_connected_to: [], connection_chars: [], is_final_state: false};
            elements = [...elements, node];
            states = [...states, node];
            if(isStartStateSelected){
                startStatePosition = states.length - 1;
                startStateCoordinates = `${cursor_x_pos}${cursor_y_pos}`;
                isStartStateSelected = false;
            }else if(isFinalStateSelected){
                finalStatePositions = [...finalStatePositions, states.length - 1];
                nodeConnection.is_final_state = true;
            }
            stateConnections[`${cursor_x_pos}${cursor_y_pos}`] = nodeConnection;

        }else if(lineSelected && !drawingLine){
            if(!selectedState){
                dialogue = "You must place an arrow on top of another Node";
                return;
            }
            dialogue = "";
            const connection: Arrow = {x1_pos: cursor_x_pos, y1_pos: cursor_y_pos, x2_pos: cursor_x_pos, y2_pos: cursor_y_pos, element: "Connection"}
            connections = [...connections, connection];
            drawingLine = true;
            linkStart = [cursor_x_pos, cursor_y_pos];
            previouslySelectedNodeKey = `${cursor_x_pos}${cursor_y_pos}`

        }else if(lineSelected && drawingLine){
            if(!selectedState || !previouslySelectedNodeKey){
                dialogue = "The arrow must point to a valid Node";
                return;
            }
            const previousNode = stateConnections[previouslySelectedNodeKey];
            const currentNode = stateConnections[`${cursor_x_pos}${cursor_y_pos}`];
            if(!previousNode || !currentNode){
                dialogue = "The arrow must point to a valid Node";
                return;
            }
            previousNode.nodes_connected_to.push(`${cursor_x_pos}${cursor_y_pos}`);
            previousNode.connection_chars.push("a");
            currentNode.nodes_connected_from.push( previouslySelectedNodeKey);
            stateConnections = stateConnections;
            const line = connections.pop();
            if(line){
                line.x2_pos = cursor_x_pos;
                line.y2_pos = cursor_y_pos;
                connections = [...connections, line];
                elements = [...elements, line];
                drawingLine = false;
            }
        }
    }

    // Decent start
    // Try and draw without redrawing whole canvas
    const handleMove = (event: MouseEvent) =>{
        const cursor_x_pos = roundToNearest(event.x + window.scrollX, 20);
        const cursor_y_pos = roundToNearest(event.y + window.scrollY, 20);
        if(lineSelected && drawingLine){
            const line = connections.pop();
            if(line){
                line.x2_pos = cursor_x_pos;
                line.y2_pos = cursor_y_pos;
                connections = [...connections, line];
            }
        }else{
            return;
        }

    }

    const handleUndoEvent = (event: KeyboardEvent): void =>{
        if(event.ctrlKey === true && event.key === "z"){
            undo();
        }
    }

</script>

<svelte:window on:keydown={handleUndoEvent} on:resize={()=>{width = window.innerWidth; height = window.innerHeight;}}/> 
<div class="w-fit h-fit relative font-semibold overflow-x-hidden">
    <div class="text-center absolute top-5">
        {#if isStringAccepted}
            <div>The string was accepted!!</div>
        {:else if isStringAccepted !== undefined}
            <div>The string was not accepted</div>
        {/if}
    </div>
    <canvas style="width: {width}; height: {height};" width={width} height={height} bind:this={canvas} on:mousemove={handleMove} on:click={handleClick} >
    </canvas>
    <div class="text-center select-none flex flex-col justify-between gap-3 bg-opacity-100 w-32 h-fit absolute right-4 top-0 bottom-0 my-auto border-black border-2 rounded-md px-2 py-4 mr-0.5 z-50">
        <div class="flex flex-col gap-2">
            <button on:click={()=>{isFinalStateSelected = false; isStartStateSelected = true; lineSelected = false;}} class="flex flex-col self-center" style="line-height: 15px;">
                New Start State
                <div class="mt-2 self-center bg-green-600 rounded-full w-14 h-14 border-black border-[1px]">
                </div>
            </button>
            <button on:click={()=>{isFinalStateSelected = false; isStartStateSelected = false; lineSelected = false;}}  class="flex flex-col self-center">
                New State
                <div class="self-center bg-orange-600 rounded-full w-14 h-14 border-black border-[1px]">
                </div>
            </button>
            <button on:click={()=>{isStartStateSelected = false; isFinalStateSelected = true; lineSelected = false;}} class="flex flex-col self-center" style="line-height: 15px;">
                New Final State
                <div  class="mt-2 self-center bg-blue-600 rounded-full w-14 h-14 border-black border-[1px]">
                </div>
            </button>
            <button on:click={()=>{lineSelected = true;}} class="flex flex-col " style="line-height: 15px;">
                New Connection
                <svg class="hover:cursor-pointer w-10 self-center" data-slot="icon" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3"></path>
                </svg>
            </button>

        </div>
        <div class="flex justify-center mt-2">
            <svg on:click={()=>{undo()}} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M2.515 10.674a1.875 1.875 0 0 0 0 2.652L8.89 19.7c.352.351.829.549 1.326.549H19.5a3 3 0 0 0 3-3V6.75a3 3 0 0 0-3-3h-9.284c-.497 0-.974.198-1.326.55l-6.375 6.374ZM12.53 9.22a.75.75 0 1 0-1.06 1.06L13.19 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L15.31 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-1.72 1.72-1.72-1.72Z"></path>
              </svg>
            <svg on:click={handleTrash} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1 1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991 2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 0 1-.256-1.478A48.567 48.567 0 0 1 7.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0 1 3.369 0c1.603.051 2.815 1.387 2.815 2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6 0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0 1 0-1.5.058l.347 9a.75.75 0 1 0 1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347 9a.75.75 0 0 0 1.5.058l.345-9Z"></path>
            </svg>
        </div>
    </div>
    <div class="absolute top-0 right-0 left-0 w-fit h-fit mx-auto transition-all duration-300 bg-pink-400 px-5 py-1 rounded-md text-center">
        {dialogue}
    </div>
    <div class="flex flex-col justify-center">
        <form class="flex self-center" on:submit|preventDefault={handleSubmit}>
            <label for="string">
                Check String:
                <input class="border-black border-2 text-3xl rounded-md px-2 py-1" type="text" name="string">
            </label>
        </form>
    </div>

</div>
<a href="/">Home</a>