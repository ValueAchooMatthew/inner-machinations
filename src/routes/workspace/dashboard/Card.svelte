<script lang="ts">
  import { goto } from "$app/navigation";
  import { draw } from "$lib/drawingFuncs";
  import type { TauriGeneratedAutomataInformation } from "$lib/types";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";

  export let workspace_name: string;
  export let workspace_to_delete: string | null;
  export let email: string | null;

  const handleClick = () => {
    document.cookie =
      "workspace_name=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    document.cookie = "workspace_name" + "=" + workspace_name + "; path=/";
    goto("workbench");
  };

  let canvas: HTMLCanvasElement | null;
  let retrieved_data: TauriGeneratedAutomataInformation;

  onMount(async () => {
    const context = canvas?.getContext("2d");
    const width = canvas?.width;
    const height = canvas?.height;
    if (!context || !width || !height) {
      return;
    }
    retrieved_data =
    await invoke("retrieve_workspace_data", {
      email: email,
      workspaceName: workspace_name,
    });

    draw(
      context,
      width,
      height,
      retrieved_data[1],
      retrieved_data[2],
      retrieved_data[0],
      null,
      5
    );
    
  });
</script>

<div class="flex">
  <button class="w-64 h-80 hover:w-80 hover:h-96 p-3 bg-orange-500
    transition-all duration-300 rounded-md shadow-2xl flex flex-col justify-between overflow-hidden"
    on:click={handleClick}>
    <canvas bind:this={canvas} class=" bg-white self-center" />
    <span class="font-bold text-white text-2xl my-2 self-center">
      {workspace_name}
      <br>
      {#if retrieved_data}
        <span class="text-sm">
          Last updated: {retrieved_data[5]}
        </span>
      {/if}
    </span>

  </button>
  <button class="absolute flex justify-center bg-orange-500 rounded-r-md w-12 h-12"
    on:click={() => {
      workspace_to_delete = workspace_name;
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
        d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
      ></path>
    </svg>
  </button>
</div>
