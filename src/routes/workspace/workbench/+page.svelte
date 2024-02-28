<script lang="ts">
    import { onMount } from "svelte";
    import { draw, roundToNearest } from "../../../lib/utils";
    import type { State, Connection, Link } from "../../../lib/interfaces";

    // Consider splitting elements into state and connection arrays
    let elements: Array<State | Connection> = [];
    let startStatePosition: number;
    let finalStatePositions: Array<number> = []; 
    let links: Array<Link> = [];

    $: width = 900;
    $: height = 900;
    let canvas: HTMLCanvasElement | null;
    let context: CanvasRenderingContext2D;
    let lineSelected = false;
    let drawingLine = false;
    let selectedLink: Link = {startNode: [0, 0], nextNode: [0, 0], character: ""};
    let selectedStartState = false;
    let selectedFinalState = false;

    $: console.log(elements)

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
        const cursor_x_pos = roundToNearest(event.x, 100);
        const cursor_y_pos = roundToNearest(event.y, 100);
        if(!lineSelected){
            if(selectedStartState){
                elements.push({x_pos: cursor_x_pos, y_pos: cursor_y_pos, element: "State"});
                startStatePosition = elements.length - 1
            }else if(selectedFinalState){
                elements.push({x_pos: cursor_x_pos, y_pos: cursor_y_pos, element: "State"});
                finalStatePositions.push(elements.length - 1)
            }else{
                elements.push({x_pos: cursor_x_pos, y_pos: cursor_y_pos, element: "State"});
            }
        }else if(lineSelected && !drawingLine){
            elements.push({x1_pos: cursor_x_pos, y1_pos: cursor_y_pos, x2_pos: cursor_x_pos, y2_pos: cursor_y_pos, element: "Connection"});
            drawingLine = true;
            selectedLink.startNode = [cursor_x_pos, cursor_y_pos];
        }else if(lineSelected && drawingLine){

            selectedLink.nextNode = [cursor_x_pos, cursor_y_pos];
            links.push(selectedLink);
            selectedLink = {startNode: [0, 0], nextNode: [0, 0], character: ""};

            const line = elements.pop();
            if(line && line.element === "Connection"){
                line.x2_pos = cursor_x_pos;
                line.y2_pos = cursor_y_pos;
                elements.push(line);
                drawingLine = false;
            }
        }
        draw(context, width, height, elements, startStatePosition, finalStatePositions);
    }

    // Decent start
    // Try and draw without redrawing whole canvas
    const handleMove = (event: MouseEvent) =>{
        const cursor_x_pos = roundToNearest(event.x, 20);
        const cursor_y_pos = roundToNearest(event.y, 20);
        if(lineSelected && drawingLine){
            const line = elements.pop();
            if(line && line.element === "Connection"){
                line.x2_pos = cursor_x_pos;
                line.y2_pos = cursor_y_pos;
                elements.push(line);
            }
            draw(context, width, height, elements, startStatePosition, finalStatePositions);
        }else{
            return;
        }

    }

    const handleUndo = (event: KeyboardEvent): void =>{
        if(event.ctrlKey === true && event.key === "z"){
            elements.pop();
            draw(context, width, height, elements, startStatePosition, finalStatePositions);

        }
    }

</script>

<svelte:window on:keydown={handleUndo} on:resize={()=>{width = window.innerWidth; height = window.innerHeight;}}/> 
<div class="w-full h-full relative">
    <canvas style="width: {width}; height: {height};" width={width} height={height} bind:this={canvas} id="Canvas" on:mousemove={handleMove} on:click={handleClick} >
    </canvas>
    <div class="text-center flex flex-col justify-between gap-3 bg-opacity-100 w-32 h-fit absolute right-0 top-0 bottom-0 my-auto border-black border-2 rounded-md px-2 py-4 mr-0.5 z-50">
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
            <div class="flex flex-col" style="line-height: 15px;">
                New Connection
                <button on:click={()=>{lineSelected = true;}} class="mt-2 self-center w-10 border-black border-2">
                </button>
            </div>

        </div>
        <div class="flex justify-center mt-2">
            <svg on:click={()=>{elements.pop(); draw(context, width, height, elements, startStatePosition, finalStatePositions);}} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M2.515 10.674a1.875 1.875 0 0 0 0 2.652L8.89 19.7c.352.351.829.549 1.326.549H19.5a3 3 0 0 0 3-3V6.75a3 3 0 0 0-3-3h-9.284c-.497 0-.974.198-1.326.55l-6.375 6.374ZM12.53 9.22a.75.75 0 1 0-1.06 1.06L13.19 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L15.31 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-1.72 1.72-1.72-1.72Z"></path>
              </svg>
            <svg on:click={()=>{selectedFinalState = false; selectedStartState = false; elements = []; draw(context, width, height, elements, startStatePosition, finalStatePositions);}} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path clip-rule="evenodd" fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1 1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991 2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 0 1-.256-1.478A48.567 48.567 0 0 1 7.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0 1 3.369 0c1.603.051 2.815 1.387 2.815 2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6 0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0 1 0-1.5.058l.347 9a.75.75 0 1 0 1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347 9a.75.75 0 0 0 1.5.058l.345-9Z"></path>
            </svg>

        </div>

    </div>
</div>
<a href="/">Home</a>