<script lang="ts">
  import Banner from "./Banner.svelte";
  import Group from "./Group.svelte";
  import Card from "./Card.svelte";
  import RegexCard from "./RegexCard.svelte";
  import ConfirmDelete from "./ConfirmDelete.svelte";
  import Notifications from "$lib/components/Notifications.svelte";
  import { goto } from "$app/navigation";
  import { invokeCommand } from "$lib/utils/backend";
  import { firstFreeName, setSessionValue } from "$lib/utils/miscUtils";
  import { dialogue_to_user, workspace_name } from "$lib/utils/automataStores";
  import { createRegex } from "$lib/regexes/savedRegex";
  import type { RegexData, SavedItem } from "$lib/types/interfaces";

  // Null until they've arrived, so neither group says it's empty before then
  let saved_workspace_names: Array<string> | null = null;
  let saved_regexes: Array<RegexData> | null = null;
  let to_delete: SavedItem | null = null;

  const getSaved = async () => {
    [saved_workspace_names, saved_regexes] = await Promise.all([
      invokeCommand<Array<string>>("get_saved_workspaces"),
      invokeCommand<Array<RegexData>>("get_saved_regexes"),
    ]);
  }

  $: {
    // Done to trigger a re-retrieval of the database whenever something is deleted
    to_delete = to_delete;
    getSaved();
  }

  // A new workspace is called "Untitled Project", numbered if that name is taken
  async function createWorkspace() {
    const saved_workspace_names: Array<string> = await invokeCommand("get_saved_workspaces");
    const new_workspace_name = firstFreeName("Untitled Project", saved_workspace_names);
    await invokeCommand("create_workspace", {workspaceName: new_workspace_name});
    setSessionValue("workspace_name", new_workspace_name);
    workspace_name.set(new_workspace_name);
    dialogue_to_user.set(null);
    goto("/workspace/workbench");
  }

  async function createAndOpenRegex() {
    await createRegex();
    goto("/workspace/regexes");
  }
</script>

<Banner/>
<div class="text-center pt-40 pb-24 px-12 relative min-h-screen w-full flex flex-col gap-20">
  <Group title="Automata" create_label="New Automaton" create={createWorkspace}>
    {#if saved_workspace_names}
      <!-- Keyed, so each card keeps the preview it drew when a workspace before it is deleted -->
      {#each saved_workspace_names as workspace_title (workspace_title)}
        <Card bind:to_delete={to_delete} {workspace_title}/>
      {:else}
        <p class="text-xl text-gray-600">No saved automata yet</p>
      {/each}
    {/if}
  </Group>
  <Group title="Regular Expressions" create_label="New Regular Expression" create={createAndOpenRegex}>
    {#if saved_regexes}
      {#each saved_regexes as saved_regex (saved_regex.regex_name)}
        <RegexCard bind:to_delete={to_delete} {saved_regex}/>
      {:else}
        <p class="text-xl text-gray-600">No saved regular expressions yet</p>
      {/each}
    {/if}
  </Group>
  <ConfirmDelete bind:to_delete={to_delete}/>
  <div class="flex justify-center">
    <Notifications />
  </div>
</div>
