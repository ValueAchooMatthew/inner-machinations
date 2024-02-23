<script lang="ts">
    import { user_email } from "$lib/user";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";

    let email: String = "";

    user_email.subscribe((inputted_email)=>{
        email = inputted_email;
    });

    $: code = "";
    let isRegistered = false;
    let response = "";
    onMount(async () =>{
        isRegistered = await invoke("is_user_verified", {emailAddress: email});
        if(!isRegistered){
            code = await invoke("send_email", {emailAddress: email});
        }

    })
    // TODO: Prevent emails from being sent if an attempt hasnt yet been made

    const handleSubmit = async (event: SubmitEvent) =>{
        if(event.target instanceof HTMLFormElement){
            const data = new FormData(event.target);
            const enteredCode = data.get("code");
            if(enteredCode == code){
                invoke("verify_user", {emailAddress: "matthewtamerfarah@gmail.com"})
                response = "You were successfully verified!";

            }else{
                response = "The entered code was incorrect. Please ensure you are logging in with the correct email address. Another email has been sent with a code for verification to the provided email."
                code = await invoke("send_email", {emailAddress: email});
            }
        }else{
            return;
        }
    }

</script>
{#if !isRegistered }
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