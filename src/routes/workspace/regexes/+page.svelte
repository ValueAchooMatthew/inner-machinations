<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import TestFeedback from "../workbench/TestFeedback.svelte"
  import Banner from "./Banner.svelte";
  import { drawRegexParseTree, get_dimensions_of_parse_tree } from "$lib/utils/drawingFuncs";
  import type { Token } from "$lib/types/types";
  import { onMount } from "svelte";
  import type { RegexWorkspaceData } from "$lib/types/interfaces";
  import { current_regex } from "$lib/utils/regexStores";
  import { email, workspace_name } from "$lib/utils/userStores";

  let was_string_accepted: boolean | null = null;
  let regex: string = "";
  let string_to_test: string = "";
  let canvas: HTMLCanvasElement | undefined;
  let canvas_wrapper: HTMLDivElement | undefined;
  let width = 1000;
  let height = 600;

  const x_distance_of_children = 256;
  const y_distance_of_children = 175;
  const shrink_factor = 2;

  onMount(async () => {
    if(!canvas_wrapper) {
      return;
    }
    width = canvas_wrapper.clientWidth;
    height = canvas_wrapper.clientHeight;

    const workspace_data: RegexWorkspaceData = await invoke("retrieve_regex_workspace_data", {email: $email, regex_name: $workspace_name});

    current_regex.set(
      workspace_data.regex
    );

    regex = workspace_data.regex;
  });

  async function processRegex(regex: string, string_to_test: string) {
    const accepted: boolean = await invoke("test_string_regex", {regex: regex, string_to_check: string_to_test});
    was_string_accepted = accepted;
    return accepted;
  };

  async function updateCanvas(context: CanvasRenderingContext2D, regex: string) {

    if(!canvas_wrapper) {
      return;
    }
    const parse_tree: Token =  await invoke("build_parse_tree", {regex: regex});
    let [necessary_width, _] = get_dimensions_of_parse_tree(
      parse_tree, 
      x_distance_of_children, 
      y_distance_of_children, 
      shrink_factor
    );

    const scale_factor = necessary_width/canvas_wrapper.clientWidth;

    context.clearRect(0, 0, width, height);
    context.save();
    context.scale(1/scale_factor, 1/scale_factor);

    drawRegexParseTree(
      parse_tree, 
      context, 
      { x: Math.floor(width*scale_factor/2), y: 200*scale_factor }, 
      x_distance_of_children, 
      y_distance_of_children, 
      shrink_factor
    );
    context.restore();
    canvas_wrapper.scrollTo(Math.floor(width/2) - Math.floor(canvas_wrapper.clientWidth/2), 100);
  }
  
  $: {
    current_regex.set(
      regex
    );
    processRegex(regex, string_to_test);
  }
  
  $: {
    const context = canvas?.getContext("2d");
    if (context) {
      updateCanvas(context, regex).catch((e) => {
        console.log(e);
      });
    }
  }
  
  async function handleUpdatingRegex(event: Event & {currentTarget: EventTarget & HTMLInputElement}) {
    regex = event.currentTarget.value;
    await invoke("save_regex_workspace", {regex: regex, regex_name: $workspace_name, email: $email});
  }
  
  function handleStringChecking(event: Event & {currentTarget: EventTarget & HTMLInputElement}) {
    string_to_test = event.currentTarget.value;
  }

</script>

<div class="h-screen bg-gray-200">
  <Banner/>
  <div class="flex py-12 px-10 justify-around gap-10">
    <div class="w-full h-[40rem] rounded-md border-black border-2 z-50"
      bind:this={canvas_wrapper}>
      <canvas class="self-center rounded-md bg-white flex-shrink-0"
        style={`width: ${width}px; height: ${height}px;`}
        {width}
        {height}
        bind:this={canvas}>
      </canvas>
    </div>
    <div class="font-semibold flex justify-center">
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
            <input class="text-gray-950 bg-white px-2 py-1 rounded-md mt-0.5 overflow-hidden h-12 border-black border-2" on:input={handleUpdatingRegex} id="regex" name="regex" type="text" value={regex}>
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
