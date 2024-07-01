<script lang="ts">
  import { Automata } from "$lib/enums";
  import OptionsMenu from "./OptionsMenu.svelte";
  import type { CheckedStringResponse, Connection, State } from "$lib/interfaces";
  import Whiteboard from "./Whiteboard.svelte";
  import Banner from "./Banner.svelte";
  import { checkInputtedString } from "$lib/stringVerificationFuncs";
  import Notifications from "./Notifications.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { dialogue_to_user, start_state_index, state_positions, input_alphabet, start_state_position, type_of_automata } from "$lib/automataStores";
  import { setTauriResponses } from "$lib/parsingBackendResponsesFuncs";

  export let data;

  // let start_state_index: number | null = null;
  let string_to_check: string;
  let is_string_accepted: boolean | null = null;
  let states_traversed: Array<State> = [];

  // let state_connections: Map<string, State> = new Map<string, State>();
  let highlighted_state: State | null = null;
  let is_showing_string_traversal: boolean;
  let default_connection_char: string;
  let sidebar_open: boolean;
  let is_strict_checking: boolean;
  let workspace_name: string | undefined = data.workspace_name;
  let email: string | undefined = data.email;

  onMount(async ()=> {
    if (!email || !workspace_name) {
      return;
    }
    const tauri_response: [
      number | null,
      Array<State>,
      Array<Connection>,
      { [key: string]: State }
    ] = await invoke("retrieve_workspace_data", {
      email: email,
      workspaceName: workspace_name,
    });

    setTauriResponses(
      tauri_response
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
      is_strict_checking,
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
    if(is_showing_string_traversal){
      handleIncrementalStringChecking();
    }
  };

  const handleIncrementalStringChecking = async () => {
    let i = 0;
    const traverseAutomata = 
    setInterval(()=>{
      // Every other tick, we're going to unhighlight the previously highlighted state so transitions
      // are more apparent to see
      console.log(states_traversed);
      if(i % 2 == 1) {
        highlighted_state = null;
      } else if(i === 2*states_traversed.length){
        checkInputtedString(
        $start_state_position, 
        $type_of_automata, 
        $state_positions, 
        string_to_check,
        is_strict_checking,
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

</script>

<div
  class="relative flex font-semibold overflow-x-hidden w-full h-full bg-gray-200 min-h-screen"
>
  <aside
    class=" bg-orange-500 flex flex-col top-0
    absolute transition-all duration-300 overflow-hidden z-50 w-full h-full"
    class:left-0={sidebar_open}
    class:-left-full={!sidebar_open}
  >
    <OptionsMenu
      bind:is_strict_checking
      bind:default_connection_char
      bind:sidebar_open
      bind:is_showing_string_traversal
    />
  </aside>
  <div class="w-full min-w-0">
    <Banner
      {email}
      bind:workspace_name
      bind:sidebar_open
    />
    <main class="flex">
      <Whiteboard
        {email}
        {workspace_name}
        {highlighted_state}
        {default_connection_char}
        {is_string_accepted}
      />
    </main>
    <div class="flex justify-center mt-3 gap-4">
      <form
        id="stringCheckingForm"
        class="flex self-center gap-2 align-middle select-none"
        on:submit|preventDefault={handleStringInput}
        on:change={()=>{is_string_accepted = null}}
      >
        <label class="w-40 text-2xl self-center" for="string">
          String To Test:
        </label>
        <input
          class="border-black border-2 text-3xl rounded-md px-2 py-1"
          type="text"
          name="string"
          id="string"
        />
        <button class="w-40 bg-orange-500 rounded-md text-xl font-semibold border-black 
          border-2 hover:-translate-y-4 duration-300 transition-all will-change-transform" form="stringCheckingForm">
          Check String
        </button>

      </form>      
      <Notifications/>
    </div>
  </div>
</div>
