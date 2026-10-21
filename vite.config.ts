import { sveltekit } from "@sveltejs/kit/vite";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vitest/config";

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  // tauri dev loads the frontend from this port, so fail rather than quietly use another one
  server: {
    port: 5173,
    strictPort: true,
  },
  test: {
    include: ["src/**/*.test.ts"],
    environment: "node",
    setupFiles: ["src/tests/setup.ts"],
  },
});
