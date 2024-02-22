<script lang="ts">
	import { goto } from "$app/navigation";


	$: response = "";

	import { invoke } from "@tauri-apps/api/tauri";
	const handleSubmit = async (event: SubmitEvent): Promise<void> =>{
		response = "";
		event.preventDefault();
		if(event.target instanceof HTMLFormElement){
			const data = new FormData(event.target);
			const email = data.get("email");
			const password = data.get("password");
			const isRegistered: boolean = await invoke("is_user_registered", {email: email, password: password});
			if(isRegistered){
				response = "This email is already registered. Click 'Log In' to sign in with your email"

			}else{

				invoke("handle_registration_event", {email: email, password: password});
				goto("/registration");

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

<body class="h-screen flex justify-center align-middle">

	<form class="self-center text-3xl font-semibold xl:mt-52 mt-32 font-Nunito" on:submit={handleSubmit}>
		<div class="flex flex-col justify-around">
			<label  class="self-center my-4">
				Email
				<input required aria-required="true" class="border-black border-[2px] rounded-md px-2 py-1" name="email" type="email">
			</label>
			<label class="self-center">
				Password
				<input required aria-required="true" class="border-black border-[2px] rounded-md px-2 py-1 mr-[3.2rem]" name="password" type="password">
			</label>
		</div>
		<div class="text-center mt-8 font-Montserrat text-lg italic mb-4">
			{response}
		</div>
		<div class="flex justify-center text-3xl mt-10">
			<button class="will-change-transform bg-orange-600 text-gray-100 rounded-md py-3 px-7 border-black border-[1.5px]
			font-semibold hover:-translate-y-2 transition-all duration-[400ms] shadow-2xl">
				Sign Up
			</button>
		</div>
		<div class="text-center mt-4">
			<span class=" text-lg italic text-orange-500">
				Or <a class="underline " href="/registration">Log In</a>
			</span>
		</div>
	
	</form>

</body>
