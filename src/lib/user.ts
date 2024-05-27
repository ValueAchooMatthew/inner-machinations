import { writable, type Writable } from "svelte/store";

export const is_a_user_logged_in: Writable<boolean> = writable();
export const user_email: Writable<string> = writable();
