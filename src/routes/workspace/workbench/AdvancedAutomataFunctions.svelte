<script src="" lang="ts">
  import { input_alphabet, list_of_connections, list_of_states, 
  start_state_index, start_state_position, state_positions } from "$lib/automataStores";
  import type { Connection, State } from "$lib/interfaces";
  import { convertCoordinateToString } from "$lib/miscUtils";
  import { setTauriResponses } from "$lib/parsingBackendResponsesFuncs";
  import { invoke } from "@tauri-apps/api";

  const handleStateMinimization = async (): Promise<void> => {
    
    const tauri_response: [
      number | null,
      Array<State>,
      Array<Connection>,
      { [key: string]: State }
    ] = await invoke("minimize_dfa", {
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

</script>

<button class="bg-orange-500 rounded-md text-lg border-2 border-black"
on:click={handleStateMinimization}>
  Minimize DFA
</button>
