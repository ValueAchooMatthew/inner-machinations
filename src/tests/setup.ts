import { beforeEach, vi } from "vitest";
import { backend } from "./fakeBackend";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (command: string, args?: unknown) => backend.invoke(command, args),
}));

// The pages pass values to each other through sessionStorage, which Node doesn't have
const sessionValues = new Map<string, string>();
vi.stubGlobal("sessionStorage", {
  getItem: (name: string) => sessionValues.get(name) ?? null,
  setItem: (name: string, value: string) => sessionValues.set(name, value),
  removeItem: (name: string) => sessionValues.delete(name),
});

beforeEach(() => {
  backend.reset();
  sessionValues.clear();
});
