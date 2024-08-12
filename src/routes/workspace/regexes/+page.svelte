<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import TitleHeader from "$lib/components/TitleHeader.svelte";
  import Banner from "./Banner.svelte";

  let was_string_accepted: boolean | undefined = undefined
  let regex: string | undefined = undefined;

  function handleUpdatingRegex(event: Event & {currentTarget: EventTarget & HTMLInputElement}): void {
    regex = event.currentTarget.value;
  }

  async function handleStringChecking(event: Event & {currentTarget: EventTarget & HTMLInputElement}): Promise<boolean> {

    const string_to_test = event.currentTarget.value;

    const accepted: boolean = await invoke("test_string_regex", {regex: regex, stringToCheck: string_to_test});
    console.log(await invoke("build_parse_tree", {regex: regex}));
    was_string_accepted = accepted;
    return accepted;
  }

</script>
<Banner />
<div class="flex flex-col justify-center p-24">
  <div class="flex flex-col justify-center mt-30 gap-3 font-semibold text-2xl">
    <form class="self-center" action="">
      <label for="regex">Build Regular Expression</label>
      <input class="text-3xl px-1 py-0.5 border-black border-2 rounded-md self-center" on:input={handleUpdatingRegex} id="regex" name="regex" type="text">
    </form>
    <form class="self-center" >
      <label for="string_test">Check String: </label>
      <input class="text-3xl px-1 py-0.5 border-black border-2 rounded-md" on:input={handleStringChecking} id="string_test" name="string_test" type="text">
    </form>
    <div class="self-center mt-64">
      {#if (was_string_accepted === true)}
      The string was accepted!
      {:else if (was_string_accepted === false)}
      The string wasn't accepted :(
      {/if}
    </div>
  </div>
  <canvas>

  </canvas>
</div>
