<script lang="ts">
  import { dialogue_to_user } from "$lib/utils/svelteStores";

  let dialogue: string | null = "";
  dialogue_to_user.subscribe((value)=>{
    dialogue = value;
  })

</script>
<div class="bg-orange-500 text-white rounded-md py-4 flex justify-center align-middle text-2xl gap-5 transition-all duration-500 select-none font-semibold"
class:w-0:={!dialogue} class:w-fit={dialogue} class:px-4={dialogue} class:h-0={!dialogue} class:h-fit={dialogue} class:py-4={dialogue}>
  <svg class="self-center w-10 h-10" class:hidden={!dialogue} data-slot="icon" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
    <path 
      stroke-linecap="round" 
      stroke-linejoin="round" 
      d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z">
    </path>
  </svg>
  <div class="self-center max-w-[42rem] text-center">
    {#if dialogue}
    {dialogue}
    {/if}
  </div>
</div>