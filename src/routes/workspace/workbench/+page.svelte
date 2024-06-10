<script lang="ts">
  import { Automata } from "$lib/enums";
  import OptionsMenu from "./OptionsMenu.svelte";
  import type { CheckedStringResponse, Connection, State } from "$lib/interfaces";
  import Whiteboard from "./Whiteboard.svelte";
  import Banner from "./Banner.svelte";
  import { checkInputtedString } from "$lib/stringVerificationFuncs";
  import Notifications from "./Notifications.svelte";

  export let data;

  $: {

    checkInputtedString(
      start_state_coordinates, 
      automata_selected, 
      state_connections, 
      string_to_check,
      is_strict_checking,
      input_alphabet
      )
      .then((result: CheckedStringResponse) => {
        is_string_accepted = result.is_string_accepted;
        states_traversed = result.states_traversed;
        dialogue = result.dialogue;
      })
      .catch((err)=>{
        console.log(err);
      });
  }

  let dialogue: string = "";
  let start_state_index: number | null = null;
  let string_to_check: string;
  let is_string_accepted: boolean | null = null;
  let states_traversed: Array<State> = [];
  let start_state_coordinates: string | null = null;
  let automata_selected: Automata = Automata.DFA;
  // hashing every coordinate to a state for use when user click on a given coordinate point
  // Allows for O(1) access without having to search for the state which was clicked in the State array
  let state_connections: Map<string, State> = new Map<string, State>();
  let highlighted_state: State | null = null;
  let connections: Array<Connection>;
  let default_connection_char: string;
  let sidebar_open: boolean;
  let is_strict_checking: boolean;
  let input_alphabet: Array<string>;
  let workspace_name: string | undefined = data.workspace_name;

  const handleSubmit = (event: SubmitEvent) => {
    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }
    const data = new FormData(event.target);
    const inputted_string = data.get("string");
    if (!inputted_string) {
      return;
    }
    if (start_state_index === null) {
      dialogue = "You must specify at least one start state";
      return;
    }
    // Here to trigger the tauri invoke to fire even if the same string is inputted as the previous submission
    string_to_check = "";
    string_to_check = inputted_string.toString();
  };

  const handleIncrementalStringChecking = async () => {

    is_string_accepted = null;
    let i = 0;
    const traverseAutomata = 
    setInterval(()=>{
      if(i === states_traversed.length){
        checkInputtedString(
        start_state_coordinates, 
        automata_selected, 
        state_connections, 
        string_to_check,
        is_strict_checking,
        input_alphabet
        )
        .then((result: CheckedStringResponse) => {
          is_string_accepted = result.is_string_accepted;
          states_traversed = result.states_traversed;
          dialogue = result.dialogue;
        })
        .catch((err)=>{
          console.log(err);
        });
          clearInterval(traverseAutomata);
      }
      highlighted_state = states_traversed[i];
      i++
    }, 500)

    // The inputted string is checked after so that the result of whether the string is accepted or not
    // Is not displayed to the user until after the traversal is finished



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
      bind:input_alphabet
      bind:is_strict_checking
      bind:default_connection_char
      bind:sidebar_open
    />
  </aside>
  <div class="w-full min-w-0">
    <Banner
      email={data.email}
      {state_connections}
      {connections}
      bind:workspace_name
      bind:sidebar_open
      bind:automata_selected
    />
    <main class="flex">
      <Whiteboard
        email={data.email}
        bind:connections
        {workspace_name}
        bind:start_state_coordinates
        bind:dialogue
        bind:state_connections
        bind:start_state_index
        {highlighted_state}

        {default_connection_char}
        {is_string_accepted}
      />
    </main>
    <div class="flex justify-center mt-3">
      <form
        class="flex self-center gap-2 align-middle select-none"
        on:submit|preventDefault={handleSubmit}
      >
        <label class="w-40 text-2xl self-center" for="string">
          Check String:
        </label>
        <input
          class="border-black border-2 text-3xl rounded-md px-2 py-1"
          type="text"
          name="string"
          id="string"
        />
        <div class="w-40"></div>
      </form>
      <button on:click={handleIncrementalStringChecking}>
        Show String Verification Steps
      </button>
      
      <Notifications {dialogue} />
    </div>
  </div>
</div>
