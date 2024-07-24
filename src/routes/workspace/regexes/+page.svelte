<script lang="ts">
  import { state_positions } from "$lib/utils/automataStores";
  import { parseStatePositions } from "$lib/utils/parsingBackendResponsesFuncs";
  import { invoke } from "@tauri-apps/api";

  let global_regex: string | undefined;

  async function handleSubmit(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    if (!(event.target instanceof HTMLFormElement)) {
      console.log("There was an error reading the regex");
      return;
    }

    const data = new FormData(event.target);
    const regex = data.get("regex");
    global_regex = regex?.toString();
    if(!regex) {
      console.log("There was an error reading the regex");
      return;
    }

    const parsed_regex = await invoke("interpret_regex", {regex: regex});
  }

  async function handleStringChecking(event: SubmitEvent): Promise<boolean> {
    event.preventDefault();
    if (!(event.target instanceof HTMLFormElement) || global_regex === undefined) {
      console.log("There was an error reading the regex");
      return false;
    }

    const data = new FormData(event.target);
    const string_to_test = data.get("string_test");

    const parsed_regex = await invoke("interpret_regex", {regex: global_regex});
    const accepted: boolean = await invoke("test_string_regex", {parseTree: parsed_regex, stringToCheck: string_to_test});
    console.log(accepted);
    return accepted;
  }


</script>
<div class="flex flex-col justify-center mt-30 gap-3 font-semibold">
  <form class="self-center" on:submit={handleSubmit}>
    <label for="regex">Build Regular Expression</label>
    <input class="text-3xl px-1 py-0.5 border-black border-2 rounded-md self-center" id="regex" name="regex" type="text">
  </form>
  <form class="self-center" on:submit={handleStringChecking}>
    <label for="string_test">Check String: </label>
    <input class="text-3xl px-1 py-0.5 border-black border-2 rounded-md" id="string_test" name="string_test" type="text">
  </form>
</div>
