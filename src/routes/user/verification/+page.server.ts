export const load = ({ cookies }) => {
  const email = cookies.get("email");
  console.log(cookies);

  return {
    email,
  };
};
