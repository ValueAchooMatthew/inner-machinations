// What the workbench does in response to the user: picking tools, clicking, dragging and typing on the canvas,
// and opening workspaces. The components pass events here, and draw whatever the stores then hold.

import { get } from "svelte/store";
import type { WorkspaceData } from "$lib/types/interfaces";
import { Automata } from "$lib/types/enums";
import {
  automaton, connection_start, default_connection_character, dialogue_to_user, input_alphabet, selection,
  should_show_string_traversal, should_strict_check, tool, type_of_automata, workspace_name, type Tool,
} from "$lib/utils/automataStores";
import { invokeCommand } from "$lib/utils/backend";
import { setSessionValue } from "$lib/utils/miscUtils";
import { saveNow } from "./autosave";
import { roundToNearest } from "$lib/utils/mathFuncs";
import { checkInputtedString, type CheckedStringResponse } from "$lib/utils/stringVerificationFuncs";
import {
  addConnection, addState, connectionById, connectionNear, emptyAutomaton, EPSILON, moveControlPoint, moveState,
  relabelConnection, setFinal, stateAt, stateIsAt, type Automaton, type Point,
} from "./automaton";

// New states go on a grid this many pixels apart, and dragged states on a finer one
const PLACING_GRID = 100;
const MOVING_GRID = 20;

// How close a press must be to a control point of the selected connection to drag it
const CONTROL_POINT_REACH = 12;

const snapTo = (point: Point, grid: number): Point => ({
  x: Math.max(0, roundToNearest(point.x, grid)),
  y: Math.max(0, roundToNearest(point.y, grid)),
});

const tell = (message: string): void => dialogue_to_user.set(message);

// What the pointer is dragging with the select tool, from press to release
type Drag = { kind: "state"; id: number } | { kind: "control point"; connection: number; index: 0 | 1 };
let drag: Drag | null = null;

const clearTransientState = (): void => {
  connection_start.set(null);
  selection.set(null);
  drag = null;
};

export function selectTool(newTool: Tool): void {
  clearTransientState();
  tool.set(newTool);
}

// Shows the workspace the backend returned, starting afresh: the state tool, nothing selected, no message
export function loadWorkspace(workspaceData: WorkspaceData): void {
  automaton.set(workspaceData.automaton);
  type_of_automata.set(workspaceData.type_of_automata === "DFA" ? Automata.DFA : Automata.NFA);
  input_alphabet.set(workspaceData.alphabet);
  should_strict_check.set(workspaceData.should_strict_check);
  should_show_string_traversal.set(workspaceData.should_show_string_traversal);
  default_connection_character.set(workspaceData.default_connection_character);
  dialogue_to_user.set(null);
  clearTransientState();
  tool.set("state");
}

// Replaces the automaton, as converting or minimizing does
export function replaceAutomaton(newAutomaton: Automaton): void {
  clearTransientState();
  automaton.set(newAutomaton);
}

export function trash(): void {
  clearTransientState();
  automaton.set(emptyAutomaton());
}

// Places a state where the canvas was clicked, unless one is there already. Returns whether it did
function placeState(point: Point, isStart: boolean): boolean {
  const current = get(automaton);
  const position = snapTo(point, PLACING_GRID);
  if (stateAt(current, point) || stateIsAt(current, position)) {
    tell("You cannot place a Node on top of another Node.");
    return false;
  }
  automaton.set(addState(current, position, isStart).automaton);
  return true;
}

export function clickCanvas(point: Point): void {
  dialogue_to_user.set(null);
  const current = get(automaton);
  const clickedState = stateAt(current, point);

  switch (get(tool)) {
    case "state":
      placeState(point, false);
      return;

    case "start":
      if (placeState(point, true)) {
        tool.set("state");
      }
      return;

    case "final":
      if (!clickedState) {
        tell("You must make an existing Node a final Node.");
      } else if (clickedState.isFinal) {
        tell("The Node is already a final Node.");
      } else {
        automaton.set(setFinal(current, clickedState.id, true));
      }
      return;

    case "connection":
    case "epsilon": {
      const from = get(connection_start);
      if (from === null) {
        if (!clickedState) {
          tell("You must place an arrow on top of a Node.");
          return;
        }
        connection_start.set(clickedState.id);
        return;
      }
      if (!clickedState) {
        tell("The arrow must point to a valid Node.");
        return;
      }
      const character = get(tool) === "epsilon" ? EPSILON : get(default_connection_character);
      automaton.set(addConnection(current, from, clickedState.id, character).automaton);
      connection_start.set(null);
      if (character !== EPSILON && !get(input_alphabet).includes(character)) {
        input_alphabet.update((alphabet) => [...alphabet, character]);
      }
      return;
    }

    case "select":
      // The select tool acts when the pointer is pressed, which comes before the click
      return;
  }
}

// With the select tool, picks the connection nearest the point, so it can be relabelled or reshaped
export function doubleClickCanvas(point: Point): void {
  if (get(tool) !== "select") {
    return;
  }
  const near = connectionNear(get(automaton), point);
  selection.set(near ? { kind: "connection", id: near.id } : null);
}

