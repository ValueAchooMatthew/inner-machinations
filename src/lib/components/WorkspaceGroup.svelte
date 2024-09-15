<script lang="ts">
  import { goto } from "$app/navigation";
  import { dialogue_to_user, email, workspace_name } from "$lib/utils/svelteStores";

  export let workspace_group_name: string;
  export let redirect: string;
  // Cannot specify as invoke function also returns type Promise<unknown>
  export let create_new_untitled_project: (email: string) => Promise<unknown>;

  async function handleClick() {
    document.cookie =
      "workspace_name=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    // We assume always that the call back is of type () => Promise<string>
    const new_workspace_name = await create_new_untitled_project($email);
    if(typeof new_workspace_name !== "string") {
      return;
    }
    workspace_name.set(
      new_workspace_name
    );
    dialogue_to_user.set(null);
    await goto(redirect);
  }

</script>

<div class="flex w-[23rem] bg-orange-500 my-6 px-6 py-2 rounded-md text-gray-50 justify-between shadow-sm">
  <span class="font-bold text-4xl self-center">
    {workspace_group_name}
  </span>
  <button class="flex flex-col justify-center group" on:click={handleClick}>
    <svg class="ml-2 group-hover:cursor-pointer group-hover:-rotate-90 transition-all duration-200 w-10 h-10 self-center"
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
    <span class="text-xs">
      Create New
    </span>
  </button>
</div>