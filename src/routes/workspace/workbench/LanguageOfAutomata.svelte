<script lang="ts">
  import { Automata } from "$lib/types/enums";
  import { email, start_state_position, state_positions, type_of_automata, workspace_name } from "$lib/utils/automataStores";
  import { invoke } from "@tauri-apps/api";

  let language_of_automata: string;

  $: {
    if($start_state_position !== null) {
      invoke("determine_language_of_automata", {
        statePositions: $state_positions, 
        startStateKey: $start_state_position, 
        typeOfAutomata: Automata[$type_of_automata],
      })
      .then((data)=>{
        if(typeof(data) === "string") {
          language_of_automata = data
        }
      })
      .catch((e)=>{
        console.log(e)
      });

    }
  }

</script>
<div class="text-2xl text-center text-orange-500 py-4 select-none">
  {#if language_of_automata != undefined}
  <h3 class="">
    The language of the automata is: {language_of_automata}
  </h3>
  {/if}
</div>