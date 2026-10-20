import { describe, expect, it } from "vitest";
import { addConnection, addState, emptyAutomaton, EPSILON, moveControlPoint, setFinal } from "./automaton";
import { fromBackend, startStateKey, toConnections, toStatePositions, type BackendConnection } from "./api";

const example = () => {
  const first = addState(emptyAutomaton(), { x: 200, y: 200 }, true);
  const second = addState(first.automaton, { x: 500, y: 200 });
  let automaton = setFinal(second.automaton, second.id, true);
  automaton = addConnection(automaton, first.id, second.id, "a").automaton;
  automaton = addConnection(automaton, second.id, first.id, EPSILON).automaton;
  automaton = addConnection(automaton, first.id, first.id, "b").automaton;
  return moveControlPoint(automaton, 1, 0, { x: 250, y: 100 });
};

describe("sending an automaton", () => {
  it("keys states by position and lists their connections", () => {
    expect(toStatePositions(example())).toEqual({
      "200,200": {
        position: { x: 200, y: 200 },
        states_connected_to: { a: ["500,200"], b: ["200,200"] },
        is_start: true,
        is_final: false,
        element: "State",
      },
      "500,200": {
        position: { x: 500, y: 200 },
        states_connected_to: { [EPSILON]: ["200,200"] },
        is_start: false,
        is_final: true,
        element: "State",
      },
    });
    expect(startStateKey(example())).toBe("200,200");
    expect(startStateKey(emptyAutomaton())).toBeNull();
  });

  it("sends each connection's curve", () => {
    expect(toConnections(example())[0]).toEqual({
      curve: {
        start_point: { x: 200, y: 200 },
        control_point_one: { x: 250, y: 100 },
        control_point_two: { x: 500, y: 200 },
        end_point: { x: 500, y: 200 },
      },
      connection_character: "a",
      element: "Connection",
    });
  });
});

describe("reading an automaton", () => {
  it("gets back what was sent", () => {
    const automaton = example();
    expect(fromBackend(toStatePositions(automaton), toConnections(automaton))).toEqual(automaton);
  });

  it("draws connections the backend has but didn't draw", () => {
    const automaton = example();
    const [, ...withoutFirst] = toConnections(automaton);
    const read = fromBackend(toStatePositions(automaton), withoutFirst);
    expect(read.connections.map((connection) => `${connection.from} ${connection.character} ${connection.to}`).sort())
      .toEqual(["1 a 2", "1 b 1", `2 ${EPSILON} 1`]);
    // Bent away from the epsilon connection already drawn between the same two states
    expect(read.connections.find((connection) => connection.character === "a")?.controlPoints)
      .toEqual([{ x: 300, y: 300 }, { x: 400, y: 300 }]);
  });

  it("leaves out connections that don't join two states, and duplicates", () => {
    const automaton = example();
    const stray: BackendConnection = {
      curve: {
        start_point: { x: 200, y: 200 },
        control_point_one: { x: 200, y: 200 },
        control_point_two: { x: 900, y: 900 },
        end_point: { x: 900, y: 900 },
      },
      connection_character: "a",
      element: "Connection",
    };
    const connections = [...toConnections(automaton), stray, toConnections(automaton)[0]];
    expect(fromBackend(toStatePositions(automaton), connections)).toEqual(automaton);
  });
});
