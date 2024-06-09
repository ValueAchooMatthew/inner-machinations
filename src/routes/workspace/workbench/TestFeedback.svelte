<script lang="ts">
  export let is_string_accepted: boolean | null;
</script>

<div class="flex-col align-middle justify-cente0 select-none">
  {#if is_string_accepted}
    <svg
      class="w-24 h-24 self-center mx-auto"
      data-slot="icon"
      aria-hidden="true"
      fill="none"
      stroke-width="2.2"
      stroke="green"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M9 12.75 11.25 15 15 9.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
    </svg>
    <div class="text-center mx-auto">
      The string <br /> was accepted
    </div>
  {:else if is_string_accepted !== null}
    <svg
      class="w-24 h-24 self-center mx-auto"
      data-slot="icon"
      aria-hidden="true"
      fill="none"
      stroke-width="2.2"
      stroke="red"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="m9.75 9.75 4.5 4.5m0-4.5-4.5 4.5M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
    </svg>
    <div class="text-center mx-auto">
      The string <br /> was not accepted
    </div>
  {:else}
    <div class="w-24 h-36"></div>
  {/if}
</div>
