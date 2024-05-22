<script lang="ts">
    import { Automata } from "$lib/enums";
    import type { State } from "$lib/interfaces";
    import Whiteboard from "./whiteboard.svelte";
    import { invoke } from "@tauri-apps/api";

    $: {if(start_state_coordinates && string_to_check){
        let check_string: () => Promise<void>;
        switch(automata_selected){
            case Automata.DFA:
                check_string = async () => {
                is_string_accepted = await invoke("test_string_dfa", {stateConnections: state_connections, 
                    startStateCoordinates: start_state_coordinates, stringToCheck: string_to_check});
                };
                break;

            case Automata.NFA:
                check_string = async () => {
                is_string_accepted = await invoke("test_string_nfa", {stateConnections: state_connections, 
                    startStateCoordinates: start_state_coordinates, stringToCheck: string_to_check});
                };
                break;
        }

        check_string().catch((e)=>{
            console.log(e);
        });
    }};

    let dialogue: string = "";
    let start_state_index: number = -1;
    let string_to_check: string = "";
    let is_string_accepted: boolean;
    let start_state_coordinates: String | null = null;
    let automata_selected: Automata = Automata.DFA;
    // hashing every coordinate to a state for use when user click on a given coordinate point
    // Allows for O(1) access without having to search for the state which was clicked in the State array
    let state_connections:  Map<String, State> = new Map<String, State>();
    let default_connection_char: string;
    let sidebar_open: boolean = false;

    const handleCharChange = (event: Event) => {
        if(!(event instanceof InputEvent)){
            return;
        }
        const data = event.data;
        if(!data){
            return;
        }
        default_connection_char = event.data;
    }


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
<div class="relative flex font-semibold overflow-x-hidden w-full h-full ">
    <div class="bg-gray-200 p-4 text-center w-min-fit h-min-fit flex flex-col rounded-r-md h-full absolute transition-all duration-300 overflow-hidden z-50 w-full"  class:left-0={sidebar_open} class:-left-full={!sidebar_open}>
        <div class="flex justify-start">
            <button class="w-12 h-12 z-10 self-center" on:click={()=>{sidebar_open = !sidebar_open;}}>
                <svg data-slot="icon" aria-hidden="true" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" stroke-linecap="round" stroke-linejoin="round"></path>
                </svg>
            </button>
        </div>
        <div class="font-bold">
            Input Alphabet (works for DFA's only):
            <input on:input={handleCharChange} class="border-black border-2 rounded-md" type="text">
        </div>
        <div>
            Strict Checking (works for DFA's only):
        </div>
        <div>
            Specify default connection character (default: a):
            <input on:input={handleCharChange} class="border-black border-2 rounded-md" type="text">
        </div>
    
    </div>
       
    <div class="w-full flex-1 min-w-0">
        <div class="flex shadow-lg py-4 px-2 bg-gray-200 w-full">
            <div class="flex justify-start align-middle w-full gap-6">
                <button class="w-12 h-12 z-10 self-center" on:click={()=>{sidebar_open = !sidebar_open;}}>
                    <svg data-slot="icon" aria-hidden="true" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                </button>
                <div class="flex gap-2 text-4xl font-bold self-center">
                    <button class={automata_selected === Automata.DFA ? "text-orange-500":""} on:click={()=>{automata_selected = Automata.DFA}}>
                        DFA
                    </button>
                    <span class="text-orange-500">|</span>
                    <button class={automata_selected === Automata.NFA ? "text-orange-500":""} on:click={()=>{automata_selected = Automata.NFA}}>
                        NFA
                    </button>
                </div>
            </div>
    
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
        bind:state_connections={state_connections} bind:start_state_index={start_state_index} default_connection_char={default_connection_char}/>
    
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


</div>
