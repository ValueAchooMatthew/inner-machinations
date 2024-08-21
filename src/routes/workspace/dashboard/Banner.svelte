<script lang="ts">
  import { goto } from "$app/navigation";
  import { dialogue_to_user, email, workspace_name } from "$lib/utils/automataStores";
  import { invoke } from "@tauri-apps/api";

  async function handleClick() {
    document.cookie =
    "workspace_name=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    const does_workspace_exist = await invoke("does_workspace_name_exist", {workspaceName: "Untitled Project", email: $email})
    if(does_workspace_exist) {
      dialogue_to_user.set(
        "Untitled Project already exists"
      )
      return;
    } 
    
    document.cookie = "workspace_name" + "=" + "Untitled Project" + "; path=/";
    workspace_name.set(
      "Untitled Project"
    )
    dialogue_to_user.set("");
    await invoke("create_workspace", {email: $email, workspaceName: "Untitled Project"});
    goto("/workspace/workbench");
  }
</script>

<div class="bg-orange-500 flex shadow-lg py-4 pl-2 pr-4 w-full text-gray-100 text-4xl font-bold justify-between align-middle z-50 fixed top-0">
  <div class="flex justify-start w-64">
    <svg class="ml-2 hover:cursor-pointer hover:-rotate-90 w-12 h-12 transition-all duration-200"
      on:click={handleClick}
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
  </div>

  <div class="flex flex-col justify-center"> My Dashboard </div>

  <a class="flex gap-2 font-bold self-center justify-end w-64" href="/">
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
    <span> Home </span>
  </a>
</div>
