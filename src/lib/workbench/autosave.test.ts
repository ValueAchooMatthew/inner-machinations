import { get } from "svelte/store";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { input_alphabet } from "$lib/utils/automataStores";
import { backend } from "../../tests/fakeBackend";
import { automaton, click, launchApp, message, openWorkspace, selectTool, undo } from "../../tests/workbenchDriver";
import { AUTOSAVE_DELAY, save_status, saveNow, startAutosave, stopAutosave } from "./autosave";

const saves = () => backend.callsTo("save_workspace");

// Lets the promises that are already settled run, as the event loop would between timer ticks
const settle = () => vi.advanceTimersByTimeAsync(0);

beforeEach(() => {
  vi.useFakeTimers();
  launchApp();
  openWorkspace({ states: ["200,200 start"], transitions: [] });
  backend.respond("save_workspace", (args) => args.alphabet);
  startAutosave();
  selectTool("state");
});

afterEach(() => {
  stopAutosave();
  vi.useRealTimers();
});

describe("autosave", () => {
  it("doesn't save a workspace just for opening it", async () => {
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves()).toEqual([]);
    expect(get(save_status)).toBe("saved");
  });

  it("saves a moment after the last edit, once", async () => {
    click(500, 200);
    expect(get(save_status)).toBe("unsaved");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY - 100);
    click(800, 200);
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY - 100);
    expect(saves()).toEqual([]);

    await vi.advanceTimersByTimeAsync(100);
    expect(saves()).toHaveLength(1);
    expect(saves()[0].automaton.states).toHaveLength(3);
    expect(get(save_status)).toBe("saved");
  });

  it("saves the alphabet when it changes", async () => {
    input_alphabet.set(["a", "b", "c"]);
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(saves()[0].alphabet).toEqual(["a", "b", "c"]);
  });

  it("keeps the alphabet as the backend saved it, without saving again", async () => {
    backend.respond("save_workspace", () => ["a", "b"]);
    input_alphabet.set(["a", "b", "", "a"]);
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(get(input_alphabet)).toEqual(["a", "b"]);
    expect(saves()).toHaveLength(1);
    expect(get(save_status)).toBe("saved");
  });

  it("counts an edit undone before it was saved as saved", async () => {
    click(500, 200);
    undo();
    expect(get(save_status)).toBe("saved");
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves()).toEqual([]);
    expect(get(save_status)).toBe("saved");
  });

  it("saves straight away when asked, and not again afterwards", async () => {
    click(500, 200);
    await saveNow();
    expect(saves()).toHaveLength(1);
    expect(get(save_status)).toBe("saved");
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves()).toHaveLength(1);
  });

  it("does nothing when asked to save with nothing changed", async () => {
    await saveNow();
    expect(saves()).toEqual([]);
  });

  it("says when saving failed, and tries again on the next edit", async () => {
    backend.respond("save_workspace", () => {
      throw "Couldn't save the workspace: disk I/O error";
    });
    click(500, 200);
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(get(save_status)).toBe("failed");
    expect(message()).toBe("Couldn't save the workspace: disk I/O error");

    backend.respond("save_workspace", (args) => args.alphabet);
    click(800, 200);
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(get(save_status)).toBe("saved");
    expect(saves().at(-1).automaton.states).toHaveLength(3);
  });

  it("rejects when asked to save and it fails, so leaving can wait", async () => {
    backend.respond("save_workspace", () => {
      throw "Couldn't save the workspace: disk I/O error";
    });
    click(500, 200);
    await expect(saveNow()).rejects.toThrow();
    expect(get(save_status)).toBe("failed");
  });

  it("saves edits made during a save once it finishes", async () => {
    let finishSave = () => {};
    backend.respond("save_workspace", (args) => new Promise((resolve) => {
      finishSave = () => resolve(args.alphabet);
    }));
    click(500, 200);
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(get(save_status)).toBe("saving");

    click(800, 200);
    finishSave();
    await settle();
    expect(get(save_status)).toBe("unsaved");

    backend.respond("save_workspace", (args) => args.alphabet);
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(saves()).toHaveLength(2);
    expect(saves()[1].automaton.states).toHaveLength(3);
    expect(get(save_status)).toBe("saved");
  });

  it("stops saving once stopped", async () => {
    stopAutosave();
    click(500, 200);
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves()).toEqual([]);
    expect(automaton().states).toHaveLength(2);
  });
});
