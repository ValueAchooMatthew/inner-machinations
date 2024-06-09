/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      fontFamily: {
        Nunito: ["Nunito", "serif"],
        Montserrat: ["Montserrat", "serif"],
      },
    },
  },
  plugins: [],
};
