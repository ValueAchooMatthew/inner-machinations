// The saved regular expression open in the regular expression builder. Like the workbench's workspace
// (src/lib/workbench/autosave.ts), it is saved a moment after each edit, and regex_save_status tells the
// user where things stand.

import { get, writable, type Writable } from "svelte/store";
import { dialogue_to_user } from "$lib/utils/automataStores";
import { invokeCommand } from "$lib/utils/backend";
import { firstFreeName, getSessionValue, setSessionValue } from "$lib/utils/miscUtils";
import { AUTOSAVE_DELAY, type SaveStatus } from "$lib/workbench/autosave";
import type { RegexData } from "$lib/types/interfaces";

export const regex_name: Writable<string> = writable(getSessionValue("regex_name"));
// As typed, which needn't be a regular expression that can be built yet
export const regex: Writable<string> = writable("");
export const regex_save_status: Writable<SaveStatus> = writable("saved");

// What was last saved, to tell edits from the regular expression merely being shown
let saved_regex: string | null = null;

let pending_save: ReturnType<typeof setTimeout> | undefined;
let save_in_progress: Promise<void> | null = null;
let stop_watching: (() => void) | null = null;

const tell = (message: string): void => dialogue_to_user.set(message);

// Autosave can be stopped while a save is under way
const isWatching = (): boolean => stop_watching !== null;

const scheduleSave = (): void => {
  clearTimeout(pending_save);
  pending_save = setTimeout(() => {
    // A failure has been shown to the user, and the next edit tries again
    saveRegexNow().catch(() => {});
  }, AUTOSAVE_DELAY);
};

// Makes the named regular expression the one the builder opens
export function chooseRegex(name: string): void {
  setSessionValue("regex_name", name);
  regex_name.set(name);
  dialogue_to_user.set(null);
}

// A new regular expression is called "Untitled Regular Expression", numbered if that name is taken, and is
// chosen to be opened
export async function createRegex(): Promise<void> {
  const saved_regexes: Array<RegexData> = await invokeCommand("get_saved_regexes");
  const new_regex_name = firstFreeName("Untitled Regular Expression", saved_regexes.map((saved) => saved.regex_name));
  await invokeCommand("create_regex", { regexName: new_regex_name });
  chooseRegex(new_regex_name);
}

// Shows the chosen regular expression as it was saved, and starts watching for edits to it
export async function openRegex(): Promise<void> {
  stopRegexAutosave();
  const saved: RegexData = await invokeCommand("retrieve_regex", { regexName: get(regex_name) });
  saved_regex = saved.regex;
  regex.set(saved.regex);
  regex_save_status.set("saved");

  stop_watching = regex.subscribe((current) => {
    if (current === saved_regex) {
      // Typing back what was saved leaves nothing to save, even after failing to save something else
      if (get(regex_save_status) === "unsaved" || get(regex_save_status) === "failed") {
        clearTimeout(pending_save);
        regex_save_status.set("saved");
      }
      return;
    }
    if (get(regex_save_status) !== "saving") {
      regex_save_status.set("unsaved");
    }
    scheduleSave();
  });
}

export function stopRegexAutosave(): void {
  clearTimeout(pending_save);
  stop_watching?.();
  stop_watching = null;
}

// Saves now, if anything is open and there is something new to save. Resolves once saved, and rejects if
// saving fails, so that leaving the builder can wait for it
export async function saveRegexNow(): Promise<void> {
  clearTimeout(pending_save);
  if (save_in_progress) {
    await save_in_progress.catch(() => {});
  }
  if (!isWatching() || get(regex) === saved_regex) {
    return;
  }

  const regex_to_save = get(regex);
  regex_save_status.set("saving");

  const save = (async () => {
    await invokeCommand("save_regex", { regexName: get(regex_name), regex: regex_to_save });
    saved_regex = regex_to_save;
  })();
  save_in_progress = save;

  try {
    await save;
  } catch (error) {
    regex_save_status.set("failed");
    throw error;
  } finally {
    save_in_progress = null;
  }

  // Edits made while saving are saved next
  if (isWatching() && get(regex) !== saved_regex) {
    regex_save_status.set("unsaved");
    scheduleSave();
  } else {
    regex_save_status.set("saved");
  }
}

// Renames the open regular expression, saving it first. A name that is blank, or that another regular
// expression has, is left alone; `explain` says why, for when the user has asked to rename rather than
// just stopped typing
export async function renameRegex(new_name: string, { explain = false } = {}): Promise<void> {
  const current_name = get(regex_name);
  if (new_name === current_name) {
    return;
  }
  if (!new_name.trim()) {
    if (explain) {
      tell("A regular expression needs a name");
    }
    return;
  }
  if (await invokeCommand<boolean>("does_regex_name_exist", { regexName: new_name })) {
    if (explain) {
      tell(`There is already a regular expression called "${new_name}"`);
    }
    return;
  }
  await saveRegexNow();
  await invokeCommand("update_regex_name", { originalRegexName: current_name, newRegexName: new_name });
  regex_name.set(new_name);
  setSessionValue("regex_name", new_name);
}
