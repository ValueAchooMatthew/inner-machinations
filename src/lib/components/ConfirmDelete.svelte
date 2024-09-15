<script lang="ts">
  import { WorkspaceType } from "$lib/types/enums";
  import { current_workspace_type, email, workspace_name } from "$lib/utils/svelteStores";
  import { invoke } from "@tauri-apps/api";
  export let workspace_to_delete: string | null;

  const handleDelete = async () => {
    if (workspace_to_delete === null) {
      return;
    }
    switch ($current_workspace_type) {
      case WorkspaceType.RegularAutomata:
        await invoke("delete_regular_automata_workspace", {
          email: $email,
          workspace_name: workspace_to_delete,
        });
        break;
    
      case WorkspaceType.Regex:
        await invoke("delete_regex_workspace", {
          email: $email,
          workspace_name: workspace_to_delete
        });
        break;
    }

    workspace_to_delete = null;
  };
</script>

<div class="bg-orange-500 text-white font-bold px-4 py-3 rounded-xl fixed transition-all
    duration-300 mx-auto my-auto left-0 right-0 w-fit h-fit bottom-0 border-4 border-white"
  class:top-full={workspace_to_delete === null}
  class:mt-96={workspace_to_delete === null}
  class:top-0={workspace_to_delete !== null}>
  <span class="text-2xl">
    Are you sure you want to delete "{workspace_to_delete}"?
  </span>
  <div class="flex justify-around text-lg">
    <button on:click={handleDelete}> Yes </button>
    <button
      on:click={() => {
        workspace_to_delete = null;
      }}>
      No
    </button>
  </div>
</div>
