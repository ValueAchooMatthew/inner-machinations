// The automaton being edited on the workbench. States have ids that stay the same when they move, and
// connections refer to their states by id, so this one value is all there is to keep in step. Every
// operation returns a new automaton and leaves the one it was given unchanged, so earlier versions can
// be kept for undo.

import type { BezierCurve } from "$lib/types/interfaces";
import { distanceFromBezierCurveToPoint } from "$lib/utils/mathFuncs";

export const EPSILON = "ϵ";

// States are drawn as circles of this radius, with a ring at 42 pixels around final states
export const STATE_RADIUS = 35;

export interface Point {
  x: number;
  y: number;
}

export interface AutomatonState {
  id: number;
  position: Point;
  isStart: boolean;
  isFinal: boolean;
}

export interface AutomatonConnection {
  id: number;
  from: number;
  to: number;
  // A single character, or EPSILON
  character: string;
  // The curve's two control points; its ends are the centres of the states it joins
  controlPoints: [Point, Point];
}

export interface Automaton {
  states: Array<AutomatonState>;
  connections: Array<AutomatonConnection>;
}

export const emptyAutomaton = (): Automaton => ({ states: [], connections: [] });

const nextId = (items: Array<{ id: number }>): number => items.reduce((largest, item) => Math.max(largest, item.id), 0) + 1;

const distance = (first: Point, second: Point): number => Math.hypot(first.x - second.x, first.y - second.y);

export const stateById = (automaton: Automaton, id: number): AutomatonState | undefined =>
  automaton.states.find((state) => state.id === id);

export const connectionById = (automaton: Automaton, id: number): AutomatonConnection | undefined =>
  automaton.connections.find((connection) => connection.id === id);

export const startState = (automaton: Automaton): AutomatonState | undefined =>
  automaton.states.find((state) => state.isStart);

// The state drawn at the point, if any. A click just outside a state's circle still counts, since final
// states are drawn with a ring around them
export const stateAt = (automaton: Automaton, point: Point): AutomatonState | undefined => {
  let closest: AutomatonState | undefined;
  for (const state of automaton.states) {
    const stateDistance = distance(state.position, point);
    if (stateDistance <= STATE_RADIUS + 10 && (!closest || stateDistance < distance(closest.position, point))) {
      closest = state;
    }
  }
  return closest;
};

export const stateIsAt = (automaton: Automaton, position: Point): boolean =>
  automaton.states.some((state) => state.position.x === position.x && state.position.y === position.y);

export function addState(automaton: Automaton, position: Point, isStart = false): { automaton: Automaton; id: number } {
  const id = nextId(automaton.states);
  const states = isStart ? automaton.states.map((state) => ({ ...state, isStart: false })) : automaton.states;
  return {
    automaton: { ...automaton, states: [...states, { id, position, isStart, isFinal: false }] },
    id,
  };
}

// Makes the state the start state, and no other. Null leaves the automaton without a start state
export const setStart = (automaton: Automaton, id: number | null): Automaton => ({
  ...automaton,
  states: automaton.states.map((state) => ({ ...state, isStart: state.id === id })),
});

export const setFinal = (automaton: Automaton, id: number, isFinal: boolean): Automaton => ({
  ...automaton,
  states: automaton.states.map((state) => (state.id === id ? { ...state, isFinal } : state)),
});

// Moves the state, carrying the ends of the curves attached to it along
export function moveState(automaton: Automaton, id: number, position: Point): Automaton {
  const state = stateById(automaton, id);
  if (!state) {
    return automaton;
  }
  const shift = (point: Point): Point => ({
    x: point.x + position.x - state.position.x,
    y: point.y + position.y - state.position.y,
  });
  return {
    states: automaton.states.map((other) => (other.id === id ? { ...other, position } : other)),
    connections: automaton.connections.map((connection) => {
      const [first, second] = connection.controlPoints;
      if (connection.from !== id && connection.to !== id) {
        return connection;
      }
      return {
        ...connection,
        controlPoints: [connection.from === id ? shift(first) : first, connection.to === id ? shift(second) : second],
      };
    }),
  };
}

// Removes the state and every connection to or from it
export const removeState = (automaton: Automaton, id: number): Automaton => ({
  states: automaton.states.filter((state) => state.id !== id),
  connections: automaton.connections.filter((connection) => connection.from !== id && connection.to !== id),
});

