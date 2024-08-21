<script lang="ts">
  import OptionsMenu from "./OptionsMenu.svelte";
  import type { CheckedStringResponse, Connection, State } from "$lib/types/interfaces";
  import Whiteboard from "./Whiteboard.svelte";
  import Banner from "./Banner.svelte";
  import { checkInputtedString } from "$lib/utils/stringVerificationFuncs";
  import Notifications from "$lib/components/Notifications.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { dialogue_to_user, start_state_index, state_positions, input_alphabet, 
  start_state_position, type_of_automata, email, workspace_name } from "$lib/utils/automataStores";
  import { setTauriResponses } from "$lib/utils/parsingBackendResponsesFuncs";
  import type { WorkspaceData } from "$lib/types/interfaces";
  import TestFeedback from "./TestFeedback.svelte";
  import AdvancedAutomataFunctions from "./AdvancedAutomataFunctions.svelte";
  import Sidebar from "./Sidebar.svelte";
  import LanguageOfAutomata from "./LanguageOfAutomata.svelte";

  let string_to_check: string;
  let is_string_accepted: boolean | null = null;
  let states_traversed: Array<State> = [];

  // let state_connections: Map<string, State> = new Map<string, State>();
  let highlighted_state: State | null = null;
  let should_show_string_traversal: boolean;
  let default_connection_character: string;
  let is_option_menu_open: boolean = false;
  let should_strict_check: boolean;

  onMount(async () => {
    const workspace_data: WorkspaceData = await invoke("retrieve_workspace_data", {
      email: $email,
      workspaceName: $workspace_name,
    });

    setTauriResponses(
      workspace_data
    );

  });

  const handleStringInput = (event: SubmitEvent) => {
    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }
    const data = new FormData(event.target);
    const inputted_string = data.get("string");
    if (!inputted_string) {
      return;
    }
    if (start_state_index === null) {
      dialogue_to_user.set("You must specify at least one start state");
      return;
    }
    // Here to trigger the tauri invoke to fire even if the same string is inputted as the previous submission
    string_to_check = "";
    string_to_check = inputted_string.toString();

    checkInputtedString(
      $start_state_position, 
      $type_of_automata, 
      $state_positions, 
      string_to_check,
      should_strict_check,
      $input_alphabet)
      .then((result: CheckedStringResponse) => {
        is_string_accepted = result.is_string_accepted;
        states_traversed = result.states_traversed;
        dialogue_to_user.set(
          result.dialogue
        );
      })
      .catch((err)=>{
        console.log(err);
      });
    if(should_show_string_traversal) {
      handleIncrementalStringChecking();
    }
  };

  const handleIncrementalStringChecking = async () => {
    let i = 0;
    const traverseAutomata = setInterval(() => {
      // Every other tick, we're going to unhighlight the previously highlighted state so transitions
      // are more apparent to see
      if(i % 2 == 1) {
        highlighted_state = null;
      } else if(i === 2*states_traversed.length){
        checkInputtedString(
        $start_state_position, 
        $type_of_automata, 
        $state_positions, 
        string_to_check,
        should_strict_check,
        $input_alphabet
        )
        .then((result: CheckedStringResponse) => {
          is_string_accepted = result.is_string_accepted;
          states_traversed = result.states_traversed;
          dialogue_to_user.set(
            result.dialogue
          );
        })
        .catch((err)=>{
          console.log(err);
        });
        clearInterval(traverseAutomata);
      } else {
        // Never an odd number, so we can do this without having to worry
        // about floats
        highlighted_state = states_traversed[i/2];
      }
      i++

    }, 300)

  }

  async function handleKeyDownEvent(event: KeyboardEvent) {
    if(event.key === "Escape") {
      is_option_menu_open = !is_option_menu_open;
    }
  }

</script>

<!-- <svelte:window on:keydown={handleKeyDownEvent}/> -->
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
          <Whiteboard
            {highlighted_state}
            {default_connection_character}/>
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
            id="string"/>
          <button class="w-40 bg-orange-500 rounded-md text-xl font-semibold border-black 
            border-2 hover:-translate-y-4 duration-300 transition-all will-change-transform" form="stringCheckingForm">
            Check String
          </button>
        </form>      
        <Notifications/>
      </div>
      <LanguageOfAutomata />
    </main>
  </div>
</div>
