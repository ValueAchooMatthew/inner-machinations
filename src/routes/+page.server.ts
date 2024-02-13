import type { Actions } from './$types';
export const prerender = false;
import { invoke } from "@tauri-apps/api/tauri";

export const actions = {
	default: async (event) => {
		// TODO log the user in
        const data = await event.request.formData();
		const email = data.get("email");
		const password = data.get("password");
		if(email && password){

			// currently crashes when invoked, must fix.
			// invoke("retrieve_info");

		}
		console.log(data)

	},
} satisfies Actions;