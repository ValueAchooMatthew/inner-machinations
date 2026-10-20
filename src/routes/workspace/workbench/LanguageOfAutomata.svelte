<script lang="ts">
  import { start_state_position, state_positions } from "$lib/utils/automataStores";
  import { invokeCommand } from "$lib/utils/backend";

  let language_of_automata: string;

  $: {
    if($start_state_position !== null) {
      invokeCommand("determine_language_of_automata", {
        statePositions: $state_positions, 
        startStateKey: $start_state_position,
      })
      .then((data)=>{
        if(typeof(data) === "string") {
          language_of_automata = data
        }
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