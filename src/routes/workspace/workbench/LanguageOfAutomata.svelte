<script lang="ts">
  import { automaton } from "$lib/utils/automataStores";
  import { invokeCommand } from "$lib/utils/backend";
  import { startState, type Automaton } from "$lib/workbench/automaton";

  let language_of_automata: string | undefined;

  // Every change asks again; only the latest answer is shown, however the answers arrive
  let latest_request = 0;

  const determineLanguage = async (current: Automaton) => {
    const request = ++latest_request;
    if (!startState(current)) {
      language_of_automata = undefined;
      return;
    }
    const language: string = await invokeCommand("determine_language_of_automata", { automaton: current });
    if (request === latest_request) {
      language_of_automata = language;
    }
  };

  $: determineLanguage($automaton);

</script>
<div class="text-2xl text-center text-orange-500 py-4 select-none">
  {#if language_of_automata != undefined}
  <h3 class="">
    The language of the automata is: {language_of_automata}
  </h3>
  {/if}
</div>