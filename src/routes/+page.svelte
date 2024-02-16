<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	const handleSubmit = async (event: SubmitEvent): Promise<void> =>{
		event.preventDefault();
		if(event.target instanceof HTMLFormElement){
			const data = new FormData(event.target);
			const email = data.get("email");
			const password = data.get("password");
			invoke("register_user", {em: email, pswd: password});

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

<body class="h-screen py-10 relative">
	<svg xmlns="http://www.w3.org/2000/svg" class="absolute top-0 -z-10" viewBox="0 0 1440 320"><path fill="#ff5500" fill-opacity="1" d="M0,256L60,234.7C120,213,240,171,360,170.7C480,171,600,213,720,213.3C840,213,960,171,1080,176C1200,181,1320,235,1380,261.3L1440,288L1440,0L1380,0C1320,0,1200,0,1080,0C960,0,840,0,720,0C600,0,480,0,360,0C240,0,120,0,60,0L0,0Z"></path></svg>
	<div class="mb-6 text-gray-100">
		<h1 class="text-center text-6xl font-semibold">
			Inner Machinations
		</h1>
		<h2 class="italic text-center text-2xl mt-4">
			A desktop application to create your very own DFA's and NFA's
		</h2>
	</div>
	
	<form class="text-3xl font-semibold mt-32" on:submit={handleSubmit}>
		<div class="flex flex-col justify-around">
			<label  class="self-center my-4">
				Email
				<input required aria-required="true" class="border-black border-[2px] rounded-md px-2 py-1" name="email" type="email">
			</label>
			<label class="self-center">
				Password
				<input required aria-required="true" class="border-black border-[2px] rounded-md px-2 py-1 mr-[3.2rem]" name="password" type="password">
			</label>
		</div >
		<div class="flex justify-center text-5xl mt-10">
			<button class="bg-orange-600 text-gray-100 rounded-md pb-4 pt-2 px-7 border-black border-[1.5px]
			font-semibold hover:bg-gray-100 hover:text-orange-600 transition-all duration-[400ms] shadow-2xl">
				Log In
			</button>
		</div>
	</form>
	<div class="text-center mt-4">
		<span class=" text-lg italic text-orange-500">
			Or <a class="underline " href="/registration">Register</a>
		</span>
	</div>

	<svg xmlns="http://www.w3.org/2000/svg" class="absolute bottom-0 -z-10" viewBox="0 0 1440 320"><path fill="#ff5500" fill-opacity="1" d="M0,160L60,181.3C120,203,240,245,360,240C480,235,600,181,720,160C840,139,960,149,1080,165.3C1200,181,1320,203,1380,213.3L1440,224L1440,320L1380,320C1320,320,1200,320,1080,320C960,320,840,320,720,320C600,320,480,320,360,320C240,320,120,320,60,320L0,320Z"></path></svg>
</body>
