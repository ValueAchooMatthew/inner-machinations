import { get } from "svelte/store";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { dialogue_to_user } from "$lib/utils/automataStores";
import { backend } from "../../tests/fakeBackend";
import { AUTOSAVE_DELAY } from "$lib/workbench/autosave";
import {
  chooseRegex, createRegex, openRegex, regex, regex_name, regex_save_status, renameRegex, saveRegexNow,
  stopRegexAutosave,
} from "./savedRegex";

const saves = () => backend.callsTo("save_regex");
const message = () => get(dialogue_to_user);

// Lets the promises that are already settled run, as the event loop would between timer ticks
const settle = () => vi.advanceTimersByTimeAsync(0);

const savedRegex = (regex_name: string, regex = "") => ({ regex_name, regex, date_of_last_update: "2026-10-20 12:00:00" });

beforeEach(() => {
  stopRegexAutosave();
  regex.set("");
  regex_name.set("");
  regex_save_status.set("saved");
  dialogue_to_user.set(null);
});

describe("creating", () => {
  it("names a new regular expression after the ones already saved, and chooses it", async () => {
    backend.respond("get_saved_regexes", () => [savedRegex("Untitled Regular Expression"), savedRegex("Other")]);
    await createRegex();
    expect(backend.callsTo("create_regex")).toEqual([{ regexName: "Untitled Regular Expression 2" }]);
    expect(get(regex_name)).toBe("Untitled Regular Expression 2");
    expect(sessionStorage.getItem("regex_name")).toBe("Untitled Regular Expression 2");
  });

  it("chooses nothing when it can't be created", async () => {
    backend.respond("get_saved_regexes", () => []);
    backend.respond("create_regex", () => {
      throw "Couldn't create the regular expression: disk I/O error";
    });
    await expect(createRegex()).rejects.toThrow();
    expect(get(regex_name)).toBe("");
    expect(message()).toBe("Couldn't create the regular expression: disk I/O error");
  });
});

describe("opening", () => {
  it("shows the chosen regular expression as it was saved", async () => {
    backend.respond("retrieve_regex", (args) => savedRegex(args.regexName, "a(b+c)*"));
    chooseRegex("Mine");
    await openRegex();
    expect(backend.callsTo("retrieve_regex")).toEqual([{ regexName: "Mine" }]);
    expect(get(regex)).toBe("a(b+c)*");
    expect(get(regex_save_status)).toBe("saved");
  });

  it("says why when it can't be opened, and saves nothing", async () => {
    vi.useFakeTimers();
    backend.respond("retrieve_regex", () => {
      throw "There is no regular expression called \"Gone\"";
    });
    chooseRegex("Gone");
    await expect(openRegex()).rejects.toThrow();
    expect(message()).toBe("There is no regular expression called \"Gone\"");
    regex.set("a");
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves()).toEqual([]);
    vi.useRealTimers();
  });
});

