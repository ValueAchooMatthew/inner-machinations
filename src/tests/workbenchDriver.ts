// Drives the workbench the way its components do, so workbench.test.ts can describe behaviour in terms
// of tools, clicks and what ends up on the canvas, rather than in terms of stores. When the workbench's
// internals change, this file follows them and the tests stay as they are.
//
// Automata are written as readable strings:
//   states:      "200,200", "200,200 start", "500,200 final", "500,200 start final"
//   transitions: "200,200 -a-> 500,200", with ϵ for epsilon

import { get } from "svelte/store";
import { Action, Automata } from "$lib/types/enums";
import type { Connection, State, WorkspaceData } from "$lib/types/interfaces";
import {
  current_action, default_connection_character, dialogue_to_user, input_alphabet, list_of_all_elements,
  list_of_connections, list_of_states, selected_connection_index, should_show_string_traversal,
  should_strict_check, start_state_index, start_state_position, state_positions, type_of_automata,
} from "$lib/utils/automataStores";
import { handleUserClickingCanvas } from "$lib/utils/userEvents";
import { indexOfClosestBezierCurveToPoint, roundToNearest } from "$lib/utils/mathFuncs";
import { setTauriResponses } from "$lib/utils/parsingBackendResponsesFuncs";
import { saveWorkspace } from "$lib/utils/savingWorkspaceFuncs";
import { checkInputtedString } from "$lib/utils/stringVerificationFuncs";
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

const TOOLS: Record<Tool, Action> = {
  state: Action.ADDING_REGULAR_STATE,
  start: Action.ADDING_START_STATE,
  final: Action.ADDING_FINAL_STATE,
  connection: Action.PLACING_START_OF_LINE,
  epsilon: Action.PLACING_START_OF_EPSILON_LINE,
  select: Action.CLICKING,
};

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

// Reads the automaton out of states keyed by position, as the frontend holds them and as it sends them
// to the backend. Characters left with no connections are ignored
export const describeStatePositions = (
  statePositions: Map<string, State> | Record<string, any>,
): AutomatonDescription => {
  const entries: Array<[string, any]> =
    statePositions instanceof Map ? [...statePositions.entries()] : Object.entries(statePositions);
  const states = entries.map(([key, state]) => describeState(key, state)).sort();
  const transitions = entries
    .flatMap(([key, state]) => {
      const connections: Array<[string, Array<string>]> =
        state.states_connected_to instanceof Map
          ? [...state.states_connected_to.entries()]
          : Object.entries(state.states_connected_to);
      return connections.flatMap(([character, targets]) => targets.map((target) => `${key} -${character}-> ${target}`));
    })
    .sort();
  return { states, transitions };
};

const straightOrLoopCurve = (from: { x: number; y: number }, to: { x: number; y: number }) => {
  const isLoop = from.x === to.x && from.y === to.y;
  return {
    start_point: from,
    control_point_one: isLoop ? { x: from.x - 200, y: from.y + 200 } : from,
    control_point_two: isLoop ? { x: to.x - 200, y: to.y - 200 } : to,
    end_point: to,
  };
};

// Like starting the app: every store back to how it starts out
export const launchApp = (): void => {
  state_positions.set(new Map());
  list_of_states.set([]);
  list_of_connections.set([]);
  list_of_all_elements.set([]);
  start_state_index.set(null);
  start_state_position.set(null);
  input_alphabet.set(["a", "b"]);
  type_of_automata.set(Automata.DFA);
  dialogue_to_user.set(null);
  current_action.set(Action.ADDING_REGULAR_STATE);
  selected_connection_index.set(null);
  should_strict_check.set(false);
  should_show_string_traversal.set(false);
  default_connection_character.set("a");
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
    list_of_connections: transitions.map((transition) => ({
      curve: straightOrLoopCurve(position(transition.from), position(transition.to)),
      connection_character: transition.character,
      element: "Connection",
    })),
    type_of_automata: options.type ?? "DFA",
    date_of_last_update: "2026-10-20 12:00:00",
    alphabet: options.alphabet ?? ["a", "b"],
    should_strict_check: options.strictChecking ?? false,
    should_show_string_traversal: false,
    default_connection_character: options.defaultCharacter ?? "a",
  };

  // The data arrives as JSON, so nothing in it is shared between the lists
  setTauriResponses(JSON.parse(JSON.stringify(workspaceData)) as WorkspaceData);
};

// Like pressing one of the sidebar's tool buttons
export const selectTool = (tool: Tool): void => {
  current_action.set(Action.CLICKING);
  selected_connection_index.set(null);
  current_action.set(TOOLS[tool]);
};

// Like clicking the canvas at the given position, in canvas pixels
export const click = (x: number, y: number): void => {
  handleUserClickingCanvas(roundToNearest(x, 100), roundToNearest(y, 100), get(default_connection_character));
};

// Like double-clicking the canvas, which the browser delivers as two clicks and then a double click
export const doubleClick = (x: number, y: number): void => {
  click(x, y);
  click(x, y);
  current_action.set(Action.CLICKING);
  selected_connection_index.set(null);
  selected_connection_index.set(indexOfClosestBezierCurveToPoint({ x, y }));
};

// Like pressing the sidebar's trash button
export const trash = (): void => {
  current_action.set(Action.CLICKING);
  selected_connection_index.set(null);
  state_positions.set(new Map());
  start_state_index.set(null);
  start_state_position.set(null);
  list_of_states.set([]);
  list_of_connections.set([]);
  list_of_all_elements.set([]);
  current_action.set(Action.CLICKING);
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
export const automaton = (): AutomatonDescription => describeStatePositions(get(state_positions));

// What the canvas draws: every state and every connection, in drawing order, duplicates included
export const drawing = (): { states: Array<string>; connections: Array<string> } => ({
  states: get(list_of_states).map((state) => describeState(`${state.position.x},${state.position.y}`, state)),
  connections: get(list_of_connections).map(
    (connection: Connection) =>
      `${connection.curve.start_point.x},${connection.curve.start_point.y} -${connection.connection_character}-> ` +
      `${connection.curve.end_point.x},${connection.curve.end_point.y}`,
  ),
});

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
    if (start_state_index === null) {
      dialogue_to_user.set("You must specify at least one start state");
    } else {
      const result = await checkInputtedString(
        get(start_state_position),
        get(type_of_automata),
        get(state_positions),
        stringToCheck,
        get(should_strict_check),
        get(input_alphabet),
      );
      accepted = result.is_string_accepted;
      dialogue_to_user.set(result.dialogue);
    }
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
