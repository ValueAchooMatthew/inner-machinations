<script lang="ts">
    import { Automata } from "$lib/enums";
    import OptionsMenu from "./OptionsMenu.svelte";
    import type { Connection, State } from "$lib/interfaces";
    import Whiteboard from "./Whiteboard.svelte";
    import Banner from "./Banner.svelte";
    import { invoke } from "@tauri-apps/api";
    import Notifications from "./Notifications.svelte";

    export let data;

    $: {if(start_state_coordinates && string_to_check !== undefined){
        let check_string: () => Promise<void>;
        switch(automata_selected){
            case Automata.DFA:
                check_string = async () => {
                let result = await invoke("verify_valid_dfa", {stateConnections: state_connections, 
                    inputAlphabet: input_alphabet});

                if(!result && is_strict_checking){
                    dialogue = `Your DFA either does not specify every connection provided in the input alphabet, or specifies them more than once.
                    Update the model or disable strict checking`;
                    is_string_accepted = null;
                    return;
                };
                dialogue="";
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
    let string_to_check: string;
    let is_string_accepted: boolean | null = null;
    let start_state_coordinates: string | null = null;
    let automata_selected: Automata = Automata.DFA;
    // hashing every coordinate to a state for use when user click on a given coordinate point
    // Allows for O(1) access without having to search for the state which was clicked in the State array
    let state_connections: Map<string, State> = new Map<string, State>();
    let connections: Array<Connection>;
    let default_connection_char: string;
    let sidebar_open: boolean;
    let is_strict_checking: boolean
    let input_alphabet: Array<string>;
    let workspace_name: string | undefined = data.workspace_name;

    const handleSubmit = (event: SubmitEvent) => {
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

<div class="relative flex font-semibold overflow-x-hidden w-full h-full bg-gray-200 min-h-screen">
    <aside class=" bg-orange-500 flex flex-col top-0
    absolute transition-all duration-300 overflow-hidden z-50 w-full h-full" class:left-0={sidebar_open} class:-left-full={!sidebar_open}>
        <OptionsMenu bind:input_alphabet={input_alphabet} bind:is_strict_checking={is_strict_checking} bind:default_connection_char={default_connection_char} bind:sidebar_open={sidebar_open}/>
    </aside>
    <div class="w-full min-w-0">
        <Banner email={data.email} state_connections={state_connections} connections={connections} bind:workspace_name={workspace_name} bind:sidebar_open={sidebar_open} bind:automata_selected={automata_selected}/>
        <main class="flex">
            <Whiteboard email={data.email} bind:connections={connections} workspace_name={workspace_name} bind:start_state_coordinates={start_state_coordinates} bind:dialogue={dialogue} 
            bind:state_connections={state_connections} bind:start_state_index={start_state_index} 
            default_connection_char={default_connection_char} is_string_accepted={is_string_accepted}/>
        </main>
        <div class="flex justify-center mt-3">
            <form class="flex self-center gap-2 align-middle" on:submit|preventDefault={handleSubmit}>
                <label class="w-40 text-2xl self-center" for="string">
                    Check String:
                </label>
                <input class="border-black border-2 text-3xl rounded-md px-2 py-1" type="text" name="string" id="string">
                <div class="w-40">
                </div>
            </form>
            <Notifications dialogue={dialogue} />
        </div>
    </div>
</div>
