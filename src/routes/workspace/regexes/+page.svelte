<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import TestFeedback from "../workbench/TestFeedback.svelte"
  import Banner from "./Banner.svelte";
  import { drawParseTree, get_dimensions_of_parse_tree } from "$lib/utils/drawingFuncs";
  import type { Token } from "$lib/types/types";
  import { onMount, tick } from "svelte";

  let was_string_accepted: boolean | null = null;
  let regex: string = "";
  let string_to_test: string = "";
  let canvas: HTMLCanvasElement | undefined;
  let canvas_wrapper: HTMLDivElement | undefined;
  let width = 1000;
  let height = 600;
  let is_control_button_pressed = false;
  let scale = 1;

  const x_distance_of_children = 256;
  const y_distance_of_children = 175;
  const shrink_factor = 2;

  onMount(() => {
    if(!canvas_wrapper) {
      return;
    }
    width = canvas_wrapper.clientWidth + 5000;
    height = canvas_wrapper.clientHeight + 5000;
  });

  async function processRegex(regex: string, string_to_test: string) {
    const accepted: boolean = await invoke("test_string_regex", {regex: regex, stringToCheck: string_to_test});
    was_string_accepted = accepted;
    return accepted;
  };

  async function updateCanvas(context: CanvasRenderingContext2D, regex: string, scale: number) {

    if(!canvas_wrapper) {
      return;
    }
    
    const parse_tree: Token =  await invoke("build_parse_tree", {regex: regex});

    context.scale(scale, scale);

    await tick();
    context.clearRect(0, 0, width/scale, height/scale);
    drawParseTree(
      parse_tree, 
      context, 
      { x: Math.floor(width/2), y: 200 }, 
      x_distance_of_children, 
      y_distance_of_children, 
      shrink_factor
    );
    
    canvas_wrapper.scrollTo(Math.floor(width/2) - Math.floor(canvas_wrapper.clientWidth/2), Math.floor(100));
  }
  
  $: {
    processRegex(regex, string_to_test);
  }
  
  $: {
    const context = canvas?.getContext("2d");
    if (context) {
      updateCanvas(context, regex, scale).catch((e) => {
        console.log(e);
      });
    }
  }
  
  function handleUpdatingRegex(event: Event & {currentTarget: EventTarget & HTMLInputElement}): void {
    regex = event.currentTarget.value;
  }
  
  function handleStringChecking(event: Event & {currentTarget: EventTarget & HTMLInputElement}): void {
    string_to_test = event.currentTarget.value;
  }

  function handleZoom(event: WheelEvent) {
    if(is_control_button_pressed) {
      if(event.deltaY < 1) {
        scale += event.deltaY * -0.01;
        scale = Math.min(Math.max(0.125, scale), 4);
      }
    }
  }
  
  async function handleControlPressDown(event: KeyboardEvent) {
    await tick();
    if(event.key == "Control") {
      is_control_button_pressed = true;
    }
  }

  async function handleControlPressUp(event: KeyboardEvent) {
    await tick();
    if(event.key == "Control") {
      is_control_button_pressed = false;
    }
  }
  
</script>


<svelte:window on:keydown={handleControlPressDown} on:keyup={handleControlPressUp} />
<div class="h-screen bg-gray-200">
  <Banner />
  <div class="flex py-12 px-10 justify-around gap-10">
    <div class="w-full h-[40rem] overflow-scroll rounded-md border-black border-2 z-50"
      on:wheel={handleZoom}
      bind:this={canvas_wrapper}>
      <canvas class="self-center rounded-md bg-white flex-shrink-0"
        style={`width: ${width}px; height: ${height}px;`}
        {width}
        {height}
        bind:this={canvas}>
      </canvas>
    </div>
    <div class="font-bold flex justify-center">
      <TestFeedback is_string_accepted={was_string_accepted} />
    </div>
  </div>
  <div class="flex justify-center">
    <div class="flex flex-col justify-start p-12 font-semibold bg-gray-200">
      <div class="flex justify-between gap-3 font-semibold text-2xl">
        <div class="w-48 h-24">
        </div>
        <form class="self-center flex flex-col gap-6" action="">
          <div>
            <label for="regex">Build Regular Expression: </label>
            <input class="text-gray-950 bg-white px-2 py-1 rounded-md mt-0.5 overflow-hidden h-12 border-black border-2" on:input={handleUpdatingRegex} id="regex" name="regex" type="text">
          </div>
          <div>
            <label for="string_test">Check String: </label>
            <input class="text-gray-950 bg-white px-2 py-1 rounded-md mt-0.5 overflow-hidden h-12 border-black border-2" on:input={handleStringChecking} id="string_test" name="string_test" type="text">
          </div>
        </form>
      </div>
    </div>
  </div>

</div>
