<script lang="ts">
  import { setSessionValue } from "$lib/utils/miscUtils";
  import { goto } from "$app/navigation";
  import TitleHeader from "$lib/components/TitleHeader.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import LogIn from "./LogIn.svelte";
  import { LazyStore } from "@tauri-apps/plugin-store";
  import { email } from "$lib/utils/automataStores";

  async function authenticateUser() {
    const store = new LazyStore(".settings.dat");
    let email_obj = await store.get<{value: string}>("email");

    if(!email_obj) {
      return;
    }

    const user_email = email_obj.value;
    email.set(user_email);

    setSessionValue("email", user_email);

    const is_email_registered = await invoke("is_user_registered", {
      email: user_email
    });

    const is_email_verified = await invoke("is_user_verified", {
      email: user_email,
    });

    if(is_email_verified) {
      goto("../workspace/dashboard");
    } else if(is_email_registered) {
      goto("verification");
    }
  }

</script>

{#await authenticateUser()}
  <p>
    Loading...
  </p>
{:then _} 
<div class="bg-gray-100 flex flex-col overflow-hidden gap-3 h-screen justify-between ">
  <TitleHeader />
  <main class="z-50">
    <LogIn />
  </main>
  <div class="min-h-64">
  </div>
</div>
{/await}