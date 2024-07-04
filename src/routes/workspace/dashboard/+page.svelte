<script lang="ts">
  import Banner from "./Banner.svelte";
  import Card from "./Card.svelte";
  import { invoke } from "@tauri-apps/api";
  import SignOut from "./SignOut.svelte";
  import ConfirmDelete from "./ConfirmDelete.svelte";
  import { getCookie } from "$lib/miscUtils";

  let saved_workspace_names: Array<string> = new Array();
  let data = {email: getCookie("email")};
  let workspace_to_delete: string | null = null;

  const getWorkspaces = async (data: {email: string}) => {
    if(!data || !getCookie("email")){
      return;
    }
    saved_workspace_names = await invoke("get_users_saved_workspaces", {email: data.email});
  }

  $: {
    // Done to trigger a re-retrieval of the database whenever a workspace is deleted
    workspace_to_delete = workspace_to_delete;
    getWorkspaces(data);
  }
</script>

<Banner/>
<div class="text-center p-48 relative h-screen w-full flex flex-col">
  <div class="flex gap-20 justify-center h-fit">
    {#if saved_workspace_names}
      {#each saved_workspace_names as workspace, _}
        <Card email={data.email} bind:workspace_to_delete={workspace_to_delete} bind:workspace_name={workspace}/>
      {/each}
    {/if}
  </div>
  <div class="fixed bottom-12 right-12">
    <SignOut/>
  </div>
  <ConfirmDelete email={data.email} bind:workspace_to_delete={workspace_to_delete}/>
</div>