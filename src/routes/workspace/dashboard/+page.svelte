<script lang="ts">
  import Banner from "./Banner.svelte";
  import Card from "./Card.svelte";
  import { invoke } from "@tauri-apps/api";
  import SignOut from "./SignOut.svelte";
  import ConfirmDelete from "./ConfirmDelete.svelte";
  import { dialogue_to_user, email, workspace_name } from "$lib/utils/automataStores";
  import Notifications from "$lib/components/Notifications.svelte";
  import { goto } from "$app/navigation";

  let saved_workspace_names: Array<string> = new Array();
  let workspace_to_delete: string | null = null;

  const getWorkspaces = async (): Promise<Array<string>> => {
    return await invoke("get_users_saved_workspaces", {email: $email});
  }

  $: {
    // Done to trigger a re-retrieval of the database whenever a workspace is deleted
    workspace_to_delete = workspace_to_delete;
    getWorkspaces();
  }

  async function handleClick() {
    document.cookie =
    "workspace_name=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    const does_workspace_exist = await invoke("does_workspace_name_exist", {workspaceName: "Untitled Project", email: $email})
    if(does_workspace_exist) {
      dialogue_to_user.set("Untitled Project already exists")
      return;
    } 
    
    document.cookie = "workspace_name" + "=" + "Untitled Project" + "; path=/";
    workspace_name.set(
      "Untitled Project"
    )
    dialogue_to_user.set(null);
    await invoke("create_workspace", {email: $email, workspaceName: "Untitled Project"});
    goto("/workspace/workbench");
  }

</script>

<Banner/>
<div class="py-24 px-12 relative h-screen w-full flex flex-col">
  <div class="flex w-[23rem] bg-orange-500 my-6 px-6 py-2 rounded-md text-gray-50 justify-between shadow-sm">
    <span class="font-bold text-4xl self-center">
      NFA's/DFA's
    </span>
    <button class="flex flex-col justify-center" on:click={handleClick}>
      <svg class="ml-2 hover:cursor-pointer hover:-rotate-90 w-10 h-10 transition-all duration-200 self-center"
        data-slot="icon"
        fill="none"
        stroke-width="2.2"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 4.5v15m7.5-7.5h-15">
        </path>
      </svg>
      <span class=" text-xs">
        Create New
      </span>
    </button>
  </div>
  <div class="flex flex-wrap justify-center gap-12 max-w-[80rem] mx-auto text-center">
    {#await getWorkspaces() then saved_workspace_names}
      {#each saved_workspace_names as workspace_title, _}
        <Card workspace_title={workspace_title} bind:workspace_to_delete={workspace_to_delete}/>
      {/each}
    {/await}
  </div>
  <div class="flex bg-orange-500 w-[23rem] my-6 px-6 py-2 rounded-md text-gray-50 gap-12 justify-between shadow-sm">
    <span class="font-bold text-4xl self-center">
      Regexes
    </span>
    <a class="flex flex-col justify-center" href="./regexes">
      <svg class="ml-2 hover:cursor-pointer hover:-rotate-90 w-10 h-10 transition-all duration-200 self-center"
        data-slot="icon"
        fill="none"
        stroke-width="2.2"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 4.5v15m7.5-7.5h-15">
        </path>
      </svg>
      <span class=" text-xs">
        Create New
      </span>
    </a>
  </div>

  <div class="fixed bottom-12 right-12">
    <SignOut/>
  </div>
  <ConfirmDelete bind:workspace_to_delete={workspace_to_delete}/>
  <!-- <a href="./regexes">Go To regexes</a> -->
  <div class="flex justify-center mt-24">
    <Notifications />
  </div>
</div>
