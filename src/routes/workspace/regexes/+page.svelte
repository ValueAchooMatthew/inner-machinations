<script lang="ts">
  import { invoke } from "@tauri-apps/api";


  async function handleSubmit(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    if (!(event.target instanceof HTMLFormElement)) {
      console.log("There was an error reading the regex");
      return;
    }

    const data = new FormData(event.target);
    const regex = data.get("regex");
    console.log(data)
    if(!regex) {
      console.log("There was an error reading the regex");
      return;
    }

    const balls = await invoke("interpret_regex", {regex: regex});
    console.log(balls)
  }

</script>
<div class="flex justify-center mt-30">
  <form on:submit={handleSubmit}>
    <input class="text-3xl px-1 py-0.5 border-black border-2 rounded-md" id="regex" name="regex" type="text">
  </form>
</div>
