<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api";
    import { draw, roundToNearest } from "../../../lib/utils";
    import type { State, Connection, Node } from "../../../lib/interfaces";

    // Consider splitting elements into state and connection arrays
    // Map coordinates to elements?
    // Consider refactoring into rust backend for faster performance in future
    let nodes: Array<State> = [];
    let connections: Array<Connection> = [];
    let elements: Array<State | Connection> = [];
    let startStatePosition: number = -1;
    let finalStatePositions: Array<number> = []; 
    let nodeConnections: Array<Node> = [];

    let dialogue = "";
    
    $: {if(startStatePosition != -1){
        invoke("get_links", {links: nodeConnections, startLinkPosition: startStatePosition});
    }}

    
    $: width = 900;
    $: height = 900;

    let canvas: HTMLCanvasElement | null;
    let context: CanvasRenderingContext2D;
    let lineSelected = false;
    let drawingLine = false;
    let linkStart: [number, number] = [0, 0];
    let selectedStartState = false;
    let selectedFinalState = false;

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

    const handleClick = (event: MouseEvent): void => {
        const cursor_x_pos = roundToNearest(event.x + window.scrollX, 100);
        const cursor_y_pos = roundToNearest(event.y + window.scrollY, 100);

        // const node_at_coords = nodes.some((node) => node.x_pos === cursor_x_pos && node.y_pos === cursor_y_pos);
        // Check if non final state exisys
        let nodeAtCoords = nodes.indexOf({x_pos: cursor_x_pos, y_pos: cursor_y_pos, element: "State", final: false});

        if(nodeAtCoords === -1){
            nodeAtCoords = nodes.indexOf({x_pos: cursor_x_pos, y_pos: cursor_y_pos, element: "State", final: false});
        }


        if(!lineSelected){
            if(nodeAtCoords !== -1){
                dialogue = "There is already a node at the specified location";
            }else{
                dialogue = "";
                const node: State = {x_pos: cursor_x_pos, y_pos: cursor_y_pos, element: "State", final: false};
                elements.push(node);
                if(selectedStartState){
                    startStatePosition = nodes.length - 1;
                }else if(selectedFinalState){
                    finalStatePositions.push(nodes.length - 1);
                    node.final = true;
                }
                nodes.push(node);

            }
        }else if(lineSelected && !drawingLine){
            if(nodeAtCoords !== -1){
                connections.push({x1_pos: cursor_x_pos, y1_pos: cursor_y_pos, x2_pos: cursor_x_pos, y2_pos: cursor_y_pos, element: "Connection"});
                drawingLine = true;
                linkStart = [cursor_x_pos, cursor_y_pos];
                dialogue = "";
            }else{
                dialogue = "You must place an arrow on top of where a Node element is";
            }

        }else if(lineSelected && drawingLine){
            if(nodeAtCoords !== -1){

                const node = nodes.at(nodeAtCoords)
                if(node){
                    const nodeStatus = node.final;
                    nodeConnections = [...nodeConnections, {connected_nodes: [], connection_chars: [], final: nodeStatus}];
                    const line = connections.pop();
                    if(line){
                        line.x2_pos = cursor_x_pos;
                        line.y2_pos = cursor_y_pos;
                        connections.push(line);
                        elements.push(line);
                        drawingLine = false;
                    }
                dialogue = "";
                }else{
                    return;
                }


            }else{
                dialogue = "The connection must point to a valid Node element."
            }
        }
        draw(context, width, height, nodes, connections, startStatePosition, finalStatePositions);
    }

    // Decent start
    // Try and draw without redrawing whole canvas
    const handleMove = (event: MouseEvent) =>{
        const cursor_x_pos = roundToNearest(event.x, 20);
        const cursor_y_pos = roundToNearest(event.y, 20);
        if(lineSelected && drawingLine){
            const line = connections.pop();
            if(line){
                line.x2_pos = cursor_x_pos;
                line.y2_pos = cursor_y_pos;
                connections.push(line);
            }
            draw(context, width, height, nodes, connections, startStatePosition, finalStatePositions);
        }else{
            return;
        }

    }

    const undo = (): void =>{
        const element: State | Connection | undefined = elements.pop();
        if(!element){
            return;
        }else{
            if(element.element === "State"){
                nodes.pop();
            }else{
                connections.pop();
            }
        }
        draw(context, width, height, nodes, connections, startStatePosition, finalStatePositions);
    }

    const handleUndoEvent = (event: KeyboardEvent): void =>{
        if(event.ctrlKey === true && event.key === "z"){
            undo();
        }
    }

</script>

<svelte:window on:keydown={handleUndoEvent} on:resize={()=>{width = window.innerWidth; height = window.innerHeight;}}/> 
<div class="w-full h-full relative font-semibold">
    <canvas style="width: {width}; height: {height};" width={width} height={height} bind:this={canvas} id="Canvas" on:mousemove={handleMove} on:click={handleClick} >
    </canvas>
    <div class="text-center select-none flex flex-col justify-between gap-3 bg-opacity-100 w-32 h-fit absolute right-0 top-0 bottom-0 my-auto border-black border-2 rounded-md px-2 py-4 mr-0.5 z-50">
        <div class="flex flex-col gap-2">
            <button on:click={()=>{selectedFinalState = false; selectedStartState = true; lineSelected = false;}} class="flex flex-col self-center" style="line-height: 15px;">
                New Start State
                <div class="mt-2 self-center bg-green-600 rounded-full w-14 h-14 border-black border-[1px]">
                </div>
            </button>
            <button on:click={()=>{selectedFinalState = false; selectedStartState = false; lineSelected = false;}}  class="flex flex-col self-center">
                New State
                <div class="self-center bg-orange-600 rounded-full w-14 h-14 border-black border-[1px]">
                </div>
            </button>
            <button on:click={()=>{selectedStartState = false; selectedFinalState = true; lineSelected = false;}} class="flex flex-col self-center" style="line-height: 15px;">
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
            <svg on:click={undo} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M2.515 10.674a1.875 1.875 0 0 0 0 2.652L8.89 19.7c.352.351.829.549 1.326.549H19.5a3 3 0 0 0 3-3V6.75a3 3 0 0 0-3-3h-9.284c-.497 0-.974.198-1.326.55l-6.375 6.374ZM12.53 9.22a.75.75 0 1 0-1.06 1.06L13.19 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L15.31 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-1.72 1.72-1.72-1.72Z"></path>
              </svg>
            <svg on:click={()=>{selectedFinalState = false; selectedStartState = false; nodes = []; connections = []; draw(context, width, height, nodes, connections, startStatePosition, finalStatePositions);}} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1 1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991 2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 0 1-.256-1.478A48.567 48.567 0 0 1 7.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0 1 3.369 0c1.603.051 2.815 1.387 2.815 2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6 0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0 1 0-1.5.058l.347 9a.75.75 0 1 0 1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347 9a.75.75 0 0 0 1.5.058l.345-9Z"></path>
            </svg>

        </div>
    </div>
    <div>
        {dialogue}
    </div>
</div>
<a href="/">Home</a>