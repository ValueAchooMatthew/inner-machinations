// Converts between the workbench's automaton and the form the backend commands take and return, in which
// states are keyed by their position as "x,y" and list the states they connect to by character

import type { Connection, State, WorkspaceData } from "$lib/types/interfaces";
import {
  addConnection, addState, connectionCurve, emptyAutomaton, moveControlPoint, setFinal, startState, type Automaton,
  type Point,
} from "./automaton";

// What the commands call states and connections
export type BackendState = State;
export type BackendConnection = Connection;

export const positionKey = (point: Point): string => `${point.x},${point.y}`;

export function toStatePositions(automaton: Automaton): Record<string, BackendState> {
  const statePositions: Record<string, BackendState> = {};
  for (const state of automaton.states) {
    statePositions[positionKey(state.position)] = {
      position: state.position,
      states_connected_to: {},
      is_start: state.isStart,
      is_final: state.isFinal,
      element: "State",
    };
  }
  for (const connection of automaton.connections) {
    const curve = connectionCurve(automaton, connection);
    const connected = statePositions[positionKey(curve.start_point)].states_connected_to;
    (connected[connection.character] ??= []).push(positionKey(curve.end_point));
  }
  return statePositions;
}

export const toConnections = (automaton: Automaton): Array<BackendConnection> =>
  automaton.connections.map((connection) => ({
    curve: connectionCurve(automaton, connection),
    connection_character: connection.character,
    element: "Connection",
  }));

export const startStateKey = (automaton: Automaton): string | null => {
  const start = startState(automaton);
  return start ? positionKey(start.position) : null;
};

// Reads an automaton from the backend. The drawn connections keep their curves; any connection between
// states that isn't drawn gets a curve of its own, and drawn connections that don't start and end on a
// state are left out
export function fromBackend(
  statePositions: Record<string, BackendState>,
  connections: Array<BackendConnection>,
): Automaton {
  let automaton = emptyAutomaton();
  const idOfKey = new Map<string, number>();

  for (const [key, state] of Object.entries(statePositions)) {
    const added = addState(automaton, state.position, state.is_start);
    automaton = state.is_final ? setFinal(added.automaton, added.id, true) : added.automaton;
    idOfKey.set(key, added.id);
  }

  const isDrawn = new Set<string>();
  for (const connection of connections) {
    const from = idOfKey.get(positionKey(connection.curve.start_point));
    const to = idOfKey.get(positionKey(connection.curve.end_point));
    const transition = `${from} ${connection.connection_character} ${to}`;
    if (from === undefined || to === undefined || isDrawn.has(transition)) {
      continue;
    }
    isDrawn.add(transition);
    const added = addConnection(automaton, from, to, connection.connection_character);
    automaton = moveControlPoint(added.automaton, added.id, 0, connection.curve.control_point_one);
    automaton = moveControlPoint(automaton, added.id, 1, connection.curve.control_point_two);
  }

  for (const [key, state] of Object.entries(statePositions)) {
    for (const [character, targets] of Object.entries(state.states_connected_to)) {
      for (const target of targets) {
        const from = idOfKey.get(key);
        const to = idOfKey.get(target);
        if (from !== undefined && to !== undefined && !isDrawn.has(`${from} ${character} ${to}`)) {
          isDrawn.add(`${from} ${character} ${to}`);
          automaton = addConnection(automaton, from, to, character).automaton;
        }
      }
    }
  }

  return automaton;
}

export const fromWorkspaceData = (workspaceData: WorkspaceData): Automaton =>
  fromBackend(workspaceData.state_positions, workspaceData.list_of_connections);
