<script lang="ts">
    
    import { Action } from "$lib/enums";
    export let current_action: Action;
    export let clearCursor: () => void;
    export let undo: () => void;
    export let selected_connection_index: number | null;
    export let handleTrash: () => void;

</script>

<div class="text-center select-none flex flex-col justify-between
gap-3 bg-opacity-100 w-32 h-fit absolute right-4 top-0 bottom-0 my-auto border-black border-2 rounded-md px-2 py-4 mr-0.5 z-10">
    <div class="flex flex-col gap-2">
        <button on:click={()=>{clearCursor(); current_action = Action.ADDING_START_STATE;}} class="flex flex-col self-center" style="line-height: 15px;">
            New Start State
            <div class="mt-2 self-center bg-green-600 rounded-full w-14 h-14 border-black border-[1px]">
            </div>
        </button>
        <button on:click={()=>{clearCursor(); current_action = Action.ADDING_REGULAR_STATE;}}  class="flex flex-col self-center">
            New State
            <div class="self-center bg-orange-600 rounded-full w-14 h-14 border-black border-[1px]">
            </div>
        </button>
        <button on:click={()=>{clearCursor(); current_action = Action.ADDING_FINAL_STATE;}} 
            class="flex flex-col self-center" style="line-height: 15px;">
            New Final State
            <div  class="mt-2 self-center  rounded-full w-[4.5rem] h-[4.5rem] border-black border-[1px]">
            </div>
        </button>
        <button on:click={()=>{clearCursor(); current_action = Action.PLACING_LINE;}} class="flex flex-col " style="line-height: 15px;">
            New Connection
            <svg class="hover:cursor-pointer w-10 self-center" data-slot="icon" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" d="M17.25 8.25 21 12m0 0-3.75 3.75M21 12H3"></path>
            </svg>
        </button>

    </div>
    <div class="flex justify-center mt-2">
        <svg on:click={()=>{clearCursor(); undo(); selected_connection_index = null;}} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor"
             viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
            <path clip-rule="evenodd" fill-rule="evenodd" d="M2.515 10.674a1.875 1.875 0 0 0 0 2.652L8.89 19.7c.352.351.829.549 1.326.549H19.5a3 3 0 0 0 3-3V6.75a3 3 0 
            0 0-3-3h-9.284c-.497 0-.974.198-1.326.55l-6.375 6.374ZM12.53 9.22a.75.75 0 1 0-1.06 1.06L13.19 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L15.31 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-1.72 1.72-1.72-1.72Z"></path>
          </svg>
        <svg on:click={()=>{clearCursor(); handleTrash(); selected_connection_index = null;}} class="hover:cursor-pointer w-6" data-slot="icon" fill="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
            <path clip-rule="evenodd" fill-rule="evenodd" d="M16.5 4.478v.227a48.816 48.816 0 0 1 3.878.512.75.75 0 1 1-.256 1.478l-.209-.035-1.005 13.07a3 3 0 0 1-2.991 2.77H8.084a3 3 0 0 1-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 0 1-.256-1.478A48.567 48.567 
            0 0 1 7.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 0 1 3.369 0c1.603.051 2.815 1.387 2.815 2.951Zm-6.136-1.452a51.196 51.196 0 0 1 3.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 0 0-6 0v-.113c0-.794.609-1.428 1.364-1.452Zm-.355 5.945a.75.75 0 1 0-1.5.058l.347 9a.75.75 0 1 0 1.499-.058l-.346-9Zm5.48.058a.75.75 0 1 0-1.498-.058l-.347 9a.75.75 0 0 0 1.5.058l.345-9Z"></path>
        </svg>
        <svg on:click={()=>{clearCursor();}} aria-hidden="true" class="hover:cursor-pointer w-6 mb-0.5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path clip-rule="evenodd" d="M6.672 1.911a1 1 0 10-1.932.518l.259.966a1 1 0 001.932-.518l-.26-.966zM2.429 4.74a1 1 0 10-.517 1.932l.966.259a1 1 0 00.517-1.932l-.966-.26zm8.814-.569a1 1 
            0 00-1.415-1.414l-.707.707a1 1 0 101.415 1.415l.707-.708zm-7.071 7.072l.707-.707A1 1 0 003.465 9.12l-.708.707a1 1 0 001.415 1.415zm3.2-5.171a1 1 0 00-1.3 1.3l4 10a1 1 0 001.823.075l1.38-2.759 3.018 3.02a1 1 0 001.414-1.415l-3.019-3.02 2.76-1.379a1 1 0 00-.076-1.822l-10-4z" fill-rule="evenodd"></path>
          </svg>
    </div>
</div>

