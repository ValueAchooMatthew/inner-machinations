<script lang="ts">
  import { Automata } from "$lib/types/enums";
  import { workspace_name, email, type_of_automata, dialogue_to_user } from "$lib/utils/svelteStores";
  import { saveWorkspace } from "$lib/utils/savingWorkspaceFuncs";
  import { invoke } from "@tauri-apps/api";

  export let is_option_menu_open: boolean;
  let timer: NodeJS.Timeout | undefined;

  // Current workaround to save workspace 500ms after typing has stopped
  // as svelte doesn't provide on:inputend event natively
  async function renameWorkspace(new_workspace_name: string) {
    await invoke("update_regular_automata_workspace_name", {original_workspace_name: $workspace_name, email: $email, new_workspace_name: new_workspace_name});
    workspace_name.set(new_workspace_name);
  }

  async function handleInputEvent(event: Event & {currentTarget: EventTarget & HTMLInputElement}): Promise<void> {
    const new_workspace_name = event.currentTarget.value;
    
    clearTimeout(timer);
    timer = setTimeout(async () => {
      const does_new_workspace_name_already_exist: boolean = await invoke("does_regular_automata_workspace_name_exist", {workspace_name: new_workspace_name, email: $email});
      if(does_new_workspace_name_already_exist) {
        return;
      }
      await renameWorkspace(new_workspace_name);
    }, 800);
  }

  async function handleSubmitEvent(event: SubmitEvent) {
    // In case a user has finished typing and thus begun the timer for automatic input handling
    // But then prior to submission fires submit event
    clearTimeout(timer);
    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }
    const form_data = new FormData(event.target);
    const new_workspace_name = form_data.get("workspace_name")?.toString();
    if(!new_workspace_name) {
      return;
    }
    const does_new_workspace_name_already_exist: boolean = await invoke("does_regular_automata_workspace_name_exist", {workspace_name: new_workspace_name, email: $email});
    if(does_new_workspace_name_already_exist) {
      dialogue_to_user.set(`The entered workspace name already exists `);
      return;
    }
    await renameWorkspace(new_workspace_name)
  }

</script>

<div class="bg-orange-500 flex shadow-lg py-4 pl-2 pr-4 w-full text-gray-100">
  <div class="flex justify-center align-middle w-full gap-6 text-4xl">
    <div class="w-[42rem] flex justify-between">
      <button class="w-12 h-12 z-10 self-center"
        on:click={() => {
          is_option_menu_open = !is_option_menu_open;
        }}>
        <svg
          data-slot="icon"
          aria-hidden="true"
          fill="none"
          stroke-width="1.5"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg">
          <path
            d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
            stroke-linecap="round"
            stroke-linejoin="round"/>
        </svg>
      </button>
      <div>
        <form on:submit={handleSubmitEvent}>
          <label for="workspace_name"></label>
          <input class="text-gray-950 bg-white px-2 py-1 rounded-md mt-0.5 overflow-hidden h-12 border-black border-2"
            on:input={handleInputEvent}
            id="workspace_name"
            name="workspace_name"
            value={$workspace_name}
            type="text"/>
        </form>
      </div>
    </div>
    <div class="flex gap-2 font-bold self-center ml-auto mr-auto">
      <button class={$type_of_automata == Automata.DFA ? "" : "text-gray-950"}
        on:click={async () => {
          type_of_automata.set(Automata.DFA);
          await invoke("update_regular_automata_type", {email: $email, workspace_name: $workspace_name, type_of_automata: Automata[Automata.DFA]});
        }}>
        DFA
      </button>
      <span>|</span>
      <button
        class={$type_of_automata === Automata.NFA ? "" : "text-gray-950"}
        on:click={async () => {
          type_of_automata.set(Automata.NFA);
          await invoke("update_regular_automata_type", {email: $email, workspace_name: $workspace_name, type_of_automata: Automata[Automata.NFA]});
        }}>
        NFA
      </button>
    </div>
    <button class="w-[42rem] flex gap-3 justify-end" on:click={saveWorkspace}>
      <a class="flex gap-2 font-bold self-center justify-self-end"
        href="/workspace/dashboard">
        <svg class="w-10 h-10"
          data-slot="icon"
          aria-hidden="true"
          fill="none"
          stroke-width="1.5"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg">
          <path
            d="M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z"
            stroke-linecap="round"
            stroke-linejoin="round">
          </path>
        </svg>
        <span> Dashboard </span>
      </a>
      <a class="flex gap-2 font-bold self-center justify-self-end" href="/">
        <svg class="w-10 h-10 inline-block"
          data-slot="icon"
          aria-hidden="true"
          fill="none"
          stroke-width="1.5"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg">
          <path
            d="m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></path>
        </svg>
        <span> Home </span>
      </a>
    </button>
  </div>
</div>
