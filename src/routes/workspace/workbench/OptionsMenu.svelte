<script lang="ts">
  import { input_alphabet, should_show_string_traversal, should_strict_check, 
  default_connection_char, workspace_name, email } from "$lib/utils/automataStores";
  import { convertFormDataEntriesToStringArray} from "$lib/utils/miscUtils";
  import { saveOptions } from "$lib/utils/savingWorkspaceFuncs";
  import { invoke } from "@tauri-apps/api";

  export let is_option_menu_open: boolean = false;
  let form: HTMLFormElement | undefined;

  const handleAddingInputElement = () => {
    input_alphabet.update((prev_input_alphabet) => {
      prev_input_alphabet.push("");
      return prev_input_alphabet;
    })
  }

  const handleRemovingInputElement = (index: number) => {
    input_alphabet.update((prev_input_alphabet)=>{
      prev_input_alphabet.splice(index, 1);
      return prev_input_alphabet;
    })
  };

  async function handleClosingOptionMenu() {
    is_option_menu_open = false;
    if(form) {

      const data = new FormData(form);

      const alphabet = data.getAll("alphabet");
      const stringified_array = convertFormDataEntriesToStringArray(alphabet);
      const sanitized_alphabet: Array<string> = await invoke("update_workspace_alphabet", {
        workspaceName: $workspace_name, email: $email, alphabet: stringified_array
      });
      input_alphabet.set(sanitized_alphabet);

      // await invoke("");
      
      const default_connection_character = data.get("default_character")?.toString();
      if(default_connection_character === undefined) {
        return;
      }
      default_connection_char.set(default_connection_character)

    }
  }

  async function handleKeyDownEvent(event: KeyboardEvent) {
    if(event.key === "Escape") {
      is_option_menu_open = !is_option_menu_open;
      await saveOptions(form);
    }
  }

</script>
<style>
  input[type="checkbox"] {
    -webkit-appearance: none;
    appearance: none;
    width: 1.5rem;
    height: 1.5rem;
    background-color: #fff;
    border-radius: 0.375rem;
    border: 2px solid #ed8936; /* Orange border */
    position: relative;
    outline: none;
    cursor: pointer;
  }

  input[type="checkbox"]:checked {
    background-color: #ed8936; /* Orange background */
    border-color: transparent;
  }

  input[type="checkbox"]:checked::after {
    content: '';
    position: absolute;
    top: 0.2rem;
    left: 0.45rem;
    width: 0.4rem;
    height: 0.7rem;
    border: solid white;
    border-width: 0 0.2rem 0.2rem 0;
    transform: rotate(45deg);
  }
</style>

<svelte:window on:keydown={handleKeyDownEvent}/>
<div class="flex justify-start">
  <button class="w-12 h-12 z-10 self-center ml-4 mt-4"
    on:click={handleClosingOptionMenu}>
    <svg
      data-slot="icon"
      aria-hidden="true"
      fill="none"
      stroke-width="1.5"
      stroke="white"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg">
      <path
        d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
        stroke-linecap="round"
        stroke-linejoin="round">
      </path>
    </svg>
  </button>
</div>
<h1 class="text-5xl text-gray-100 text-center my-6 mt-24">Options</h1>
<div class="font-bold h-fit bg-gray-100 flex flex-col text-3xl justify-start px-72 gap-24 mx-auto rounded-[3rem] p-12">
  <form class="self-start flex flex-col gap-4" bind:this={form} id="alphabetChange">
    <div class="flex justify-between">
      <label for="alphabet"> Input Alphabet (works for DFA's only): </label>
      <div class="flex flex-col gap-2">
        <!-- Svelte way of iterating through an object with a length property, which I am using to place input elements in DOM -->
        {#each $input_alphabet as value, i}
          <div class="flex gap-1">
            <input class="border-black border-2 rounded-md px-1"
              maxlength="1"
              {value}
              type="text"
              name="alphabet"
              id="alphabet"/>
            <button class="w-8 h-8">
              <svg
                on:click={() => {
                  handleRemovingInputElement(i);
                }}
                data-slot="icon"
                aria-hidden="true"
                fill="none"
                stroke-width="1.5"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg">
                <path
                  d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
                  stroke-linecap="round"
                  stroke-linejoin="round">
                </path>
              </svg>
            </button>
          </div>
        {/each}
      </div>
      <button class="self-end mb-1" on:click={handleAddingInputElement}>
        <svg class="w-8 h-8"
          data-slot="icon"
          aria-hidden="true"
          fill="none"
          stroke-width="1.5"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg">
          <path
            d="M12 4.5v15m7.5-7.5h-15"
            stroke-linecap="round"
            stroke-linejoin="round">
          </path>
        </svg>
      </button>
    </div>
    <div>
      <label for="strict"> Strict Checking (works for DFA's only): </label>
      <input class="w-6 h-6 accent-orange-500
          checked:bg-orange-500 checked:border-transparent checked:ring-2 checked:ring-orange-500 checked:ring-offset-2
          checked:ring-offset-white rounded-md px-2 py-1"
        on:change={() => {
          should_strict_check.set(!$should_strict_check);
        }}
        value={$should_strict_check}
        type="checkbox"
        name="strict"
        id="strict"/>
    </div>
    <div>
      <label for="traversal"> Show Step-By-Step String Traversal: </label>
      <input class="w-6 h-6 accent-orange-500
          checked:bg-orange-500 checked:border-transparent checked:ring-2 checked:ring-orange-500 checked:ring-offset-2
          checked:ring-offset-white rounded-md px-2 py-1 checked:"
        on:change={() => {
          should_show_string_traversal.set(!$should_show_string_traversal);
        }}
        value={$should_show_string_traversal}
        type="checkbox"
        name="traversal"
        id="traversal"/>
    </div>

    <div>
      <label for="default_character">
        Specify default connection character (default: a):
      </label>
      <input class="border-black border-2 rounded-md px-2 py-1"
        value={$default_connection_char}
        maxlength="1"
        type="text"
        name="default_character"
        id="default_character"/>
    </div>
  </form>
</div>
