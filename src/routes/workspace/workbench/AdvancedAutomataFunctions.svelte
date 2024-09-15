<script src="" lang="ts">

  import { Automata } from "$lib/types/enums";
  import type { RegularAutomataWorkspaceData } from "$lib/types/interfaces";
  import { convertCoordinateToString, getCookie } from "$lib/utils/miscUtils";
  import { setTauriResponses } from "$lib/utils/parsingBackendResponsesFuncs";
  import { input_alphabet, list_of_regular_automata_connections, list_of_states, 
  start_state_index, start_state_position, state_positions, type_of_automata } from "$lib/utils/regularAutomataStores";
  import { email, workspace_name } from "$lib/utils/userStores";
  import { invoke } from "@tauri-apps/api";
  import { tick } from "svelte";

  const handleStateMinimization = async (): Promise<void> => {
    
    const tauri_response: RegularAutomataWorkspaceData =
    await invoke("minimize_dfa", {
      state_positions: $state_positions, 
      connections: $list_of_regular_automata_connections, 
      inputAlphabet: $input_alphabet,
      workspace_name: $workspace_name,
      email: $email
    });

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

  // Todo: Fix to work with workspace data
  const convertNFAToDFA = async () => {
    type_of_automata
      .set(Automata.DFA);

    await invoke("update_regular_automata_type", {email: $email, workspace_name: $workspace_name, type_of_automata: Automata[Automata.DFA]})
    
    await tick();
    
    const tauri_response: RegularAutomataWorkspaceData = await invoke("convert_nfa_to_dfa", {
      startStatePosition: $start_state_position,
      state_positions: $state_positions,
      email: $email,
      workspace_name: $workspace_name
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
<div class="max-w-32 select-none flex flex-col gap-4 w-full self-center">
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