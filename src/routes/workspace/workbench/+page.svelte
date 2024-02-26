<script lang="ts">
    import { onMount } from "svelte";

    let elements: Array<Circle | Line> = [];
    $: width = 900;
    $: height = 900;
    let fillColour: string = "rgb(234, 88, 12)";
    let canvas: HTMLCanvasElement | null ;
    let context: CanvasRenderingContext2D | null | undefined;
    let lineSelected = false;
    let drawingLine = false;

    const roundToNearest = (numberToRound: number, roundTo: number): number => {
        const remainder = numberToRound % roundTo;
        const half = Math.floor(roundTo / 2);
        if(remainder >= half){
            return numberToRound + (roundTo - remainder)
        }else{
            return numberToRound - remainder
        }

    }

    interface Circle{
        x_pos: number,
        y_pos: number,
        colour: string,
        element: "Circle"
    };

    interface Line {
        x1_pos: number,
        y1_pos: number,
        x2_pos: number,
        y2_pos: number,
        element: "Line"
    };

    onMount(()=>{
        width = window.innerWidth;
        height = window.innerHeight
        context = canvas?.getContext("2d");
        if(context){
            context.strokeStyle = "black";
            context.imageSmoothingQuality = "high";
        }
    })


    const draw = () =>{
        if(!context){
            return;
        }
        context.clearRect(0, 0, width, height);
        elements.forEach((obj)=>{
            if(context){
                context.beginPath();
                if(obj.element === "Circle"){
                    context.lineWidth = 3;
                    context.fillStyle = obj.colour;
                    context.arc(obj.x_pos, obj.y_pos, 35, 0, 2*Math.PI);
                    context.fill();
                    context.stroke();
                }else{
                    context.lineWidth = 7;
                    context.beginPath();
                    context.moveTo(obj.x1_pos, obj.y1_pos);
                    context.lineTo(obj.x2_pos, obj.y2_pos);
                    context.stroke();
                }
                context.closePath();
            }
        })
    }

    const handleClick = (event: MouseEvent): void => {
        const cursor_x_pos = roundToNearest(event.x, 100);
        const cursor_y_pos = roundToNearest(event.y, 100);
        if(!lineSelected){
            elements.push({x_pos: cursor_x_pos, y_pos: cursor_y_pos, colour: fillColour, element: "Circle"});
        }else if(lineSelected && !drawingLine){
            elements.push({x1_pos: cursor_x_pos, y1_pos: cursor_y_pos, x2_pos: cursor_x_pos, y2_pos: cursor_y_pos, element: "Line"});
            drawingLine = true;
        }else if(lineSelected && drawingLine){
            const line = elements.pop();
            if(line && line.element === "Line"){
                line.x2_pos = cursor_x_pos;
                line.y2_pos = cursor_y_pos;
                elements.push(line);
                drawingLine = false;
            }
        }
        draw();
    }

    // Decent start
    // Try and draw without redrawing whole canvas
    const handleMove = (event: MouseEvent) =>{
        const cursor_x_pos = roundToNearest(event.x, 20);
        const cursor_y_pos = roundToNearest(event.y, 20);
        if(lineSelected && drawingLine){
            const line = elements.pop();
            if(line && line.element === "Line"){
                line.x2_pos = cursor_x_pos;
                line.y2_pos = cursor_y_pos;
                elements.push(line);
            }
            draw();
        }else{
            return;
        }

    }

    
    const handleUndo = (event: KeyboardEvent): void =>{
        if(event.ctrlKey === true && event.key === "z"){
            elements.pop();
            draw();
        }
    }
</script>

<svelte:window on:keydown={handleUndo} on:resize={()=>{width = window.innerWidth; height = window.innerHeight;}}/> 
<div class="w-full h-full relative">
    <canvas style="width: {width}; height: {height};" width={width} height={height} bind:this={canvas} id="Canvas" on:mousemove={handleMove} on:click={handleClick} >
    </canvas>
    <div class="flex flex-col gap-3 bg-opacity-100 w-32 h-96 absolute right-0 top-0 bottom-0 my-auto border-black border-2 rounded-md px-2 py-4 mr-0.5 z-50">
        <button on:click={()=>{fillColour = "rgb(234, 88, 12)"; lineSelected = false;}} class="self-center bg-orange-600 rounded-full w-14 h-14 border-black border-[1px]">
        </button>
        <button on:click={()=>{fillColour = "rgb(22 163 74)"; lineSelected = false;}} class="self-center bg-green-600 rounded-full w-14 h-14 border-black border-[1px]">
        </button>
        <button on:click={()=>{lineSelected = true;}} class="self-center w-10 border-black border-2">
        </button>
        <button on:click={()=>{elements.pop(); draw();}}>
            Undo
        </button>
    </div>
</div>
<a href="/">Home</a>