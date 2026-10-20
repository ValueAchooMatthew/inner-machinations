// Drives the workbench the way its components do, so workbench.test.ts can describe behaviour in terms
// of tools, clicks and what ends up on the canvas, rather than in terms of stores. When the workbench's
// internals change, this file follows them and the tests stay as they are.
//
// Automata are written as readable strings:
//   states:      "200,200", "200,200 start", "500,200 final", "500,200 start final"
//   transitions: "200,200 -a-> 500,200", with ϵ for epsilon

import { get } from "svelte/store";
import { Automata } from "$lib/types/enums";
import type { State, WorkspaceData } from "$lib/types/interfaces";
import {
  automaton as automatonStore, connection_start, default_connection_character, dialogue_to_user, input_alphabet,
  selection, should_show_string_traversal, should_strict_check, tool, type_of_automata, workspace_name,
} from "$lib/utils/automataStores";
import { saveWorkspace } from "$lib/utils/savingWorkspaceFuncs";
import {
  checkString as checkStringAction, clickCanvas, doubleClickCanvas, dragCanvas, loadWorkspace, pressCanvas, pressKey,
  releaseCanvas, selectTool as selectToolAction, trash as trashAction,
} from "$lib/workbench/actions";
import { connectionCurve, defaultControlPoints, emptyAutomaton, stateById, type Automaton } from "$lib/workbench/automaton";
import { backend } from "./fakeBackend";

export type Tool = "state" | "start" | "final" | "connection" | "epsilon" | "select";

export interface AutomatonDescription {
  states: Array<string>;
  transitions: Array<string>;
}

export interface WorkspaceOptions {
  type?: "DFA" | "NFA";
  alphabet?: Array<string>;
  defaultCharacter?: string;
  strictChecking?: boolean;
}

const parseState = (description: string) => {
  const [position, ...flags] = description.split(" ");
  const [x, y] = position.split(",").map(Number);
  return { x, y, key: position, isStart: flags.includes("start"), isFinal: flags.includes("final") };
};

const parseTransition = (description: string) => {
  const match = description.match(/^(\S+) -(.+)-> (\S+)$/);
  if (!match) {
    throw new Error(`Not a transition: ${description}`);
  }
  return { from: match[1], character: match[2], to: match[3] };
};

const describeState = (key: string, state: { is_start: boolean; is_final: boolean }): string =>
  [key, state.is_start ? "start" : "", state.is_final ? "final" : ""].filter(Boolean).join(" ");

// Reads the automaton out of states keyed by position, as the backend is sent them
export const describeStatePositions = (statePositions: Record<string, State>): AutomatonDescription => {
  const entries = Object.entries(statePositions);
  const states = entries.map(([key, state]) => describeState(key, state)).sort();
  const transitions = entries
    .flatMap(([key, state]) =>
      Object.entries(state.states_connected_to).flatMap(([character, targets]) =>
        targets.map((target) => `${key} -${character}-> ${target}`),
      ),
    )
    .sort();
  return { states, transitions };
};

const describePoint = (point: { x: number; y: number }): string => `${point.x},${point.y}`;

// Like starting the app: every store back to how it starts out
export const launchApp = (): void => {
  automatonStore.set(emptyAutomaton());
  tool.set("state");
  connection_start.set(null);
  selection.set(null);
  releaseCanvas();
  input_alphabet.set(["a", "b"]);
  type_of_automata.set(Automata.DFA);
  dialogue_to_user.set(null);
  should_strict_check.set(false);
  should_show_string_traversal.set(false);
  default_connection_character.set("a");
  workspace_name.set("Test workspace");
};

// Like opening a saved workspace: the workbench loads it the way the backend returns it
export const openWorkspace = (
  automaton: AutomatonDescription = { states: [], transitions: [] },
  options: WorkspaceOptions = {},
): void => {
  const states = automaton.states.map(parseState);
  const transitions = automaton.transitions.map(parseTransition);
  const position = (key: string) => {
    const state = states.find((state) => state.key === key);
    if (!state) {
      throw new Error(`No state at ${key}`);
    }
    return { x: state.x, y: state.y };
  };

  const backendStates = states.map((state) => {
    const connections: Record<string, Array<string>> = {};
    for (const transition of transitions.filter((transition) => transition.from === state.key)) {
      (connections[transition.character] ??= []).push(transition.to);
    }
    return {
      position: { x: state.x, y: state.y },
      states_connected_to: connections,
      is_start: state.isStart,
      is_final: state.isFinal,
      element: "State",
    };
  });
  const startIndex = states.findIndex((state) => state.isStart);

  const workspaceData = {
    start_state_index: startIndex >= 0 ? startIndex : null,
    start_state_position: startIndex >= 0 ? states[startIndex].key : null,
    state_positions: Object.fromEntries(states.map((state, index) => [state.key, backendStates[index]])),
    list_of_states: backendStates,
    // Laid out as the workbench lays out connections drawn in this order
    list_of_connections: transitions.map((transition, index) => {
      const from = position(transition.from);
      const to = position(transition.to);
      const alreadyJoining = transitions
        .slice(0, index)
        .filter((other) => (other.from === transition.from && other.to === transition.to) || (other.from === transition.to && other.to === transition.from))
        .length;
      const [control_point_one, control_point_two] = defaultControlPoints(from, to, transition.from === transition.to, alreadyJoining);
      return {
        curve: { start_point: from, control_point_one, control_point_two, end_point: to },
        connection_character: transition.character,
        element: "Connection",
      };
    }),
    type_of_automata: options.type ?? "DFA",
    date_of_last_update: "2026-10-20 12:00:00",
    alphabet: options.alphabet ?? ["a", "b"],
    should_strict_check: options.strictChecking ?? false,
    should_show_string_traversal: false,
    default_connection_character: options.defaultCharacter ?? "a",
  };

  // The data arrives as JSON, so nothing in it is shared between the lists
  loadWorkspace(JSON.parse(JSON.stringify(workspaceData)) as WorkspaceData);
};

