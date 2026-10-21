import { invokeCommand } from "$lib/utils/backend";
import { workspace_name } from "./automataStores";
import type { Automaton } from "$lib/workbench/automaton";
import { get } from "svelte/store";

// Saves the automaton and the alphabet, and returns the alphabet as saved: the backend keeps only its distinct
// single characters. Autosave decides when (src/lib/workbench/autosave.ts)
export const saveWorkspace = async (automaton: Automaton, alphabet: Array<string>): Promise<Array<string>> => {
  return await invokeCommand("save_workspace", {
    workspaceName: get(workspace_name),
    automaton,
    alphabet
  });
};
