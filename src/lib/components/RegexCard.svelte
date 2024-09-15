<script lang="ts">
  import { goto } from "$app/navigation";
  import type { RegexWorkspaceData } from "$lib/types/interfaces";
  import type { Token } from "$lib/types/types";
  import { current_workspace_type, dialogue_to_user, email, workspace_name } from "$lib/utils/svelteStores";
  import { drawRegexParseTree, get_dimensions_of_parse_tree } from "$lib/utils/drawingFuncs";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import { WorkspaceType } from "$lib/types/enums";

  export let workspace_title: string;
  export let workspace_to_delete: string | null;

  let canvas: HTMLCanvasElement | undefined;
  let workspace_data: RegexWorkspaceData;

  const x_distance_of_children = 150;
  const y_distance_of_children = 100;
  const shrink_factor = 2;

  onMount(async () => {
    // Clean up in future
    workspace_data = await invoke("retrieve_regex_workspace_data", {email: $email, regex_name: workspace_title});
    await invoke("build_parse_tree", {regex: workspace_data.regex}).then((data) => {
      const context = canvas?.getContext("2d");
      if(!context || !canvas) {
        return;
      }

      const parse_tree = data as Token;

      let [necessary_width, necessary_height] = get_dimensions_of_parse_tree(
        parse_tree, 
        x_distance_of_children, 
        y_distance_of_children, 
        shrink_factor
      );

      const scale_factor = Math.min(Math.max(necessary_width/canvas.clientWidth, necessary_height/canvas.clientHeight), 5);

      context.scale(1/scale_factor, 1/scale_factor);

      drawRegexParseTree(
        parse_tree,
        context,
        {x: canvas.width*scale_factor/2, y: 50*scale_factor},
        x_distance_of_children,
        y_distance_of_children,
        shrink_factor
      );

    }).catch((e)=> {
      console.log(e);
    });
  });

  async function handleClick() {
    current_workspace_type.set(
      WorkspaceType.Regex
    );
    document.cookie =
      "workspace_name=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    document.cookie = "workspace_name" + "=" + workspace_title + "; path=/";
    workspace_name.set(
      workspace_title
    );
    dialogue_to_user.set(
      ""
    );
    await goto("regexes");
  }
</script>

<div class="flex transform transition-transform duration-300 hover:scale-105 hover:shadow-2xl shadow-lg ">
  <button class="w-64 h-80 p-3 bg-orange-500
    transition-all duration-300 rounded-md flex flex-col justify-between overflow-hidden shadow-md"
    on:click={handleClick}>
    <canvas
    bind:this={canvas} class="bg-white self-center shadow-sm w-fit" />
    <span class="font-bold text-white text-2xl my-2 self-center">
      {workspace_title}
      <br>
      {#if workspace_data}
        <span class="text-base">
        </span>
        <br>
        <span class="text-sm">
          Last updated: {workspace_data.date_of_last_update}
        </span>
      {/if}
    </span>
  </button>
  <button class="absolute flex justify-center bg-orange-500 rounded-r-md w-12 h-12"
    on:click={() => {
      current_workspace_type.set(
        WorkspaceType.Regex
      );
      workspace_to_delete = workspace_title;
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


