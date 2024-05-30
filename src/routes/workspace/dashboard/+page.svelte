<script lang="ts">
  import { onMount } from "svelte";
  import Banner from "./Banner.svelte";
  import Card from "./Card.svelte";
  import { invoke } from "@tauri-apps/api";
  import SignOut from "./SignOut.svelte";

  let saved_workspace_names: Array<String> = new Array();

  export let data;
  
  onMount(async () => {
    if(!data || !data.email){
      return;
    }
    saved_workspace_names = await invoke("get_users_saved_workspaces", {email: data.email});
  })

</script>

<Banner/>
<h1 class="text-center mt-40">
  <div class="flex gap-20 justify-center h-fit">
    {#if saved_workspace_names}
      {#each saved_workspace_names as workspace, _}
        <Card workspace_name={workspace}/>
      {/each}
    {:else}
      <Card workspace_name={"gabby"}/>
    {/if}
  </div>
  <div class="absolute right-12 bottom-12">
    <SignOut/>
  </div>
</h1>