// Like pressing one of the sidebar's tool buttons
export const selectTool = (newTool: Tool): void => {
  selectToolAction(newTool);
};

// Like clicking the canvas at the given position, in canvas pixels. The pointer is pressed and released
// first, as it is for a real click
export const click = (x: number, y: number): void => {
  pressCanvas({ x, y });
  releaseCanvas();
  clickCanvas({ x, y });
};

// Like double-clicking the canvas, which the browser delivers as two clicks and then a double click
export const doubleClick = (x: number, y: number): void => {
  click(x, y);
  click(x, y);
  doubleClickCanvas({ x, y });
};

// Like pressing the pointer at one position, moving it through the others and releasing it at the last
export const drag = (...points: Array<[number, number]>): void => {
  const [[x, y], ...rest] = points;
  pressCanvas({ x, y });
  for (const [nextX, nextY] of rest) {
    dragCanvas({ x: nextX, y: nextY });
  }
  releaseCanvas();
};

// Like pressing a key while the canvas has the focus
export const pressKeys = (key: string, modifiers: { ctrlKey?: boolean; metaKey?: boolean; altKey?: boolean } = {}): void => {
  pressKey({ key, ctrlKey: false, metaKey: false, altKey: false, ...modifiers });
};

// Like pressing the sidebar's trash button
export const trash = (): void => {
  trashAction();
};

const describeConnection = (current: Automaton, id: number): string | null => {
  const connection = current.connections.find((connection) => connection.id === id);
  if (!connection) {
    return null;
  }
  const curve = connectionCurve(current, connection);
  return `${describePoint(curve.start_point)} -${connection.character}-> ${describePoint(curve.end_point)}`;
};

// The selected state or connection, described like the others
export const selected = (): string | null => {
  const current = get(automatonStore);
  const picked = get(selection);
  if (!picked) {
    return null;
  }
  if (picked.kind === "state") {
    const state = stateById(current, picked.id);
    return state ? describePoint(state.position) : null;
  }
  return describeConnection(current, picked.id);
};

export const setAutomatonType = (type: "DFA" | "NFA"): void => {
  type_of_automata.set(type === "DFA" ? Automata.DFA : Automata.NFA);
};

export const setStrictChecking = (shouldStrictCheck: boolean): void => {
  should_strict_check.set(shouldStrictCheck);
};

// The message shown to the user, if any
export const message = (): string | null => get(dialogue_to_user) || null;

export const alphabet = (): Array<string> => get(input_alphabet);

// The automaton the workbench holds, which is what it checks strings against and saves
export const automaton = (): AutomatonDescription => {
  const current = get(automatonStore);
  return {
    states: current.states
      .map((state) => describeState(describePoint(state.position), { is_start: state.isStart, is_final: state.isFinal }))
      .sort(),
    transitions: current.connections.flatMap((connection) => describeConnection(current, connection.id) ?? []).sort(),
  };
};

// What the canvas draws: every state and every connection, in drawing order, duplicates included. A
// connection that has been started but not finished is drawn from its state to the pointer
export const drawing = (): { states: Array<string>; connections: Array<string> } => {
  const current = get(automatonStore);
  const from = get(connection_start);
  const unfinished = from === null ? undefined : stateById(current, from);
  return {
    states: current.states.map((state) =>
      describeState(describePoint(state.position), { is_start: state.isStart, is_final: state.isFinal }),
    ),
    connections: [
      ...current.connections.flatMap((connection) => describeConnection(current, connection.id) ?? []),
      ...(unfinished ? [`${describePoint(unfinished.position)} -> pointer`] : []),
    ],
  };
};

// Saves the way the save button does, and returns the automaton the backend was sent
export const save = async (): Promise<AutomatonDescription> => {
  await saveWorkspace();
  const [args] = backend.callsTo("save_workspace").slice(-1);
  return describeStatePositions(args.states);
};

export interface StringCheck {
  accepted: boolean | null;
  message: string | null;
  // The command the string was checked with, and the automaton it was sent
  command: string | null;
  sent: AutomatonDescription | null;
}

// Like typing a string into "String To Test" and pressing Check String
export const checkString = async (stringToCheck: string): Promise<StringCheck> => {
  const callsBefore = backend.calls.length;
  let accepted: boolean | null = null;

  // As the workbench page does it
  if (stringToCheck) {
    accepted = (await checkStringAction(stringToCheck)).is_string_accepted;
  }

  const testCall = backend.calls
    .slice(callsBefore)
    .find((call) => call.command === "test_string_dfa" || call.command === "test_string_nfa");
  return {
    accepted,
    message: message(),
    command: testCall?.command ?? null,
    sent: testCall ? describeStatePositions(testCall.args.statePositions) : null,
  };
};
