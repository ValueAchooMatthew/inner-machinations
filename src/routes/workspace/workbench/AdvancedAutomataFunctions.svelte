<script src="" lang="ts">

  import { Automata } from "$lib/types/enums";
  import { input_alphabet, list_of_connections, list_of_states, 
  start_state_index, start_state_position, state_positions, type_of_automata } from "$lib/utils/automataStores";
  import { convertCoordinateToString } from "$lib/utils/miscUtils";
  import { setTauriResponses } from "$lib/utils/parsingBackendResponsesFuncs";
  import { invoke } from "@tauri-apps/api";
  import type { TauriGeneratedAutomataInformation } from "$lib/types/types";

  const handleStateMinimization = async (): Promise<void> => {
    
    const tauri_response: TauriGeneratedAutomataInformation =
    await invoke("minimize_dfa", {
      stateConnections: $state_positions, 
      connections: $list_of_connections, 
      inputAlphabet: $input_alphabet});

    setTauriResponses(
      tauri_response
    );

    // This is not included in the setTauriResponses function because the syntactic sugar for $ is not present in .ts files
    // and I wanted to keep things as svelte-y as possible
    start_state_position.set(
      $start_state_index !== null? convertCoordinateToString($list_of_states[$start_state_index].position): 
      null
    );
    
  }

  const convertNFAToDFA = async () => {
    type_of_automata
      .set(Automata.DFA);
    
    const tauri_response: TauriGeneratedAutomataInformation =
    await invoke("convert_nfa_to_dfa", {
      startStatePosition: $start_state_position,
      statePositions: $state_positions
    });

    setTauriResponses(
      tauri_response
    );

    start_state_position.set(
      $start_state_index !== null? convertCoordinateToString($list_of_states[$start_state_index].position): 
      null
    );

  }

</script>
<div class="max-w-32 select-none flex flex-col gap-4">
  {#if $type_of_automata === Automata.DFA }
  <button class="bg-orange-500 rounded-md text-lg border-2 border-black px-1 py-0.5" on:click={handleStateMinimization}>
    Minimize DFA
  </button>
  {:else}
  <button class="bg-orange-500 rounded-md text-lg border-2 border-black px-1 py-0.5" on:click={convertNFAToDFA}>
    Convert NFA to DFA
  </button>
  {/if}
</div>