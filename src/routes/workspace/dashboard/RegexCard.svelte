<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { drawParseTree } from "$lib/utils/drawingFuncs";
  import { chooseRegex } from "$lib/regexes/savedRegex";
  import type { RegexData, SavedItem } from "$lib/types/interfaces";
  import type { Token } from "$lib/types/types";

  export let saved_regex: RegexData;
  export let to_delete: SavedItem | null;

  const handleClick = () => {
    chooseRegex(saved_regex.regex_name);
    goto("/workspace/regexes");
  };

  let canvas: HTMLCanvasElement | null;

  onMount(async () => {
    const context = canvas?.getContext("2d");
    const width = canvas?.width;
    const height = canvas?.height;
    if (!context || !width || !height) {
      return;
    }
    try {
      const parse_tree = await invoke<Token>("build_parse_tree", { regex: saved_regex.regex });
      // The card shows only the middle of its canvas
      drawParseTree(parse_tree, context, width, height, { margin: 30 });
    } catch {
      // One that can't be built yet has no preview, only the text below
    }
  });
</script>

<div class="flex transform transition-transform duration-300 hover:scale-105 hover:shadow-2xl shadow-lg ">
  <button class="w-64 h-80 p-3 bg-orange-500
    transition-all duration-300 rounded-md flex flex-col justify-between overflow-hidden"
    on:click={handleClick}>
    <canvas
    bind:this={canvas} class="bg-white self-center border-black border w-fit"></canvas>
    <span class="font-bold text-white text-2xl my-2 self-center max-w-full">
      {saved_regex.regex_name}
      <span class="block font-mono text-lg truncate" title={saved_regex.regex}>
        {saved_regex.regex || "Empty"}
      </span>
      <span class="text-sm">
        Last updated: {saved_regex.date_of_last_update}
      </span>
    </span>
  </button>
  <button class="absolute flex justify-center bg-orange-500 rounded-r-md w-12 h-12"
    title="Delete {saved_regex.regex_name}"
    on:click={() => {
      to_delete = { kind: "regex", name: saved_regex.regex_name };
    }}>
    <svg
      class="w-8 h-8 self-center"
      data-slot="icon"
      fill="none"
      stroke-width="1.5"
      stroke="white"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true">
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0">
      </path>
    </svg>
  </button>
</div>
