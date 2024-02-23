<script lang="ts">
	import { goto } from "$app/navigation";
	import { invoke } from "@tauri-apps/api/tauri";
	import { user_email } from "../../../lib/user";

	let response = "";

	const handleSubmit = async (event: SubmitEvent): Promise<void> =>{
		event.preventDefault();
		if(event.target instanceof HTMLFormElement){
			const data = new FormData(event.target);
			const email =  data.get("email")?.toString();
			const password = data.get("password")?.toString();
			if(!email || !password){
				return;
			}
			user_email.set(email);
			const isRegistered: boolean = await invoke("is_user_registered", {email: email, password: password});
			if(!isRegistered){
				invoke("register_user", {email: email, password: password});
				goto("/verification");	
			}else{
				const isCorrectLogin: boolean = await invoke("is_correct_log_in", {emailAddress: email, pwrd: password});
				const isVerified: boolean = await invoke("is_user_verified", {emailAddress: email});
				if(!isCorrectLogin){
					response = "Sorry, you've entered an incorrect password"
				}else{
					if(!isVerified){
						goto("/verification");
					}else{
						goto("/dashboard");
					}
				}
			}
		}else{
			console.log("There was an error parsing user information")
			return;
		}	
	}
</script>

<style lang="postcss">
	:global(html) {
	  background-color: theme(colors.gray.100);
	}
</style>

<main class="flex justify-center h-screen w-full align-middle">
	<form class="flex flex-col justify-center text-3xl font-semibold font-Nunito" on:submit={handleSubmit}>
		<label class="self-center my-4">
			Email
			<input required aria-required="true" class="border-black border-[2px] rounded-md px-2 py-1" name="email" type="email">
		</label>
		<label class="self-center">
			Password
			<input required aria-required="true" class="border-black border-[2px] rounded-md px-2 py-1 mr-[3.5rem]" name="password" type="password">
		</label>
		<span class="text-orange-600 mt-4 text-lg text-center w-full">
			{response}
		</span>
		<button class="mt-4 will-change-transform w-fit mx-auto bg-orange-600 text-gray-100 rounded-md py-3 px-7 border-black border-[1.5px]
		font-semibold hover:-translate-y-2 transition-all duration-[400ms] shadow-2xl">
			Sign Up/Log-in
		</button>
		<a class="text-center mt-2 font-Montserrat text-xl" href="/">Return to Home</a>
	</form>
</main>