import { writable, type Writable } from "svelte/store";

export const user_email: Writable<string> = writable();
export const user_password: Writable<string> = writable();