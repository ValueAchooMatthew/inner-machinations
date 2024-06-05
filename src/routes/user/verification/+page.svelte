<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    export let data;

    $: code = "";
    let is_verified = false;
    let response = "";
    onMount(async () =>{
        if(data === null || data.email === null){
            console.log("The user's email is null");
            return;
        }
        is_verified = await invoke("is_user_verified", {emailAddress: data.email});
        if(!is_verified){
            code = await invoke("send_email", {emailAddress: data.email});
        }

    })

    const handleSubmit = async (event: SubmitEvent) =>{
        if(!(event.target instanceof HTMLFormElement) || !data || !data.email){
            return;
        }
        const form_data = new FormData(event.target);
        const enteredCode = form_data.get("code");
        if(enteredCode != code){
            response = "The entered code was incorrect. Please ensure you are logging in with the correct email address. Another email has been sent with a code for verification to the provided email."
            code = await invoke("send_email", {emailAddress: data.email});
            return;
        }
        invoke("verify_user", {emailAddress: "matthewtamerfarah@gmail.com"})
            response = "You were successfully verified!";
    }

</script>
<svg xmlns="http://www.w3.org/2000/svg" class="absolute -top-12 -z-10" viewBox="0 0 1440 320"><path fill="#ff5500" fill-opacity="1" d="M0,256L60,234.7C120,213,240,171,360,170.7C480,171,600,213,720,213.3C840,213,960,171,1080,176C1200,181,1320,235,1380,261.3L1440,288L1440,0L1380,0C1320,0,1200,0,1080,0C960,0,840,0,720,0C600,0,480,0,360,0C240,0,120,0,60,0L0,0Z"></path></svg>
<div class="absolute w-full z-50 top-10 mb-6 text-gray-100 font-outline-2 font-Montserrat">
    <h1 class="text-center xl:text-7xl text-6xl font-semibold">
        Inner Machinations
    </h1>
    <h2 class="italic text-center xl:text-2xl text-2xl mt-2 font-outline-2">
        A desktop application to create your very own DFA's and NFA's
    </h2>
</div>
{#if !is_verified }
<main class="h-screen text-orange-600 font-semibold py-10 relative">
    <div class="flex flex-wrap justify-center h-full text-center content-center">
        <div class="font-Nunito text-2xl text-center">
            To continue, you must first verify your email. <br>
            To verify your email, please enter in the 6 character code sent to your inbox from info.innermachinations@gmail.com.
            <br>
            <form on:submit={handleSubmit}>
                <label for="code">
                    <input class="text-5xl h-14 my-4 text-center text-gray-950 border-black border-2 rounded-lg py-10 px-4" name="code" type="text" required>
                </label>
            </form>
            {response}
            <br>
            <a class="font-Montserrat font-semibold text-gray-900 text-xl" href="/">Return to Home</a>
        </div>
    </div>

</main>
{:else}
<main class="h-screen text-orange-600 font-semibold py-10 relative">
    <div class="flex flex-col flex-wrap justify-center h-full text-center text-4xl my-4 content-center">
        Your email has already been verified!
        <a class="font-Montserrat font-semibold text-gray-900 text-xl" href="/">Return to Home</a>
    </div>
</main>

{/if}
<svg xmlns="http://www.w3.org/2000/svg" class="absolute -bottom-12 -z-10" viewBox="0 0 1440 320"><path fill="#ff5500" fill-opacity="1" d="M0,160L60,181.3C120,203,240,245,360,240C480,235,600,181,720,160C840,139,960,149,1080,165.3C1200,181,1320,203,1380,213.3L1440,224L1440,320L1380,320C1320,320,1200,320,1080,320C960,320,840,320,720,320C600,320,480,320,360,320C240,320,120,320,60,320L0,320Z"></path></svg>