<script lang="ts">
  import { Automata } from "$lib/types/enums";
  import { workspace_name, type_of_automata } from "$lib/utils/automataStores";
  import { renameWorkspace } from "$lib/workbench/actions";
  import { invokeCommand } from "$lib/utils/backend";
  import { save_status, saveNow } from "$lib/workbench/autosave";
  import { goto } from "$app/navigation";

  export let is_option_menu_open: boolean;

  const SAVE_STATUS_LABELS = {
    saved: "Saved",
    saving: "Saving…",
    unsaved: "Unsaved changes",
    failed: "Not saved",
  };

  // Leaves the workbench once the workspace is saved. If saving fails, the user stays, and is shown why
  async function leaveFor(destination: string) {
    try {
      await saveNow();
    } catch {
      return;
    }
    goto(destination);
  }

  // Renames the workspace once the user stops typing. Pressing Enter renames straight away, and explains
  // when the name can't be used
  let timer: ReturnType<typeof setTimeout> | undefined;
  function handleInputEvent(event: Event & {currentTarget: EventTarget & HTMLInputElement}): void {
    const new_workspace_name = event.currentTarget.value;
    clearTimeout(timer);
    timer = setTimeout(() => {
      renameWorkspace(new_workspace_name).catch(() => {});
    }, 800);
  }

  async function handleSubmitEvent(event: SubmitEvent) {
    clearTimeout(timer);
    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }
    const new_workspace_name = new FormData(event.target).get("workspaceName")?.toString() ?? "";
    await renameWorkspace(new_workspace_name, { explain: true }).catch(() => {});
  }

</script>

<div class="bg-orange-500 flex shadow-lg py-4 pl-2 pr-4 w-full text-gray-100">
  <div class="flex justify-center align-middle w-full gap-6 text-4xl">
    <div class="w-2xl flex justify-between">
      <button class="w-12 h-12 z-10 self-center"
        title="Options (Esc)"
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
        <form on:submit|preventDefault={handleSubmitEvent}>
          <label for="workspaceName"></label>
          <input class="text-gray-950 bg-white px-2 py-1 rounded-md mt-0.5 overflow-hidden h-12 border-black border-2"
            on:input={handleInputEvent}
            id="workspaceName"
            name="workspaceName"
            value={$workspace_name}
            type="text"/>
        </form>
      </div>
    </div>
    <div class="flex gap-2 font-bold self-center ml-auto mr-auto">
      <button class={$type_of_automata == Automata.DFA ? "" : "text-gray-950"}
        on:click={async () => {
          type_of_automata.set(Automata.DFA);
          await invokeCommand("update_automata_type", {workspaceName: $workspace_name, typeOfAutomata: Automata[Automata.DFA]});
        }}>
        DFA
      </button>
      <span>|</span>
      <button
        class={$type_of_automata === Automata.NFA ? "" : "text-gray-950"}
        on:click={async () => {
          type_of_automata.set(Automata.NFA);
          await invokeCommand("update_automata_type", {workspaceName: $workspace_name, typeOfAutomata: Automata[Automata.NFA]});
        }}>
        NFA
      </button>
    </div>
    <nav class="w-2xl flex gap-3 justify-end">
      <span class="self-center text-lg font-semibold whitespace-nowrap" id="save_status">
        {SAVE_STATUS_LABELS[$save_status]}
      </span>
      <a class="flex gap-2 font-bold self-center justify-self-end"
        href="/workspace/dashboard"
        on:click|preventDefault={() => leaveFor("/workspace/dashboard")}>
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
      <a class="flex gap-2 font-bold self-center justify-self-end" href="/" on:click|preventDefault={() => leaveFor("/")}>
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
    </nav>
  </div>
</div>