describe("autosave", () => {
  beforeEach(async () => {
    vi.useFakeTimers();
    backend.respond("retrieve_regex", (args) => savedRegex(args.regexName, "a"));
    chooseRegex("Mine");
    await openRegex();
  });

  afterEach(() => {
    stopRegexAutosave();
    vi.useRealTimers();
  });

  it("doesn't save a regular expression just for opening it", async () => {
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves()).toEqual([]);
    expect(get(regex_save_status)).toBe("saved");
  });

  it("saves a moment after the last edit, once", async () => {
    regex.set("ab");
    expect(get(regex_save_status)).toBe("unsaved");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY - 100);
    regex.set("ab*");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY - 100);
    expect(saves()).toEqual([]);

    await vi.advanceTimersByTimeAsync(100);
    expect(saves()).toEqual([{ regexName: "Mine", regex: "ab*" }]);
    expect(get(regex_save_status)).toBe("saved");
  });

  it("saves what was typed even when it can't be built yet", async () => {
    regex.set("a+");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(saves()).toEqual([{ regexName: "Mine", regex: "a+" }]);
  });

  it("counts typing back what was saved as saved", async () => {
    regex.set("ab");
    regex.set("a");
    expect(get(regex_save_status)).toBe("saved");
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves()).toEqual([]);
  });

  it("saves straight away when asked, and not again afterwards", async () => {
    regex.set("ab");
    await saveRegexNow();
    expect(saves()).toHaveLength(1);
    expect(get(regex_save_status)).toBe("saved");
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves()).toHaveLength(1);
  });

  it("does nothing when asked to save with nothing changed", async () => {
    await saveRegexNow();
    expect(saves()).toEqual([]);
  });

  it("says when saving failed, and tries again on the next edit", async () => {
    backend.respond("save_regex", () => {
      throw "Couldn't save the regular expression: disk I/O error";
    });
    regex.set("ab");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(get(regex_save_status)).toBe("failed");
    expect(message()).toBe("Couldn't save the regular expression: disk I/O error");

    backend.respond("save_regex", () => undefined);
    regex.set("abc");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(get(regex_save_status)).toBe("saved");
    expect(saves().at(-1)).toEqual({ regexName: "Mine", regex: "abc" });
  });

  it("tries again when asked to save after failing", async () => {
    backend.respond("save_regex", () => {
      throw "Couldn't save the regular expression: disk I/O error";
    });
    regex.set("ab");
    await expect(saveRegexNow()).rejects.toThrow();
    expect(get(regex_save_status)).toBe("failed");

    backend.respond("save_regex", () => undefined);
    await saveRegexNow();
    expect(saves()).toEqual([{ regexName: "Mine", regex: "ab" }, { regexName: "Mine", regex: "ab" }]);
    expect(get(regex_save_status)).toBe("saved");
  });

  it("counts typing back what was saved as saved after failing to save something else", async () => {
    backend.respond("save_regex", () => {
      throw "Couldn't save the regular expression: disk I/O error";
    });
    regex.set("ab");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(get(regex_save_status)).toBe("failed");

    regex.set("a");
    expect(get(regex_save_status)).toBe("saved");
    await saveRegexNow();
    expect(saves()).toHaveLength(1);
  });

  it("saves edits made during a save once it finishes", async () => {
    let finishSave = () => {};
    backend.respond("save_regex", () => new Promise<void>((resolve) => {
      finishSave = resolve;
    }));
    regex.set("ab");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(get(regex_save_status)).toBe("saving");

    regex.set("abc");
    finishSave();
    await settle();
    expect(get(regex_save_status)).toBe("unsaved");

    backend.respond("save_regex", () => undefined);
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(saves().map((save) => save.regex)).toEqual(["ab", "abc"]);
    expect(get(regex_save_status)).toBe("saved");
  });

  it("saves nothing more when stopped during a save", async () => {
    let finishSave = () => {};
    backend.respond("save_regex", () => new Promise<void>((resolve) => {
      finishSave = resolve;
    }));
    regex.set("ab");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    regex.set("abc");
    stopRegexAutosave();
    finishSave();
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    expect(saves().map((save) => save.regex)).toEqual(["ab"]);
  });

  it("stops saving once stopped", async () => {
    stopRegexAutosave();
    regex.set("ab");
    await vi.advanceTimersByTimeAsync(5 * AUTOSAVE_DELAY);
    await saveRegexNow();
    expect(saves()).toEqual([]);
  });
});

describe("renaming", () => {
  beforeEach(async () => {
    backend.respond("retrieve_regex", (args) => savedRegex(args.regexName, "a"));
    backend.respond("does_regex_name_exist", (args) => args.regexName === "Taken");
    chooseRegex("Mine");
    await openRegex();
  });

  afterEach(stopRegexAutosave);

  it("saves, then renames, and remembers the new name for the session", async () => {
    regex.set("ab");
    await renameRegex("Renamed", { explain: true });
    expect(backend.calls.slice(1).map((call) => call.command)).toEqual(["does_regex_name_exist", "save_regex", "update_regex_name"]);
    expect(backend.callsTo("update_regex_name")).toEqual([{ originalRegexName: "Mine", newRegexName: "Renamed" }]);
    expect(get(regex_name)).toBe("Renamed");
    expect(sessionStorage.getItem("regex_name")).toBe("Renamed");
  });

  it("won't take a blank name, and says so when asked to explain", async () => {
    await renameRegex("   ");
    expect(message()).toBeNull();
    await renameRegex("", { explain: true });
    expect(message()).toBe("A regular expression needs a name");
    expect(backend.callsTo("update_regex_name")).toEqual([]);
    expect(get(regex_name)).toBe("Mine");
  });

  it("won't take another regular expression's name, and says so when asked to explain", async () => {
    await renameRegex("Taken");
    expect(message()).toBeNull();
    await renameRegex("Taken", { explain: true });
    expect(message()).toBe("There is already a regular expression called \"Taken\"");
    expect(backend.callsTo("update_regex_name")).toEqual([]);
  });

  it("does nothing given the name it has", async () => {
    await renameRegex("Mine", { explain: true });
    expect(backend.calls.slice(1)).toEqual([]);
    expect(message()).toBeNull();
  });

  it("keeps the old name when renaming fails", async () => {
    backend.respond("update_regex_name", () => {
      throw "Couldn't rename the regular expression: disk I/O error";
    });
    await expect(renameRegex("Renamed", { explain: true })).rejects.toThrow();
    expect(get(regex_name)).toBe("Mine");
    expect(sessionStorage.getItem("regex_name")).toBe("Mine");
    expect(message()).toBe("Couldn't rename the regular expression: disk I/O error");
  });

  it("keeps saving under the new name", async () => {
    vi.useFakeTimers();
    await renameRegex("Renamed");
    regex.set("ab");
    await vi.advanceTimersByTimeAsync(AUTOSAVE_DELAY);
    expect(saves()).toEqual([{ regexName: "Renamed", regex: "ab" }]);
    vi.useRealTimers();
  });
});
