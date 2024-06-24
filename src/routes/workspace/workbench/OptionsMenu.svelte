<script lang="ts">
  import { input_alphabet } from "$lib/automataStores";

  export let default_connection_char: string = "a";
  export let sidebar_open: boolean;
  export let is_strict_checking: boolean = false;
  export let is_showing_string_traversal: boolean = false;

  const handleDefaultCharChange = (event: Event) => {
    if (!(event instanceof InputEvent)) {
      return;
    }
    const data = event.data;
    if (data === null || data === "") {
      return;
    }
    default_connection_char = data;
  };

  const handleSubmitAll = (event: SubmitEvent) => {
    event.preventDefault();
    if (!(event.target instanceof HTMLFormElement)) {
      return;
    }

    // Clearing input alphabet so form data becomes the new input alphabet 
    input_alphabet.set(
      new Array()
    );

    const form_data = new FormData(event.target);
    form_data.forEach((entry) => {

      // If the user attempts to add an empty string or multiple of the same character to the
      // input alphabet, the character isnt added and no new characters are added
      if($input_alphabet.includes(entry.toString()) || "" === entry.toString()) {
        return;
      }

      input_alphabet.update((prev_input_alphabet: Array<string>) => {
        prev_input_alphabet.push(entry.toString());
        return prev_input_alphabet;
      })
    });
  };

  const handleAddingNewCharInput = () => {
    input_alphabet.update((prev_input_alphabet)=>{
      prev_input_alphabet.push("");
      return prev_input_alphabet;
    })

  }

  const handleRemovingCharInput = (index: number) => {
    input_alphabet.update((prev_input_alphabet)=>{
      prev_input_alphabet.splice(index, 1);
      return prev_input_alphabet;
    })
  };

</script>

<div class="flex justify-start">
  <button
    class="w-12 h-12 z-10 self-center ml-4 mt-4"
    on:click={() => {
      sidebar_open = !sidebar_open;
    }}
  >
    <svg
      data-slot="icon"
      aria-hidden="true"
      fill="none"
      stroke-width="1.5"
      stroke="white"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
    </svg>
  </button>
</div>
<h1 class="text-5xl text-gray-100 text-center my-6 mt-24">Options</h1>
<div
  class="font-bold h-fit bg-gray-100 flex flex-col text-3xl justify-start px-96 gap-12 mx-auto rounded-[3rem] p-12"
>
  <form class="self-start" id="alphabetChange" on:submit={handleSubmitAll}>
    <div class="flex justify-center gap-3">
      <label for="alphabet"> Input Alphabet (works for DFA's only): </label>
      <div class="flex flex-col gap-2">
        <!-- Svelte way of itearting through an object with a length property, which I am using to place input elements in DOM -->
        {#each $input_alphabet as value, i}
          <div class="flex gap-1">
            <input
              maxlength="1"
              {value}
              class="border-black border-2 rounded-md px-1"
              type="text"
              name="alphabet"
              id="alphabet"
            />
            <button class="w-8 h-8">
              <svg
                on:click={() => {
                  handleRemovingCharInput(i);
                }}
                data-slot="icon"
                aria-hidden="true"
                fill="none"
                stroke-width="1.5"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                ></path>
              </svg>
            </button>
          </div>
        {/each}
        <button
          on:submit={handleSubmitAll}
          class="bg-orange-500 w-fit h-fit self-center px-2 py-1 rounded-md text-white text-md font-semibold border-black border-2"
          form="alphabetChange"
        >
          Submit All
        </button>
      </div>

      <button class="self-end mb-1" on:click={handleAddingNewCharInput}>
        <svg
          class="w-8 h-8"
          data-slot="icon"
          aria-hidden="true"
          fill="none"
          stroke-width="1.5"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M12 4.5v15m7.5-7.5h-15"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></path>
        </svg>
      </button>
    </div>
  </form>

  <form class="self-start">
    <label for="strict"> Strict Checking (works for DFA's only): </label>
    <input
      class="w-6 h-6 accent-orange-500
        checked:bg-orange-500 checked:border-transparent checked:ring-2 checked:ring-orange-500 checked:ring-offset-2
        checked:ring-offset-white rounded-md px-2 py-1"
      on:change={() => {
        is_strict_checking = !is_strict_checking;
      }}
      type="checkbox"
      name="strict"
      id="strict"
    />
  </form>
  <form class="self-start">
    <label for="showingTraversalSteps"> Show Step-By-Step String Traversal: </label>
    <input
      class="w-6 h-6 accent-orange-500
        checked:bg-orange-500 checked:border-transparent checked:ring-2 checked:ring-orange-500 checked:ring-offset-2
        checked:ring-offset-white rounded-md px-2 py-1 checked:"
      on:change={() => {
        is_showing_string_traversal = !is_showing_string_traversal;
      }}
      type="checkbox"
      name="showingTraversalSteps"
      id="showingTraversalSteps"
    />
  </form>

  <form class="self-start">
    <label for="char">
      Specify default connection character (default: a):
    </label>
    <input
      value={default_connection_char}
      maxlength="1"
      on:input={handleDefaultCharChange}
      class="border-black border-2 rounded-md px-2 py-1"
      type="text"
      name="char"
      id="char"
    />
  </form>
</div>
