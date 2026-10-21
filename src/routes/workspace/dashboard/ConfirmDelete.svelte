<script lang="ts">
  import { invokeCommand } from "$lib/utils/backend";
  import type { SavedItem } from "$lib/types/interfaces";

  export let to_delete: SavedItem | null;

  // Still named while the dialog slides away
  let shown_name = "";
  $: if (to_delete) {
    shown_name = to_delete.name;
  }

  const handleDelete = async () => {
    if (to_delete === null) {
      return;
    }
    if (to_delete.kind === "workspace") {
      await invokeCommand("delete_workspace", { workspaceName: to_delete.name });
    } else {
      await invokeCommand("delete_regex", { regexName: to_delete.name });
    }
    to_delete = null;
  };
</script>

<div class="bg-orange-500 text-white font-bold px-4 py-3 rounded-md fixed transition-all
    duration-300 mx-auto my-auto left-0 right-0 w-fit h-fit bottom-0 border-2 border-black"
  class:top-full={to_delete === null}
  class:mt-96={to_delete === null}
  class:top-0={to_delete !== null}>
  <span class="text-2xl">
    Are you sure you want to delete "{shown_name}"?
  </span>
  <div class="flex justify-around text-lg">
    <button on:click={handleDelete}> Yes </button>
    <button
      on:click={() => {
        to_delete = null;
      }}>
      No
    </button>
  </div>
</div>
