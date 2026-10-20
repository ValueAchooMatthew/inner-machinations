<script lang="ts">

  import { Automata } from "$lib/types/enums";
  import type { WorkspaceData } from "$lib/types/interfaces";
  import { automaton, input_alphabet, type_of_automata, workspace_name } from "$lib/utils/automataStores";
  import { invokeCommand } from "$lib/utils/backend";
  import { replaceAutomaton } from "$lib/workbench/actions";
  import { fromWorkspaceData, startStateKey, toConnections, toStatePositions } from "$lib/workbench/api";

  const handleStateMinimization = async (): Promise<void> => {
    const tauri_response: WorkspaceData = await invokeCommand("minimize_dfa", {
      statePositions: toStatePositions($automaton),
      connections: toConnections($automaton),
      inputAlphabet: $input_alphabet,
      startStatePosition: startStateKey($automaton),
      workspaceName: $workspace_name
    });
    replaceAutomaton(fromWorkspaceData(tauri_response));
  }

  const convertNFAToDFA = async () => {
    const tauri_response: WorkspaceData = await invokeCommand("convert_nfa_to_dfa", {
      statePositions: toStatePositions($automaton),
      connections: toConnections($automaton),
      inputAlphabet: $input_alphabet,
      startStatePosition: startStateKey($automaton),
      workspaceName: $workspace_name
    });
    replaceAutomaton(fromWorkspaceData(tauri_response));

    // Only once there is a DFA to show
    type_of_automata.set(Automata.DFA);
    await invokeCommand("update_automata_type", {workspaceName: $workspace_name, typeOfAutomata: Automata[Automata.DFA]});
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