import type { WorkspaceType } from "$lib/types/enums";
import { writable, type Writable } from "svelte/store";

export const current_regex: Writable<string> = writable("");
export const current_workspace_type: Writable<WorkspaceType | null> = writable(null);