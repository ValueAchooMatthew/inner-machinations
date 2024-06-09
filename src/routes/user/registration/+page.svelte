<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/tauri";

  let response = "";

  const handleSubmit = async (event: SubmitEvent): Promise<void> => {
    event.preventDefault();
    if (!(event.target instanceof HTMLFormElement)) {
      console.log("There was an error submitting user information");
      return;
    }
    const data = new FormData(event.target);
    const email = data.get("email")?.toString();
    const password = data.get("password")?.toString();
    if (!email || !password) {
      return;
    }
    const isRegistered: boolean = await invoke("is_user_registered", {
      email: email,
      password: password,
    });
    if (!isRegistered) {
      invoke("register_user", { email: email, password: password });
      goto("verification");
      return;
    }
    const isCorrectLogin: boolean = await invoke("is_correct_log_in", {
      emailAddress: email,
      pwrd: password,
    });
    const isVerified: boolean = await invoke("is_user_verified", {
      emailAddress: email,
      pwrd: password,
    });
    if (!isCorrectLogin) {
      response = "Sorry, you've entered an incorrect password";
      return;
    }
    if (!isVerified) {
      goto("verification");
      return;
    }
    document.cookie = "email" + "=" + email + "; path=/";
    goto("../workspace/dashboard");
    return;
  };
</script>

<svg
  xmlns="http://www.w3.org/2000/svg"
  class="absolute -top-12 -z-10"
  viewBox="0 0 1440 320"
  ><path
    fill="#ff5500"
    fill-opacity="1"
    d="M0,256L60,234.7C120,213,240,171,360,170.7C480,171,600,213,720,213.3C840,213,960,171,1080,176C1200,181,1320,235,1380,261.3L1440,288L1440,0L1380,0C1320,0,1200,0,1080,0C960,0,840,0,720,0C600,0,480,0,360,0C240,0,120,0,60,0L0,0Z"
  ></path></svg
>
<div
  class="absolute w-full z-50 top-10 mb-6 text-gray-100 font-outline-2 font-Montserrat"
>
  <h1 class="text-center xl:text-7xl text-6xl font-semibold">
    Inner Machinations
  </h1>
  <h2 class="italic text-center xl:text-2xl text-2xl mt-2 font-outline-2">
    A desktop application to create your very own DFA's and NFA's
  </h2>
</div>
<main class="flex justify-center h-screen w-full align-middle">
  <form
    class="flex flex-col justify-center text-3xl font-semibold font-Nunito"
    on:submit={handleSubmit}
  >
    <label class="self-center my-4">
      Email
      <input
        required
        aria-required="true"
        class="border-black border-[2px] rounded-md px-2 py-1"
        name="email"
        type="email"
      />
    </label>
    <label class="self-center">
      Password
      <input
        required
        aria-required="true"
        class="border-black border-[2px] rounded-md px-2 py-1 mr-[3.5rem]"
        name="password"
        type="password"
      />
    </label>
    <span class="text-orange-600 mt-4 text-lg text-center w-full">
      {response}
    </span>
    <button
      class="mt-4 will-change-transform w-fit mx-auto bg-orange-600 text-gray-100 rounded-md py-3 px-7 border-black border-[1.5px]
		font-semibold hover:-translate-y-2 transition-all duration-[400ms] shadow-2xl"
    >
      Sign Up/Log-in
    </button>
    <a class="text-center mt-2 font-Montserrat text-xl" href="/"
      >Return to Home</a
    >
  </form>
</main>
<svg
  xmlns="http://www.w3.org/2000/svg"
  class="absolute -bottom-12 -z-10"
  viewBox="0 0 1440 320"
  ><path
    fill="#ff5500"
    fill-opacity="1"
    d="M0,160L60,181.3C120,203,240,245,360,240C480,235,600,181,720,160C840,139,960,149,1080,165.3C1200,181,1320,203,1380,213.3L1440,224L1440,320L1380,320C1320,320,1200,320,1080,320C960,320,840,320,720,320C600,320,480,320,360,320C240,320,120,320,60,320L0,320Z"
  ></path></svg
>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
