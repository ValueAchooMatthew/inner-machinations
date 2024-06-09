export const load = async ({ cookies }) => {
  const email = cookies.get("email");
  return {
    email,
  };
};
