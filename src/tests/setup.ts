import { beforeEach, vi } from "vitest";
import { backend } from "./fakeBackend";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (command: string, args?: unknown) => backend.invoke(command, args),
}));

beforeEach(() => {
  backend.reset();
});
