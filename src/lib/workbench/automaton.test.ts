import { describe, expect, it } from "vitest";
import {
  addConnection, addState, charactersUsed, connectionCurve, connectionNear, defaultControlPoints, emptyAutomaton, EPSILON,
  moveControlPoint, moveState, relabelConnection, removeConnection, removeState, setFinal, setStart, startState, stateAt,
  stateIsAt, type Automaton,
} from "./automaton";

// Two states 300 pixels apart, joined by "a"
const twoStates = (): { automaton: Automaton; left: number; right: number; connection: number } => {
  const first = addState(emptyAutomaton(), { x: 200, y: 200 }, true);
  const second = addState(first.automaton, { x: 500, y: 200 });
  const connected = addConnection(second.automaton, first.id, second.id, "a");
  return { automaton: connected.automaton, left: first.id, right: second.id, connection: connected.id };
};

describe("states", () => {
  it("get an id no other state has, even after a removal", () => {
    const { automaton, left } = twoStates();
    const removed = removeState(automaton, left);
    const added = addState(removed, { x: 800, y: 200 });
    expect(added.automaton.states.map((state) => state.id)).toEqual([2, 3]);
  });

  it("have one start state at most", () => {
    const { automaton, left, right } = twoStates();
    expect(startState(automaton)?.id).toBe(left);

    const added = addState(automaton, { x: 800, y: 200 }, true);
    expect(added.automaton.states.filter((state) => state.isStart).map((state) => state.id)).toEqual([added.id]);

    expect(startState(setStart(automaton, right))?.id).toBe(right);
    expect(startState(setStart(automaton, null))).toBeUndefined();
  });

  it("can be made final and not final", () => {
    const { automaton, right } = twoStates();
    const final = setFinal(automaton, right, true);
    expect(final.states.find((state) => state.id === right)?.isFinal).toBe(true);
    expect(setFinal(final, right, false).states.find((state) => state.id === right)?.isFinal).toBe(false);
  });

  it("are found by a click on or just around them", () => {
    const { automaton, left, right } = twoStates();
    expect(stateAt(automaton, { x: 200, y: 200 })?.id).toBe(left);
    expect(stateAt(automaton, { x: 240, y: 215 })?.id).toBe(left);
    expect(stateAt(automaton, { x: 470, y: 190 })?.id).toBe(right);
    expect(stateAt(automaton, { x: 350, y: 200 })).toBeUndefined();
    expect(stateIsAt(automaton, { x: 500, y: 200 })).toBe(true);
    expect(stateIsAt(automaton, { x: 510, y: 200 })).toBe(false);
  });

  it("take their connections with them when removed", () => {
    const { automaton, left } = twoStates();
    expect(removeState(automaton, left)).toEqual({ states: [automaton.states[1]], connections: [] });
  });

  it("carry the ends of their curves with them when moved", () => {
    const { automaton, right, connection } = twoStates();
    const bent = moveControlPoint(automaton, connection, 1, { x: 450, y: 100 });
    const moved = moveState(bent, right, { x: 500, y: 400 });
    expect(moved.states.find((state) => state.id === right)?.position).toEqual({ x: 500, y: 400 });
    // The control point beside the moved state moves with it; the other stays put
    expect(moved.connections[0].controlPoints).toEqual([{ x: 200, y: 200 }, { x: 450, y: 300 }]);
    expect(connectionCurve(moved, moved.connections[0]).end_point).toEqual({ x: 500, y: 400 });
  });

  it("leave everything unchanged when the operation is given a state that doesn't exist", () => {
    const { automaton } = twoStates();
    expect(moveState(automaton, 99, { x: 0, y: 0 })).toBe(automaton);
    expect(() => addConnection(automaton, 1, 99, "a")).toThrow();
  });
});

describe("connections", () => {
  it("are straight the first time two states are joined", () => {
    const { automaton } = twoStates();
    expect(automaton.connections[0]).toMatchObject({
      from: 1,
      to: 2,
      character: "a",
      controlPoints: [{ x: 200, y: 200 }, { x: 500, y: 200 }],
    });
  });

  it("bend apart when the same states are joined again, in either direction", () => {
    const { automaton, left, right } = twoStates();
    const second = addConnection(automaton, left, right, "b").automaton;
    const third = addConnection(second, right, left, "c").automaton;
    const [, bentOneWay, bentTheOther] = third.connections.map((connection) => connection.controlPoints);
    expect(bentOneWay).toEqual([{ x: 300, y: 300 }, { x: 400, y: 300 }]);
    expect(bentTheOther).toEqual([{ x: 400, y: 100 }, { x: 300, y: 100 }]);
  });

  it("loop to the left first, then up, down and right", () => {
    expect(defaultControlPoints({ x: 500, y: 500 }, { x: 500, y: 500 }, true, 0)).toEqual([{ x: 300, y: 700 }, { x: 300, y: 300 }]);
    expect(defaultControlPoints({ x: 500, y: 500 }, { x: 500, y: 500 }, true, 1)).toEqual([{ x: 300, y: 300 }, { x: 700, y: 300 }]);
    expect(defaultControlPoints({ x: 500, y: 500 }, { x: 500, y: 500 }, true, 4)).toEqual([{ x: 200, y: 800 }, { x: 200, y: 200 }]);
  });

  it("are found by a click near their curve", () => {
    const { automaton, connection } = twoStates();
    expect(connectionNear(automaton, { x: 350, y: 210 })?.id).toBe(connection);
    expect(connectionNear(automaton, { x: 350, y: 300 })).toBeUndefined();
    expect(connectionNear(emptyAutomaton(), { x: 0, y: 0 })).toBeUndefined();
  });

  it("can be relabelled and removed", () => {
    const { automaton, connection } = twoStates();
    expect(relabelConnection(automaton, connection, "z").connections[0].character).toBe("z");
    expect(removeConnection(automaton, connection).connections).toEqual([]);
  });

  it("report the characters they use, leaving out epsilon", () => {
    const { automaton, left, right } = twoStates();
    const withMore = addConnection(addConnection(automaton, right, left, EPSILON).automaton, left, left, "b").automaton;
    expect(charactersUsed(addConnection(withMore, right, right, "a").automaton)).toEqual(["a", "b"]);
  });
});

describe("operations", () => {
  it("never change the automaton they are given", () => {
    const { automaton, left, right, connection } = twoStates();
    const before = structuredClone(automaton);
    addState(automaton, { x: 900, y: 900 }, true);
    setStart(automaton, right);
    setFinal(automaton, left, true);
    moveState(automaton, left, { x: 0, y: 0 });
    removeState(automaton, left);
    addConnection(automaton, left, right, "b");
    relabelConnection(automaton, connection, "z");
    moveControlPoint(automaton, connection, 0, { x: 1, y: 1 });
    removeConnection(automaton, connection);
    expect(automaton).toEqual(before);
  });
});
