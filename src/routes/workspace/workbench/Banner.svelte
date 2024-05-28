<script lang="ts">
    import { Automata } from "$lib/enums";
    import type { Connection, State } from "$lib/interfaces";
    import { user_email, is_a_user_logged_in } from "$lib/user";
    import { invoke } from "@tauri-apps/api";
    import { get } from "svelte/store";
    export let sidebar_open: boolean = false;
    export let automata_selected: Automata = Automata.DFA;
    export let workspace_name: String = "Untitled Project";
    export let state_connections: Map<String, State>;
    export let connections: Array<Connection>;


    const email = get(user_email);

    const handleSubmit = async (event: SubmitEvent) => {
        if(!(event.target instanceof HTMLFormElement) || !is_a_user_logged_in){
            return;
        }
        const form_data = new FormData(event.target);
        let new_workspace_name = form_data.get("renamedWorkspace");
        if(!new_workspace_name){
            return;
        }
        new_workspace_name = new_workspace_name.toString();

        await invoke("delete_workspace", {workspaceName: workspace_name, email: email});
        await invoke("save_workspace", {workspaceName: new_workspace_name, email: email, states: state_connections, connections: connections});
        workspace_name = new_workspace_name;
    }

</script>

<div class="bg-orange-500 flex shadow-lg py-4 pl-2 pr-4 w-full text-gray-100">
    <div class="flex justify-center align-middle w-full gap-6 text-4xl">
        <div class="w-[42rem] flex justify-between">
            <button class="w-12 h-12 z-10 self-center" on:click={()=>{sidebar_open = !sidebar_open;}}>
                <svg data-slot="icon" aria-hidden="true" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" stroke-linecap="round" stroke-linejoin="round"></path>
                </svg>
            </button>
            <div>
                <form on:submit={handleSubmit} action="">
                    <input name="renamedWorkspace" id="renamedWorkspace" 
                    class="text-gray-950 bg-white px-2 py-1 rounded-md mt-0.5 overflow-hidden h-12" value={workspace_name} type="text">
                </form>
            </div>
        </div>

        <div class="flex gap-2 font-bold self-center ml-auto mr-auto">
            <button class={automata_selected === Automata.DFA ? "":"text-gray-950"} on:click={()=>{automata_selected = Automata.DFA;}}>
                DFA
            </button>
            <span >|</span>
            <button class={automata_selected === Automata.NFA ? "":"text-gray-950"} on:click={()=>{automata_selected = Automata.NFA;}}>
                NFA
            </button>
        </div>
        <div class="w-[42rem] flex gap-3 justify-end">
            <a class="flex gap-2 font-bold self-center justify-self-end" href="/workspace/dashboard">
                <svg class="w-10 h-10" data-slot="icon" aria-hidden="true" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path d="M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z" stroke-linecap="round" stroke-linejoin="round"></path>
                  </svg>
                <span>
                    Dashboard
                </span>
            </a>
            <a class="flex gap-2 font-bold self-center justify-self-end" href="/">
                <svg class="w-10 h-10 inline-block" data-slot="icon" aria-hidden="true" fill="none" stroke-width="1.5" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" stroke-linecap="round" stroke-linejoin="round"></path>
                </svg>
                <span>
                    Home
                </span>
            </a>
        </div>
    </div>
</div>