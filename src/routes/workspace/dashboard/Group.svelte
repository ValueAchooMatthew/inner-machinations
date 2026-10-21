<script lang="ts">
  // A heading for one kind of saved item, a button to make a new one, and the cards for those saved already
  export let title: string;
  export let create_label: string;
  export let create: () => Promise<void>;
</script>

<section class="flex flex-col gap-8 w-full max-w-7xl mx-auto">
  <div class="flex justify-between items-center border-b-2 border-orange-500 pb-3">
    <h2 class="text-3xl font-bold text-gray-900">{title}</h2>
    <button class="flex gap-2 items-center bg-orange-500 text-white font-bold text-xl px-4 py-2 rounded-md shadow-lg
        transition-transform duration-300 hover:scale-105"
      on:click={() => create().catch(() => {})}>
      <svg class="w-7 h-7"
        data-slot="icon"
        fill="none"
        stroke-width="2.2"
        stroke="currentColor"
        viewBox="0 0 24 24"
        xmlns="http://www.w3.org/2000/svg"
        aria-hidden="true">
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 4.5v15m7.5-7.5h-15">
        </path>
      </svg>
      {create_label}
    </button>
  </div>
  <div class="flex flex-wrap justify-center gap-12">
    <slot />
  </div>
</section>
