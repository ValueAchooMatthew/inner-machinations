<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import TestFeedback from "../workbench/TestFeedback.svelte"
  import Banner from "./Banner.svelte";
  import { drawParseTree } from "$lib/utils/drawingFuncs";
  import { describeFailure } from "$lib/utils/backend";
  import type { Token } from "$lib/types/types";

  let was_string_accepted: boolean | null = null;
  let regex: string = "";
  let string_to_test: string = "";
  // Why the regular expression can't be built, if it can't
  let error_message: string | null = null;
  let canvas: HTMLCanvasElement | undefined;
  const width = 1000;
  const height = 600;

  // Each keystroke starts a new check; only the latest one's results are shown, however the replies arrive
  let latest_check = 0;

  async function processRegex(regex: string, string_to_test: string) {
    const check = ++latest_check;
    const context = canvas?.getContext("2d");
    if (!context) {
      return;
    }
    try {
      const [accepted, parse_tree] = await Promise.all([
        invoke<boolean>("test_string_regex", {regex: regex, stringToCheck: string_to_test}),
        invoke<Token>("build_parse_tree", {regex: regex}),
      ]);
      if (check !== latest_check) {
        return;
      }
      context.clearRect(0, 0, width, height);
      drawParseTree(parse_tree, context, width, height);
      was_string_accepted = accepted;
      error_message = null;
    } catch (error) {
      if (check !== latest_check) {
        return;
      }
      context.clearRect(0, 0, width, height);
      was_string_accepted = null;
      error_message = describeFailure(error);
    }
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
      <p class="text-orange-600 text-xl h-8" id="regex_error">
        {error_message ?? ""}
      </p>
    </form>
    <TestFeedback is_string_accepted={was_string_accepted} />
  </div>
    <canvas class="self-center border-black border-2 rounded-md mx-2 my-2 bg-white mr-0 shrink-0"
      style={`width: ${width}px; height: ${height}px;`}
      {width}
      {height}
      bind:this={canvas}>
    </canvas>
    
</div>
