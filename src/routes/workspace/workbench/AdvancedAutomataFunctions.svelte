<script lang="ts">
  import { Automata } from "$lib/types/enums";
  import { type_of_automata } from "$lib/utils/automataStores";
  import { convertNfaToDfa, minimizeDfa } from "$lib/workbench/actions";
</script>
<div class="max-w-32 select-none flex flex-col gap-4 w-full self-center">
  {#if $type_of_automata === Automata.DFA }
  <button class="bg-orange-500 rounded-md text-lg border-2 border-black px-1 py-0.5" on:click={minimizeDfa}>
    Minimize DFA
  </button>
  {:else}
  <button class="bg-orange-500 rounded-md text-lg border-2 border-black px-1 py-0.5" on:click={convertNfaToDfa}>
    Convert NFA to DFA
  </button>
  {/if}
</div>