// Saves the workspace's automaton and alphabet a moment after each edit, so nothing is lost by leaving the
// workbench or closing the app. save_status tells the user where things stand.

import { get, writable, type Writable } from "svelte/store";
import { automaton, input_alphabet } from "$lib/utils/automataStores";
import { saveWorkspace } from "$lib/utils/savingWorkspaceFuncs";
import type { Automaton } from "./automaton";

export type SaveStatus = "saved" | "unsaved" | "saving" | "failed";
export const save_status: Writable<SaveStatus> = writable("saved");

// How long after the last edit to save, so a burst of edits is saved once
export const AUTOSAVE_DELAY = 800;

// What was last saved, to tell edits from the workspace merely being shown
let saved_automaton: Automaton | null = null;
let saved_alphabet: Array<string> | null = null;

let pending_save: ReturnType<typeof setTimeout> | undefined;
let save_in_progress: Promise<void> | null = null;
let stop_watching: (() => void) | null = null;

const hasChanges = (): boolean => get(automaton) !== saved_automaton || get(input_alphabet) !== saved_alphabet;

const scheduleSave = (): void => {
  clearTimeout(pending_save);
  pending_save = setTimeout(() => {
    // A failure has been shown to the user, and the next edit tries again
    saveNow().catch(() => {});
  }, AUTOSAVE_DELAY);
};

// Starts watching for edits to the workspace on show, which counts as saved
export function startAutosave(): () => void {
  stopAutosave();
  saved_automaton = get(automaton);
  saved_alphabet = get(input_alphabet);
  save_status.set("saved");

  const handleChange = (): void => {
    if (!hasChanges()) {
      // An edit undone before it was saved leaves nothing to save
      if (get(save_status) === "unsaved") {
        clearTimeout(pending_save);
        save_status.set("saved");
      }
      return;
    }
    if (get(save_status) !== "saving") {
      save_status.set("unsaved");
    }
    scheduleSave();
  };
  const stopWatchingAutomaton = automaton.subscribe(handleChange);
  const stopWatchingAlphabet = input_alphabet.subscribe(handleChange);
  stop_watching = () => {
    stopWatchingAutomaton();
    stopWatchingAlphabet();
  };
  return stopAutosave;
}

export function stopAutosave(): void {
  clearTimeout(pending_save);
  stop_watching?.();
  stop_watching = null;
}

// Saves now, unless autosave is watching and there is nothing new to save. Resolves once saved, and rejects
// if saving fails, so that leaving the workbench can wait for it
export async function saveNow(): Promise<void> {
  clearTimeout(pending_save);
  if (save_in_progress) {
    await save_in_progress.catch(() => {});
  }
  if (stop_watching && !hasChanges() && get(save_status) !== "failed") {
    return;
  }

  const automaton_to_save = get(automaton);
  const alphabet_to_save = get(input_alphabet);
  save_status.set("saving");

  const save = (async () => {
    const saved_alphabet_as_stored = await saveWorkspace(automaton_to_save, alphabet_to_save);
    saved_automaton = automaton_to_save;
    // The backend keeps only the alphabet's distinct single characters; show those, without that counting as an edit
    if (get(input_alphabet) === alphabet_to_save) {
      saved_alphabet = saved_alphabet_as_stored;
      input_alphabet.set(saved_alphabet_as_stored);
    } else {
      saved_alphabet = alphabet_to_save;
    }
  })();
  save_in_progress = save;

  try {
    await save;
  } catch (error) {
    save_status.set("failed");
    throw error;
  } finally {
    save_in_progress = null;
  }

  // Edits made while saving are saved next
  if (stop_watching && hasChanges()) {
    save_status.set("unsaved");
    scheduleSave();
  } else {
    save_status.set("saved");
  }
}
