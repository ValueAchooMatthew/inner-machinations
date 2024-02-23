import { writable, type Writable } from "svelte/store";

export const user_email: Writable<String> = writable();
