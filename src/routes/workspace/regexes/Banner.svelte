<script lang="ts">
  import { goto } from "$app/navigation";
  import { SAVE_STATUS_LABELS } from "$lib/workbench/autosave";
  import { regex_name, regex_save_status, renameRegex, saveRegexNow } from "$lib/regexes/savedRegex";

  // Leaves the builder once the regular expression is saved. If saving fails, the user stays, and is shown why
  async function leaveFor(destination: string) {
    try {
      await saveRegexNow();
    } catch {
      return;
    }
    goto(destination);
  }

  // Renames the regular expression once the user stops typing. Pressing Enter renames straight away, and
  // explains when the name can't be used
  let timer: ReturnType<typeof setTimeout> | undefined;
  function handleInputEvent(event: Event & {currentTarget: EventTarget & HTMLInputElement}): void {
    const new_regex_name = event.currentTarget.value;
    clearTimeout(timer);
    timer = setTimeout(() => {
      renameRegex(new_regex_name).catch(() => {});
    }, 800);
  }

  async function handleSubmitEvent(event: SubmitEvent) {
    clearTimeout(timer);
    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }
    const new_regex_name = new FormData(event.target).get("regexName")?.toString() ?? "";
    await renameRegex(new_regex_name, { explain: true }).catch(() => {});
  }
</script>

<div class="bg-orange-500 h-fit py-4 px-8 flex justify-between text-4xl text-white font-bold">
  <a class="flex gap-2 font-bold self-center flex-1 basis-0"
    href="/workspace/dashboard"
    on:click|preventDefault={() => leaveFor("/workspace/dashboard")}>
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
    <span class="self-center mb-1"> Dashboard </span>
  </a>
  <!-- The name shrinks first when the window is narrow, and stays centred -->
  <form class="self-center flex justify-center mx-6 basis-[34rem] min-w-0" on:submit|preventDefault={handleSubmitEvent}>
    <label for="regexName"></label>
    <input class="w-full text-gray-950 bg-white px-2 py-1 rounded-md mt-0.5 overflow-hidden h-12 border-black border-2"
      on:input={handleInputEvent}
      id="regexName"
      name="regexName"
      value={$regex_name}
      type="text"/>
  </form>
  <nav class="flex gap-3 self-center justify-end flex-1 basis-0">
    <span class="self-center text-lg font-semibold whitespace-nowrap" id="save_status">
      {SAVE_STATUS_LABELS[$regex_save_status]}
    </span>
    <a class="flex gap-2 font-bold self-center" href="/" on:click|preventDefault={() => leaveFor("/")}>
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
  </nav>
</div>
