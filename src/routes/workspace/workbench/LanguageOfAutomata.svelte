<script lang="ts">
  import { Automata } from "$lib/types/enums";
  import { start_state_position, state_positions, type_of_automata } from "$lib/utils/automataStores";
  import { invoke } from "@tauri-apps/api";

  let language_of_automata: string;

  $: {
    if($type_of_automata === Automata.DFA && $start_state_position !== null)
    invoke("determine_language_of_dfa", {statePositions: $state_positions, startStateKey: $start_state_position})
    .then((data)=>{
      if(typeof(data) === "string") {
        language_of_automata = data
      }
    })
    .catch((e)=>{
      console.log(e)
    });
  }

</script>
<div class="text-2xl text-center text-orange-500 py-4 select-none">
  {#if $type_of_automata == Automata.DFA && language_of_automata != undefined}
  <h3 class="">
    The language of the automata is: {language_of_automata}
  </h3>
  {:else if $type_of_automata != undefined}
  <h3>
    To view the language of the NFA, first convert it to a DFA
  </h3>
  {/if}
</div>