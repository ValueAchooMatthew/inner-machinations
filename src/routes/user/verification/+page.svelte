<script lang="ts">
  import TitleHeader from "$lib/components/TitleHeader.svelte";
  import { getCookie } from "$lib/utils/miscUtils";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  // Having to use getCookie instead of server file as prod version of app
  // cannot work with server files due to disabled SSR
  let data = {
    email: getCookie("email")
  };

  let correct_code: string | null = "";
  let is_verified = false;
  let response = "";

  const handleSubmit = async (event: SubmitEvent) => {
    if (!(event.target instanceof HTMLFormElement) || !data.email) {
      return;
    }
    const form_data = new FormData(event.target);
    const enteredCode = form_data.get("code");
    // Loose type checking as enteredCode is not of type string
    if (enteredCode != correct_code) {
      response =
        "The entered code was incorrect. Please ensure you are logging in with the correct email address. Another email has been sent with a code for verification to the provided email.";
      correct_code = await invoke("send_verification_email", { email: data.email });
      return;
    }
    invoke("verify_user", { email: data.email });
    response = "You were successfully verified!";
  }
</script>

<div class="bg-gray-100 flex flex-col overflow-hidden gap-3 h-screen justify-between ">
  <TitleHeader />
  {#if !is_verified}
    <main class="h-screen flex flex-col justify-center text-orange-600 font-semibold relative">
      <div class="font-Nunito text-2xl text-center">
        To continue, you must first verify your email. <br />
        To verify your email, please enter in the 6 character code sent to your inbox
        from info.innermachinations@gmail.com.
        <br />
        <form on:submit={handleSubmit}>
          <label for="code">
            <input class="text-5xl h-14 my-4 text-center text-gray-950 border-black border-2 rounded-lg py-10 px-4"
              name="code"
              type="text"
              required/>
          </label>
        </form>
        {response}
        <br />
        <a class="font-Montserrat font-semibold text-gray-900 text-xl" href="/">
          Return to Home
        </a>
      </div>
    </main>
  {:else}
    <main class="h-screen text-orange-600 font-semibold py-10 relative">
      <div class="flex flex-col flex-wrap justify-center h-full text-center text-4xl my-4 content-center">
        Your email has already been verified!
        <a class="font-Montserrat font-semibold text-gray-900 text-xl" href="/">
          Return to Home
        </a>
      </div>
    </main>
  {/if}
  <div class="min-h-64">
  </div>
</div>