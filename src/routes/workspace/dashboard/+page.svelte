<script lang="ts">
  import Banner from "./Banner.svelte";
  import RegularAutomataCard from "../../../lib/components/RegularAutomataCard.svelte";
  import { invoke } from "@tauri-apps/api";
  import SignOut from "./SignOut.svelte";
  import ConfirmDelete from "../../../lib/components/ConfirmDelete.svelte";
  import Notifications from "$lib/components/Notifications.svelte";
  import WorkspaceGroup from "$lib/components/WorkspaceGroup.svelte";
  import RegexCard from "$lib/components/RegexCard.svelte";
  import { email } from "$lib/utils/userStores";

  let workspace_to_delete: string | null = null;

  let regular_automata_workspaces: Promise<Array<string>> = invoke("get_users_regular_automata_workspace_names", {email: $email});
  let regex_workspaces: Promise<Array<string>> = invoke("get_users_regex_workspace_names", {email: $email});

  $: {
    // Done to trigger a re-retrieval of the database whenever a workspace is deleted
    workspace_to_delete = workspace_to_delete;
    regular_automata_workspaces = invoke("get_users_regular_automata_workspace_names", {email: $email});
    regex_workspaces = invoke("get_users_regex_workspace_names", {email: $email});
  }
</script>

<Banner/>
<div class="py-24 px-12 relative h-screen w-full flex flex-col">
  <WorkspaceGroup
    workspace_group_name="NFA's/DFA's"
    redirect="workbench"
    create_new_untitled_project = {async (email) => {
    return invoke("create_regular_automata_workspace", {email: email});
  }}/>
  <div class="flex flex-wrap justify-center gap-12 max-w-[80rem] mx-auto text-center">
    {#await regular_automata_workspaces then saved_regular_automata_workspace_names}
      {#each saved_regular_automata_workspace_names as workspace_title, _}
        <RegularAutomataCard workspace_title={workspace_title} bind:workspace_to_delete={workspace_to_delete}/>
      {/each}
    {/await}
  </div>
  <WorkspaceGroup
    workspace_group_name="Regexes"
    redirect="regexes"
    create_new_untitled_project = {async (email) => { 
    return invoke("create_regex_workspace", {email: email});
    }}/>
  <div class="flex flex-wrap justify-center gap-12 max-w-[80rem] mx-auto text-center">
    {#await regex_workspaces then saved_regex_workspace_names}
      {#each saved_regex_workspace_names as workspace_title, _}
        <RegexCard workspace_title={workspace_title} bind:workspace_to_delete={workspace_to_delete}/>
      {/each}
    {/await}
  </div>
  <div class="fixed bottom-12 right-12">
    <SignOut/>
  </div>
  <ConfirmDelete bind:workspace_to_delete={workspace_to_delete}/>
  <div class="flex justify-center mt-24">
    <Notifications />
  </div>
</div>
