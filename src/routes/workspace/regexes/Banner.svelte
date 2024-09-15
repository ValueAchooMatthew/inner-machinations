<script lang="ts">
  import { current_regex, dialogue_to_user, email, workspace_name } from "$lib/utils/svelteStores";
  import { invoke } from "@tauri-apps/api";
  
  let timer: NodeJS.Timeout | undefined;

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
    const does_new_workspace_name_already_exist: boolean = await invoke("does_regex_workspace_name_exist", {regex_name: new_workspace_name, email: $email});
    if(does_new_workspace_name_already_exist) {
      console.log("fired!")
      dialogue_to_user.set("The entered workspace name already exists");
      return;
    }
    await invoke("update_regex_workspace_name", {email: $email, original_regex_name: $workspace_name, new_regex_name: new_workspace_name});
    workspace_name.set(
      new_workspace_name
    );
  }

  function handleInputEvent() {

  }

  async function handleClick() {
    await invoke("save_regex_workspace", { email: $email, regex_name: $workspace_name, regex: $current_regex });
    current_regex.set(
      ""
    );
  }

</script>

<div class="bg-orange-500 h-fit py-4 pl-2 pr-4 flex justify-between text-4xl text-white font-bold">
  <div class="w-96">
    <form class="w-fit font-semibold" on:submit={handleSubmitEvent}>
      <label for="workspace_name"></label>
      <input class="text-gray-950 bg-white px-2 py-1 rounded-md mt-0.5 overflow-hidden h-12 border-black border-2"
        on:input={handleInputEvent}
        id="workspace_name"
        name="workspace_name"
        value={$workspace_name}
        type="text"/>
    </form>
  </div>
  <span>
    Regular Expression Builder
  </span>
  <button class="flex justify-end gap-2 w-96" on:click={handleClick}>
    <a class="flex gap-2 font-bold self-center"
    href="/workspace/dashboard">
      <svg class="w-10 h-10 self-center"
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
      <span class="self-center mb-1z"> Dashboard </span>
    </a>
    <a class="flex gap-2 font-bold self-center justify-end" href="/">
      <svg class="self-center w-10 h-10"
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
          stroke-linejoin="round">
        </path>
      </svg>
      <span class="mb-1">Home</span>
    </a>
  </button>
</div>