export function addConnection(
  automaton: Automaton,
  from: number,
  to: number,
  character: string,
): { automaton: Automaton; id: number } {
  const fromState = stateById(automaton, from);
  const toState = stateById(automaton, to);
  if (!fromState || !toState) {
    throw new Error(`There is no state ${fromState ? to : from}`);
  }
  const id = nextId(automaton.connections);
  const alreadyJoining = automaton.connections.filter(
    (connection) => (connection.from === from && connection.to === to) || (connection.from === to && connection.to === from),
  ).length;
  const connection: AutomatonConnection = {
    id,
    from,
    to,
    character,
    controlPoints: defaultControlPoints(fromState.position, toState.position, from === to, alreadyJoining),
  };
  return { automaton: { ...automaton, connections: [...automaton.connections, connection] }, id };
}

export const relabelConnection = (automaton: Automaton, id: number, character: string): Automaton => ({
  ...automaton,
  connections: automaton.connections.map((connection) => (connection.id === id ? { ...connection, character } : connection)),
});

export const removeConnection = (automaton: Automaton, id: number): Automaton => ({
  ...automaton,
  connections: automaton.connections.filter((connection) => connection.id !== id),
});

export const moveControlPoint = (automaton: Automaton, id: number, index: 0 | 1, point: Point): Automaton => ({
  ...automaton,
  connections: automaton.connections.map((connection) => {
    if (connection.id !== id) {
      return connection;
    }
    const controlPoints: [Point, Point] = [...connection.controlPoints];
    controlPoints[index] = point;
    return { ...connection, controlPoints };
  }),
});

export function connectionCurve(automaton: Automaton, connection: AutomatonConnection): BezierCurve {
  const from = stateById(automaton, connection.from);
  const to = stateById(automaton, connection.to);
  if (!from || !to) {
    throw new Error(`Connection ${connection.id} joins a state that doesn't exist`);
  }
  return {
    start_point: from.position,
    control_point_one: connection.controlPoints[0],
    control_point_two: connection.controlPoints[1],
    end_point: to.position,
  };
}

// The connection whose curve passes nearest the point, if one passes within reach of it. Of curves that pass
// equally near, the one drawn first
export function connectionNear(automaton: Automaton, point: Point, reach = 30): AutomatonConnection | undefined {
  let closest: AutomatonConnection | undefined;
  let closestDistance = reach;
  for (const connection of automaton.connections) {
    const connectionDistance = distanceFromBezierCurveToPoint(point, connectionCurve(automaton, connection), 50);
    if (connectionDistance < closestDistance || (!closest && connectionDistance === closestDistance)) {
      closest = connection;
      closestDistance = connectionDistance;
    }
  }
  return closest;
}

// The characters connections use, apart from epsilon, in the order they first appear
export const charactersUsed = (automaton: Automaton): Array<string> => [
  ...new Set(automaton.connections.map((connection) => connection.character).filter((character) => character !== EPSILON)),
];

// The first connection between two states is straight, and each one after it bends further away, alternating
// sides, so their characters are drawn apart. A state's first loop to itself goes to the left, and later ones
// go up, down and right, getting bigger after that. The backend lays out the connections it creates the same way
export function defaultControlPoints(from: Point, to: Point, isLoop: boolean, index: number): [Point, Point] {
  if (isLoop) {
    const size = 200 + 100 * Math.floor(index / 4);
    const offsets: Array<[[number, number], [number, number]]> = [
      [[-size, size], [-size, -size]],
      [[-size, -size], [size, -size]],
      [[size, size], [-size, size]],
      [[size, -size], [size, size]],
    ];
    const [[firstX, firstY], [secondX, secondY]] = offsets[index % 4];
    return [
      { x: from.x + firstX, y: from.y + firstY },
      { x: to.x + secondX, y: to.y + secondY },
    ];
  }

  // The bend is measured from the same side whichever way the connection goes
  const [firstPoint, secondPoint] = from.x < to.x || (from.x === to.x && from.y <= to.y) ? [from, to] : [to, from];
  const xDistance = secondPoint.x - firstPoint.x;
  const yDistance = secondPoint.y - firstPoint.y;
  const length = Math.hypot(xDistance, yDistance);
  if (index === 0 || length === 0) {
    return [{ ...from }, { ...to }];
  }

  // 100 to one side, then 100 to the other, then 200 to the first side, and so on
  const bend = 100 * Math.floor((index + 1) / 2) * (index % 2 === 1 ? 1 : -1);
  const pointAlongCurve = (fraction: number): Point => ({
    x: Math.round(from.x + (to.x - from.x) * fraction - (yDistance / length) * bend),
    y: Math.round(from.y + (to.y - from.y) * fraction + (xDistance / length) * bend),
  });
  return [pointAlongCurve(1 / 3), pointAlongCurve(2 / 3)];
}
