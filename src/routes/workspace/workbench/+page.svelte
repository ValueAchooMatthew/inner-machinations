<script lang="ts">
  import OptionsMenu from "./OptionsMenu.svelte";
  import Whiteboard from "./Whiteboard.svelte";
  import Banner from "./Banner.svelte";
  import Notifications from "$lib/components/Notifications.svelte";
  import { onDestroy, onMount } from "svelte";
  import { invokeCommand } from "$lib/utils/backend";
  import { should_show_string_traversal, workspace_name } from "$lib/utils/automataStores";
  import { checkString, loadWorkspace } from "$lib/workbench/actions";
  import { saveNow, startAutosave, stopAutosave } from "$lib/workbench/autosave";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import type { WorkspaceData } from "$lib/types/interfaces";
  import TestFeedback from "./TestFeedback.svelte";
  import AdvancedAutomataFunctions from "./AdvancedAutomataFunctions.svelte";
  import Sidebar from "./Sidebar.svelte";
  import LanguageOfAutomata from "./LanguageOfAutomata.svelte";
  import ConfirmTrash from "./ConfirmTrash.svelte";

  let is_string_accepted: boolean | null = null;

  let highlighted_state_id: number | null = null;
  let is_option_menu_open: boolean = false;

  // Closing the window waits for unsaved changes to be saved. The window closes either way: a failed save
  // can't be put right by keeping the user from leaving
  let stopWaitingForClose: (() => void) | undefined;

  onMount(async () => {
    const workspace_data: WorkspaceData = await invokeCommand("retrieve_workspace_data", {
      workspaceName: $workspace_name,
    });
    loadWorkspace(workspace_data);
    startAutosave();
    stopWaitingForClose = await getCurrentWindow().onCloseRequested(async () => {
      await saveNow().catch(() => {});
    });
  });

  onDestroy(() => {
    stopAutosave();
    stopWaitingForClose?.();
  });

  // The traversal being shown, if any, so a new check or leaving the page can stop it
  let traversal: ReturnType<typeof setInterval> | undefined;
  let tick: HTMLAudioElement | undefined;

  const stopTraversal = (): void => {
    clearInterval(traversal);
    traversal = undefined;
    highlighted_state_id = null;
  };

  onDestroy(stopTraversal);

  const handleStringInput = async (event: SubmitEvent) => {
    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }
    // An empty box checks the empty string
    const string_to_check = new FormData(event.target).get("string")?.toString() ?? "";
    stopTraversal();
    is_string_accepted = null;

    const result = await checkString(string_to_check);
    if (!$should_show_string_traversal || result.states_traversed.length === 0) {
      is_string_accepted = result.is_string_accepted;
      return;
    }
    showTraversal(result.states_traversed, result.is_string_accepted);
  };

  // Lights up each state the string passed through in turn, with a tick, then gives the answer
  const showTraversal = (states_traversed: Array<number>, accepted: boolean | null) => {
    tick ??= new Audio("/metronome-85688.mp3");
    let step = 0;
    traversal = setInterval(() => {
      // Every other step clears the highlight, so passing through the same state twice can be seen
      if (step === 2 * states_traversed.length) {
        stopTraversal();
        is_string_accepted = accepted;
      } else if (step % 2 === 1) {
        highlighted_state_id = null;
      } else {
        highlighted_state_id = states_traversed[step / 2];
        tick?.play().catch(() => {});
      }
      step++;
    }, 300);
  };

</script>

<div class="relative flex font-semibold w-full h-full bg-gray-200 min-h-screen">
  <aside class=" bg-orange-500 flex flex-col top-0
    absolute transition-all duration-300 overflow-hidden z-50 w-full h-full"
    class:left-0={is_option_menu_open}
    class:-left-full={!is_option_menu_open}>
    <OptionsMenu
      bind:is_option_menu_open/>
  </aside>
  <div class="w-full min-w-0">
    <Banner
      bind:is_option_menu_open/>
    <main>
      <div class="w-full h-fit font-semibold flex align-middle justify-around">
        <div class="flex flex-col">
          <Whiteboard {highlighted_state_id}/>
        </div>
        <div class="flex flex-col justify-center gap-3 py-3">
          <TestFeedback {is_string_accepted} />
          <Sidebar/>
          <AdvancedAutomataFunctions />
        </div>
      </div>
      <div class="flex justify-center mt-3 gap-4">
          <form class="flex self-center gap-2 align-middle select-none"
          id="stringCheckingForm"
          on:submit|preventDefault={handleStringInput}
          on:change={()=>{is_string_accepted = null}}>
          <label class="w-40 text-2xl self-center" for="string">
            String To Test:
          </label>
          <input class="border-black border-2 text-3xl rounded-md px-2 py-1"
            type="text"
            name="string"
            id="string"
            placeholder="empty string"/>
          <button class="w-40 bg-orange-500 rounded-md text-xl font-semibold border-black 
            border-2 hover:-translate-y-4 duration-300 transition-all will-change-transform" form="stringCheckingForm">
            Check String
          </button>
        </form>      
        <Notifications/>
      </div>
      <LanguageOfAutomata />
    </main>
    <ConfirmTrash />
  </div>
</div>
