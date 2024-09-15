import { writable, type Writable } from "svelte/store";
import { getCookie } from "./miscUtils";

export const dialogue_to_user: Writable<string | null> = writable(null);
export const workspace_name: Writable<string> = writable(getCookie("workspace_name"));
export const email: Writable<string> = writable(getCookie("email"));
