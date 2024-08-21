<script lang="ts">
  import Banner from "./Banner.svelte";
  import Card from "./Card.svelte";
  import { invoke } from "@tauri-apps/api";
  import SignOut from "./SignOut.svelte";
  import ConfirmDelete from "./ConfirmDelete.svelte";
  import { email } from "$lib/utils/automataStores";
  import Notifications from "$lib/components/Notifications.svelte";

  let saved_workspace_names: Array<string> = new Array();
  let workspace_to_delete: string | null = null;

  const getWorkspaces = async () => {
    saved_workspace_names = await invoke("get_users_saved_workspaces", {email: $email});
  }

  $: {
    // Done to trigger a re-retrieval of the database whenever a workspace is deleted
    workspace_to_delete = workspace_to_delete;
    getWorkspaces();
  }
</script>

<Banner/>
<div class="text-center p-48 relative h-screen w-full flex flex-col">
  <div class="flex flex-wrap justify-center gap-12 max-w-[80rem] mx-auto">
    {#if saved_workspace_names}
      {#each saved_workspace_names as workspace_title, _}
        <Card bind:workspace_to_delete={workspace_to_delete} bind:workspace_title={workspace_title}/>
      {/each}
    {/if}
  </div>
  <div class="fixed bottom-12 right-12">
    <SignOut/>
  </div>
  <ConfirmDelete bind:workspace_to_delete={workspace_to_delete}/>
  <a href="./regexes">Go To regexes</a>
  <div class="flex justify-center mt-24">
    <Notifications />
  </div>
</div>
