
export const load = (async ({ cookies }) => {
	const workspace_name = cookies.get("workspace_name");
	const email = cookies.get("email");
	return {
		workspace_name,
		email
	};
  });