// With the select tool: pressing a control point of the selected connection starts reshaping it, pressing
// a state selects it and starts moving it, and pressing a connection selects it
export function pressCanvas(point: Point): void {
  if (get(tool) !== "select") {
    return;
  }
  const current = get(automaton);
  const selected = get(selection);

  if (selected?.kind === "connection") {
    const connection = connectionById(current, selected.id);
    const index = connection?.controlPoints.findIndex(
      (controlPoint) => Math.hypot(controlPoint.x - point.x, controlPoint.y - point.y) <= CONTROL_POINT_REACH,
    );
    if (connection && index !== undefined && index >= 0) {
      drag = { kind: "control point", connection: connection.id, index: index as 0 | 1 };
      return;
    }
  }

  const pressedState = stateAt(current, point);
  if (pressedState) {
    selection.set({ kind: "state", id: pressedState.id });
    drag = { kind: "state", id: pressedState.id };
    return;
  }

  const near = connectionNear(current, point);
  selection.set(near ? { kind: "connection", id: near.id } : null);
}

export function dragCanvas(point: Point): void {
  if (!drag) {
    return;
  }
  const current = get(automaton);
  if (drag.kind === "control point") {
    automaton.set(moveControlPoint(current, drag.connection, drag.index, point));
    return;
  }
  const position = snapTo(point, MOVING_GRID);
  const movingId = drag.id;
  const occupied = current.states.some(
    (state) => state.id !== movingId && state.position.x === position.x && state.position.y === position.y,
  );
  if (!occupied) {
    automaton.set(moveState(current, movingId, position));
  }
}

export function releaseCanvas(): void {
  drag = null;
}

// Typing a character while a connection is selected relabels it. Shortcuts such as Ctrl+S are left alone
export function pressKey(event: { key: string; ctrlKey: boolean; metaKey: boolean; altKey: boolean }): void {
  const selected = get(selection);
  if (selected?.kind !== "connection" || event.ctrlKey || event.metaKey || event.altKey || [...event.key].length !== 1) {
    return;
  }
  const character = event.key;
  automaton.set(relabelConnection(get(automaton), selected.id, character));
  selection.set(null);
  if (character !== EPSILON && !get(input_alphabet).includes(character)) {
    input_alphabet.update((alphabet) => [...alphabet, character]);
  }
}

// Checks the string against the automaton on the canvas, and shows any message that comes with the answer
export async function checkString(stringToCheck: string): Promise<CheckedStringResponse> {
  const result = await checkInputtedString(
    get(automaton),
    get(type_of_automata),
    stringToCheck,
    get(should_strict_check),
    get(input_alphabet),
  );
  dialogue_to_user.set(result.dialogue || null);
  return result;
}

// Replaces the DFA with the minimal one, unless it is minimal already, in which case it stays as the user drew it
export async function minimizeDfa(): Promise<void> {
  const minimized: { automaton: Automaton; wasAlreadyMinimal: boolean } = await invokeCommand("minimize_dfa", {
    automaton: get(automaton),
    inputAlphabet: get(input_alphabet),
  });
  if (minimized.wasAlreadyMinimal) {
    tell("This DFA is already minimal");
    return;
  }
  replaceAutomaton(minimized.automaton);
}

// Replaces the NFA with an equivalent DFA, and only then makes the workspace a DFA one
export async function convertNfaToDfa(): Promise<void> {
  const dfa: Automaton = await invokeCommand("convert_nfa_to_dfa", {
    automaton: get(automaton),
    inputAlphabet: get(input_alphabet),
  });
  replaceAutomaton(dfa);
  type_of_automata.set(Automata.DFA);
  await invokeCommand("update_automata_type", { workspaceName: get(workspace_name), typeOfAutomata: Automata[Automata.DFA] });
}

// Renames the workspace, saving it first so no save can go to the old name afterwards. A blank name, or one
// another workspace has, is left alone; `explain` says why, for when the user has asked to rename rather than
// merely paused typing
export async function renameWorkspace(newName: string, { explain = false } = {}): Promise<void> {
  const currentName = get(workspace_name);
  if (newName === currentName) {
    return;
  }
  if (!newName.trim()) {
    if (explain) {
      tell("A workspace needs a name");
    }
    return;
  }
  if (await invokeCommand<boolean>("does_workspace_name_exist", { workspaceName: newName })) {
    if (explain) {
      tell(`There is already a workspace called "${newName}"`);
    }
    return;
  }
  await saveNow();
  await invokeCommand("update_workspace_name", { originalWorkspaceName: currentName, newWorkspaceName: newName });
  workspace_name.set(newName);
  setSessionValue("workspace_name", newName);
}

// Saves the options menu's choices as it closes. Its checkboxes change their stores as they are clicked; the
// alphabet and default character are given here. A default character that isn't a single character is not taken
export async function saveOptions(alphabet: Array<string>, defaultCharacter: string): Promise<void> {
  const workspaceName = get(workspace_name);

  const savedAlphabet: Array<string> = await invokeCommand("update_workspace_alphabet", { workspaceName, alphabet });
  input_alphabet.set(savedAlphabet);
  await invokeCommand("update_strict_checking", { workspaceName, shouldStrictCheck: get(should_strict_check) });
  await invokeCommand("update_showing_string_traversal", { workspaceName, shouldShowTraversal: get(should_show_string_traversal) });

  if (defaultCharacter === get(default_connection_character)) {
    return;
  }
  if ([...defaultCharacter].length !== 1 || defaultCharacter === EPSILON) {
    tell("The default connection character must be a single character, other than ϵ");
    return;
  }
  await invokeCommand("update_default_connection_character", { workspaceName, defaultConnectionCharacter: defaultCharacter });
  default_connection_character.set(defaultCharacter);
}
