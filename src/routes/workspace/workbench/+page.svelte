<script lang="ts">
  import { Action, Automata } from "$lib/enums";


    import type { State } from "$lib/interfaces";
    import Whiteboard from "./whiteboard.svelte";
    import { invoke } from "@tauri-apps/api";

    $: {if(start_state_coordinates && string_to_check){
        const check_string = async () => {
            is_string_accepted = await invoke("test_string", {stateConnections: state_connections, 
                startStateCoordinates: start_state_coordinates, stringToCheck: string_to_check});
        };
        check_string().catch((e)=>{
            console.log(e);
        });
    }};

    let dialogue: string = "";
    let start_state_index: number = -1;
    let string_to_check: string = "";
    let is_string_accepted: boolean;
    let start_state_coordinates: string = "";
    let automata_selected: Automata = Automata.DFA;
    // hashing every coordinate to a state for use when user click on a given coordinate point
    // Allows for O(1) access without having to search for the state which was clicked in the State array
    let state_connections: {[key: string]: State | undefined} = {};

    const handleSubmit = (event: SubmitEvent)=> {
        if(!(event.target instanceof HTMLFormElement)){
            return;
        }
        const data = new FormData(event.target);
        const inputted_string = data.get("string");
        if(!inputted_string){
            return;
        }
        if(start_state_index === -1){
            dialogue = "You must specify at least one start state"
            return;
        }
        // Here to trigger the tauri invoke to fire even if the same string is inputted as the previous submission 
        string_to_check = "";
        string_to_check = inputted_string.toString();
    }
</script>
<div class="font-semibold">
    <div class="flex gap-2 justify-center text-4xl mt-2 font-bold">
        <button class={automata_selected === Automata.DFA ? "text-orange-500":""} on:click={()=>{automata_selected = Automata.DFA}}>
            DFA
        </button>
        <span class="text-orange-500">|</span>
        <button class={automata_selected === Automata.NFA ? "text-orange-500":""} on:click={()=>{automata_selected = Automata.NFA}}>
            NFA
        </button>
    </div>
    {#if (dialogue)}
        <div class="absolute top-0 right-0 left-0 w-fit h-fit mx-auto transition-all duration-300 bg-pink-400 px-5 py-1 rounded-md text-center">
            {dialogue}
        </div>
    {/if}
    {#if is_string_accepted}
        <div class="text-center flex flex-col justify-center absolute top-5 right-5 bg-green-800 rounded-full border-black border-2 w-28 h-28">
            <div class="text-sm">
                The string was accepted!!
            </div>
        </div>
    {:else if is_string_accepted !== undefined}
        <div class="text-center flex flex-col justify-center absolute top-5 right-5 bg-[#e03c3c] rounded-full border-black border-2 w-28 h-28">
            <div class="text-sm">
                The string was not accepted
            </div>
        </div>
    {/if}
    <Whiteboard bind:start_state_coordinates={start_state_coordinates} bind:dialogue={dialogue} 
    bind:state_connections={state_connections} bind:start_state_index={start_state_index}/>

    <div class="flex flex-col justify-center">
        <form class="flex self-center" on:submit|preventDefault={handleSubmit}>
            <label for="string">
                Check String:
                <input class="border-black border-2 text-3xl rounded-md px-2 py-1" type="text" name="string">
            </label>
        </form>
    </div>

    <div class="p-4 flex justify-center font-semibold">
        <a class="text-4xl" href="/">Home</a>
    </div>
</div>
