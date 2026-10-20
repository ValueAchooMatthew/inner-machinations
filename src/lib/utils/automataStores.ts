import { writable, type Writable } from "svelte/store";
import { Automata } from "../types/enums";
import { getSessionValue } from "./miscUtils";
import { emptyAutomaton, type Automaton } from "$lib/workbench/automaton";

// The automaton on the workbench: its states, and the connections between them
export const automaton: Writable<Automaton> = writable(emptyAutomaton());

// What clicking the canvas does
export type Tool = "state" | "start" | "final" | "connection" | "epsilon" | "select";
export const tool: Writable<Tool> = writable("state");

// While a connection is being drawn, between its two clicks, the state it starts from
export const connection_start: Writable<number | null> = writable(null);

// The state or connection picked with the select tool
export type Selection = { kind: "state"; id: number } | { kind: "connection"; id: number };
export const selection: Writable<Selection | null> = writable(null);

export const input_alphabet: Writable<Array<string>> = writable(new Array("a", "b"));
export const type_of_automata: Writable<Automata> = writable(Automata.DFA);
export const dialogue_to_user: Writable<string | null> = writable(null);
export const should_strict_check: Writable<boolean> = writable(false);
export const should_show_string_traversal: Writable<boolean> = writable(false);
export const default_connection_character: Writable<string> = writable("a");
export const workspace_name: Writable<string> = writable(getSessionValue("workspace_name"));
