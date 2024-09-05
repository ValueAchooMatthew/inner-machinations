<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import TestFeedback from "../workbench/TestFeedback.svelte"
  import Banner from "./Banner.svelte";
  import { drawParseTree } from "$lib/utils/drawingFuncs";
  import type { Token } from "$lib/types/types";

  let was_string_accepted: boolean | null = null;
  let regex: string = "";
  let string_to_test: string = "";
  let canvas: HTMLCanvasElement | undefined;
  const width = 1000;
  const height = 600;

  async function processRegex(regex: string, string_to_test: string) {
    const context = canvas?.getContext("2d");
    if (!context) {
      return false;
    }
    context.clearRect(0, 0, width, height);
    const accepted: boolean = await invoke("test_string_regex", {regex: regex, stringToCheck: string_to_test});
    const parse_tree: Token = await invoke("build_parse_tree", {regex: regex});
    drawParseTree(parse_tree, context, {x: width/2, y: 50});
    was_string_accepted = accepted;
    return accepted;
  }


  $: {
    processRegex(regex, string_to_test);
  }

  function handleUpdatingRegex(event: Event & {currentTarget: EventTarget & HTMLInputElement}): void {
    regex = event.currentTarget.value;
  }

  function handleStringChecking(event: Event & {currentTarget: EventTarget & HTMLInputElement}): void {
    string_to_test = event.currentTarget.value;
  }

</script>

<Banner />
<div class="flex flex-col justify-start h-full p-12 font-semibold bg-gray-200 min-h-screen">
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
    <TestFeedback is_string_accepted={was_string_accepted} />
  </div>
    <canvas class="self-center border-black border-2 rounded-md mx-2 my-2 bg-white mr-0 flex-shrink-0"
      style={`width: ${width}px; height: ${height}px;`}
      {width}
      {height}
      bind:this={canvas}>
    </canvas>
    
</div>
