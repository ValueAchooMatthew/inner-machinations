<script lang="ts">
  import { can_redo, can_undo, selection, tool } from "$lib/utils/automataStores";
  import { saveNow } from "$lib/workbench/autosave";
  import { deleteSelection, redo, requestTrash, selectTool, undo } from "$lib/workbench/actions";
</script>

<nav class="text-center select-none flex flex-col justify-between self-center gap-3 bg-opacity-100 w-32 h-fit border-black border-2 bg-white rounded-md px-2 py-4 mr-0.5 z-10 transition-all duration-300">
  <div class="flex flex-col gap-2">
    <button class="flex flex-col self-center"
      class:ring-2={$tool === "start"}
      on:click={() => selectTool("start")}
      style="line-height: 15px;">
      New Start State
      <div class="mt-2 self-center bg-green-600 rounded-full w-14 h-14 border-black border">
      </div>
    </button>
    <button class="flex flex-col self-center"
      class:ring-2={$tool === "state"}
      on:click={() => selectTool("state")}>
      New State
      <div class="self-center bg-orange-600 rounded-full w-14 h-14 border-black border">
      </div>
    </button>
    <button class="flex flex-col self-center"
      class:ring-2={$tool === "final"}
      on:click={() => selectTool("final")}
      style="line-height: 15px;">
      New Final State
      <div class="bg-white mt-2 self-center rounded-full w-18 h-18 border-black border">
      </div>
    </button>
    <button class="flex flex-col"
      class:ring-2={$tool === "connection"}
      on:click={() => selectTool("connection")}
      style="line-height: 15px;">
      New Connection
      <svg
        class="hover:cursor-pointer w-10 self-center"
        data-slot="icon"
        fill="none"
        stroke-width="1.5"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3"
        ></path>
      </svg>
    </button>
    <button
      class:ring-2={$tool === "epsilon"}
      on:click={() => selectTool("epsilon")}
      class="flex flex-col"
      style="line-height: 15px;">
      New Epsilon Connection
      <br />
      <span class=" -mb-3 self-center"> ϵ </span>
      <svg
        class="hover:cursor-pointer w-10 self-center"
        data-slot="icon"
        fill="none"
        stroke-width="1.5"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3"
        ></path>
      </svg>
    </button>
  </div>
  <div class="flex justify-around mt-2">
    <button class="w-7 h-7 rounded-sm disabled:opacity-30" title="Undo (Ctrl+Z)" disabled={!$can_undo} on:click={undo}>
      <svg fill="none" stroke="currentColor" stroke-width="1.8" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" d="M9 15 3 9m0 0 6-6M3 9h12a6 6 0 0 1 0 12h-3"></path>
      </svg>
    </button>
    <button class="w-7 h-7 rounded-sm disabled:opacity-30" title="Redo (Ctrl+Shift+Z)" disabled={!$can_redo} on:click={redo}>
      <svg fill="none" stroke="currentColor" stroke-width="1.8" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" d="m15 15 6-6m0 0-6-6m6 6H9a6 6 0 0 0 0 12h3"></path>
      </svg>
    </button>
    <button class="w-7 h-7 rounded-sm disabled:opacity-30" title="Delete the selected state or connection (Delete)" disabled={!$selection} on:click={deleteSelection}>
      <svg fill="none" stroke="currentColor" stroke-width="1.8" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9.75 14.25 12m0 0 2.25 2.25M14.25 12l2.25-2.25M14.25 12 12 14.25m-2.58 4.92-6.374-6.375a1.125 1.125 0 0 1 0-1.59L9.42 4.83c.21-.211.497-.33.795-.33H19.5a2.25 2.25 0 0 1 2.25 2.25v10.5a2.25 2.25 0 0 1-2.25 2.25h-9.284c-.298 0-.585-.119-.795-.33Z"></path>
      </svg>
    </button>
  </div>
  <div class="flex justify-around">
    <button class="w-7 h-7 rounded-sm disabled:opacity-30" title="Select, move and reshape" class:ring-2={$tool === "select"} on:click={() => selectTool("select")}>
      <svg fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <path clip-rule="evenodd" fill-rule="evenodd" d="M6.672 1.911a1 1 0 10-1.932.518l.259.966a1 1 0 001.932-.518l-.26-.966zM2.429 4.74a1 1 0 10-.517 1.932l.966.259a1 1 0 00.517-1.932l-.966-.26zm8.814-.569a1 1 
            0 00-1.415-1.414l-.707.707a1 1 0 101.415 1.415l.707-.708zm-7.071 7.072l.707-.707A1 1 0 003.465 9.12l-.708.707a1 1 0 001.415 1.415zm3.2-5.171a1 1 0 00-1.3 1.3l4 10a1 1 0 001.823.075l1.38-2.759 3.018 3.02a1 1 0 001.414-1.415l-3.019-3.02 2.76-1.379a1 1 0 00-.076-1.822l-10-4z"></path>
      </svg>
    </button>
    <button class="w-7 h-7 rounded-sm disabled:opacity-30" title="Clear the canvas" on:click={requestTrash}>
      <svg fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <path clip-rule="evenodd" fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1 1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991 2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 0 1-.256-1.478A48.567 48.567 
            0 0 1 7.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0 1 3.369 0c1.603.051 2.815 1.387 2.815 2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6 0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0 1 0-1.5.058l.347 9a.75.75 0 1 0 1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347 9a.75.75 0 0 0 1.5.058l.345-9Z"></path>
      </svg>
    </button>
    <button class="w-7 h-7 rounded-sm" title="Save now (Ctrl+S)" on:click={() => saveNow().catch(() => {})}>
      <svg fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
        <path clip-rule="evenodd" fill-rule="evenodd" d="M5.478 5.559A1.5 1.5 0 0 1 6.912 4.5H9A.75.75 0 0 0 9 3H6.912a3 3 0 0 0-2.868 2.118l-2.411 7.838a3 3 0 0 0-.133.882V18a3 3 0 0 0 3 3h15a3 3 0 0 0 3-3v-4.162c0-.299-.045-.596-.133-.882l-2.412-7.838A3 3 0 0 0 17.088 3H15a.75.75 0 0 0 0 1.5h2.088a1.5 1.5 0 0 1 1.434 1.059l2.213 7.191H17.89a3 3 0 0 0-2.684 1.658l-.256.513a1.5 1.5 0 0 1-1.342.829h-3.218a1.5 1.5 0 0 1-1.342-.83l-.256-.512a3 3 0 0 0-2.684-1.658H3.265l2.213-7.191Z"></path>
        <path clip-rule="evenodd" fill-rule="evenodd" d="M12 2.25a.75.75 0 0 1 .75.75v6.44l1.72-1.72a.75.75 0 1 1 1.06 1.06l-3 3a.75.75 0 0 1-1.06 0l-3-3a.75.75 0 0 1 1.06-1.06l1.72 1.72V3a.75.75 0 0 1 .75-.75Z"></path>
      </svg>
    </button>
  </div>
</nav>
