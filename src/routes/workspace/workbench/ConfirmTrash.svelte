<script lang="ts">
  import { confirming_trash } from "$lib/utils/automataStores";
  import { cancelTrash, confirmTrash } from "$lib/workbench/actions";
</script>

<!-- Asks before the trash button clears the canvas -->
<div class="bg-orange-500 text-white font-bold px-4 py-3 rounded-md fixed transition-all
    duration-300 mx-auto my-auto left-0 right-0 w-fit h-fit bottom-0 border-2 border-black z-50"
  class:top-full={!$confirming_trash}
  class:mt-96={!$confirming_trash}
  class:top-0={$confirming_trash}
  role="alertdialog"
  aria-labelledby="confirm_trash_question">
  <span class="text-2xl" id="confirm_trash_question">
    Clear the canvas? You can undo this.
  </span>
  <div class="flex justify-around text-lg">
    <button on:click={confirmTrash}> Clear </button>
    <button on:click={cancelTrash}> Keep </button>
  </div>
</div>
