<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";
  import { email } from "$lib/utils/svelteStores"
  import { Store } from "tauri-plugin-store-api";

  let response = "";

  const handleSubmit = async (event: SubmitEvent): Promise<void> => {
    event.preventDefault();

    if (!(event.target instanceof HTMLFormElement)) {
      console.log("There was an error submitting user information");
      return;
    }

    const data = new FormData(event.target);
    const user_email = data.get("email")?.toString();
    const password = data.get("password")?.toString();
    document.cookie = "email" + "=" + user_email + "; path=/";

    if (!user_email || !password) {
      return;
    }
    email.set(
      user_email
    )
    
    const is_registered: boolean = await invoke("is_user_registered", {
      email: user_email,
    });

    if (!is_registered) {
      invoke("register_user", { email: user_email, password: password });
      goto("verification");
      return;
    }

    const is_correct_login: boolean = await invoke("is_correct_log_in", {
      email: user_email,
      password: password,
    });

    const is_verified: boolean = await invoke("is_user_verified", {
      email: user_email,
    });

    if (!is_correct_login) {
      response = "Sorry, you've entered an incorrect password";
      return;
    }

    if (!is_verified) {
      goto("verification");
      return;
    }

    const store = new Store(".settings.dat");
    // Store always expects object as argument to be stored
    await store.set("email", {value: $email});

    goto("../workspace/dashboard");
    return;
  };
</script>


<form class="flex flex-col justify-center text-3xl font-semibold font-Nunito align-middle"
  on:submit={handleSubmit}>
  <label class="self-center my-4">
    Email
    <input class="border-black border-[2px] rounded-md px-2 py-1"
      required
      aria-required="true"
      name="email"
      type="email"/>
  </label>
  <label class="self-center">
  Password
  <input class="border-black border-[2px] rounded-md px-2 py-1 mr-[3.5rem]"
    required
    aria-required="true"
    name="password"
    type="password"/>
  </label>
  <span class="text-orange-600 mt-4 text-lg text-center w-full">
    {response}
  </span>
  <button class="mt-4 will-change-transform w-fit mx-auto bg-orange-600 text-gray-100 rounded-md py-3 px-7 border-black border-[1.5px]
    font-semibold hover:-translate-y-2 transition-all duration-[400ms] shadow-2xl">
      Sign Up/Log-in
    </button>
  <a class="text-center mt-2 font-Montserrat text-xl" href="../">
    Return to Home
  </a>
</form>