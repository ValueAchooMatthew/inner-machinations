// How the workbench behaves, through the same actions a user takes. Tests marked it.fails describe the
// intended behaviour for a known bug: they pass while the bug is there, and fail once it's fixed, as a
// reminder to drop the marker.

import { beforeEach, describe, expect, it } from "vitest";
import { backend } from "./fakeBackend";
import {
  alphabet, automaton, checkString, click, doubleClick, drawing, launchApp, message, openWorkspace, save,
  selectTool, setAutomatonType, setStrictChecking, trash,
} from "./workbenchDriver";

beforeEach(() => {
  launchApp();
  openWorkspace();
});

const placeStates = (...positions: Array<[number, number]>) => {
  selectTool("state");
  for (const [x, y] of positions) {
    click(x, y);
  }
};

const connect = (from: [number, number], to: [number, number], tool: "connection" | "epsilon" = "connection") => {
  selectTool(tool);
  click(...from);
  click(...to);
};

describe("placing states", () => {
  it("places a state where the canvas is clicked, snapped to a 100 pixel grid", () => {
    placeStates([213, 187]);
    expect(automaton().states).toEqual(["200,200"]);
  });

  it("won't place a state on top of another", () => {
    placeStates([200, 200], [230, 170]);
    expect(automaton().states).toEqual(["200,200"]);
    expect(message()).toBe("You cannot place a Node on top of another Node.");
  });

  it("places a start state, then goes back to placing ordinary states", () => {
    selectTool("start");
    click(200, 200);
    click(500, 200);
    expect(automaton().states).toEqual(["200,200 start", "500,200"]);
  });

  it("keeps one start state when another is placed", () => {
    selectTool("start");
    click(200, 200);
    selectTool("start");
    click(500, 200);
    expect(automaton().states).toEqual(["200,200", "500,200 start"]);
  });

  it.fails("keeps one start state when another is placed in a reopened workspace", () => {
    openWorkspace({ states: ["200,200 start"], transitions: [] });
    selectTool("start");
    click(500, 200);
    expect(automaton().states).toEqual(["200,200", "500,200 start"]);
  });

  it("makes an existing state final", () => {
    placeStates([200, 200]);
    selectTool("final");
    click(200, 200);
    expect(automaton().states).toEqual(["200,200 final"]);
  });

  it("asks for an existing state when making one final", () => {
    selectTool("final");
    click(200, 200);
    expect(automaton().states).toEqual([]);
    expect(message()).toBe("You must make an existing Node a final Node.");
  });
});

describe("connections", () => {
  beforeEach(() => {
    placeStates([200, 200], [500, 200]);
  });

  it("connects two states with the default character", () => {
    connect([200, 200], [500, 200]);
    expect(automaton().transitions).toEqual(["200,200 -a-> 500,200"]);
  });

  it("stays on the connection tool, so connections can be chained", () => {
    connect([200, 200], [500, 200]);
    click(500, 200);
    click(200, 200);
    expect(automaton().transitions).toEqual(["200,200 -a-> 500,200", "500,200 -a-> 200,200"]);
  });

  it("connects a state to itself", () => {
    connect([200, 200], [200, 200]);
    expect(automaton().transitions).toEqual(["200,200 -a-> 200,200"]);
  });

  it("makes epsilon connections", () => {
    connect([200, 200], [500, 200], "epsilon");
    expect(automaton().transitions).toEqual(["200,200 -ϵ-> 500,200"]);
  });

  it("asks for a state to start a connection from", () => {
    selectTool("connection");
    click(800, 800);
    expect(message()).toBe("You must place an arrow on top of a Node.");
    click(500, 200);
    expect(automaton().transitions).toEqual([]);
  });

  it("asks for a state to end a connection on, and lets the user try again", () => {
    selectTool("connection");
    click(200, 200);
    click(800, 800);
    expect(message()).toBe("The arrow must point to a valid Node.");
    click(500, 200);
    expect(automaton().transitions).toEqual(["200,200 -a-> 500,200"]);
  });

  it("adds the default character to the alphabet when it isn't there", () => {
    openWorkspace({ states: ["200,200", "500,200"], transitions: [] }, { alphabet: ["b"], defaultCharacter: "c" });
    connect([200, 200], [500, 200]);
    expect(automaton().transitions).toEqual(["200,200 -c-> 500,200"]);
    expect(alphabet()).toEqual(["b", "c"]);
  });

  it.fails("draws each state once after connecting it", () => {
    connect([200, 200], [500, 200]);
    expect(drawing().states).toEqual(["200,200", "500,200"]);
  });

  it.fails("leaves no unfinished arrow behind after switching tools mid-connection", () => {
    selectTool("connection");
    click(200, 200);
    selectTool("state");
    expect(drawing().connections).toEqual([]);
  });

  it.fails("doesn't add a loop when double-clicking a state with the connection tool", () => {
    selectTool("connection");
    doubleClick(200, 200);
    expect(automaton().transitions).toEqual([]);
  });

  it.fails("doesn't fail when double-clicking a canvas with no connections", () => {
    selectTool("select");
    expect(() => doubleClick(700, 700)).not.toThrow();
  });
});

describe("opening workspaces", () => {
  it("shows the automaton that was saved", () => {
    const saved = { states: ["200,200 start", "500,200 final"], transitions: ["200,200 -a-> 500,200"] };
    openWorkspace(saved);
    expect(automaton()).toEqual(saved);
    expect(drawing()).toEqual({ states: ["200,200 start", "500,200 final"], connections: ["200,200 -a-> 500,200"] });
  });

  it("leaves no unfinished connection from the previous workspace", () => {
    placeStates([200, 200]);
    selectTool("connection");
    click(200, 200);
    openWorkspace({ states: ["300,300"], transitions: [] });
    expect(drawing()).toEqual({ states: ["300,300"], connections: [] });
  });

  it.fails("starts each workspace on the state tool", () => {
    placeStates([200, 200]);
    selectTool("connection");
    click(200, 200);
    openWorkspace({ states: ["300,300"], transitions: [] });
    click(600, 600);
    expect(automaton().states).toEqual(["300,300", "600,600"]);
  });
});

describe("trash", () => {
  it("clears the canvas", () => {
    openWorkspace({ states: ["200,200 start", "500,200 final"], transitions: ["200,200 -a-> 500,200"] });
    trash();
    expect(automaton()).toEqual({ states: [], transitions: [] });
    expect(drawing()).toEqual({ states: [], connections: [] });
  });
});

describe("saving", () => {
  it("sends the automaton on the canvas", async () => {
    selectTool("start");
    click(200, 200);
    placeStates([500, 200]);
    selectTool("final");
    click(500, 200);
    connect([200, 200], [500, 200]);
    expect(await save()).toEqual({ states: ["200,200 start", "500,200 final"], transitions: ["200,200 -a-> 500,200"] });
  });

  it("saves a reopened workspace unchanged", async () => {
    const saved = { states: ["200,200 start", "500,200 final"], transitions: ["200,200 -a-> 500,200", "500,200 -b-> 200,200"] };
    openWorkspace(saved);
    expect(await save()).toEqual(saved);
  });

  it("shows why saving failed", async () => {
    backend.respond("save_workspace", () => {
      throw "Couldn't save the workspace: disk I/O error";
    });
    await expect(save()).rejects.toThrow();
    expect(message()).toBe("Couldn't save the workspace: disk I/O error");
  });

  it.fails("saves one start state after the start state of a reopened workspace changes", async () => {
    openWorkspace({ states: ["200,200 start"], transitions: [] });
    selectTool("start");
    click(500, 200);
    expect((await save()).states).toEqual(["200,200", "500,200 start"]);
  });
});

describe("checking strings", () => {
  const dfa = { states: ["200,200 start", "500,200 final"], transitions: ["200,200 -a-> 500,200"] };

  beforeEach(() => {
    backend.respond("test_string_dfa", () => [false, []]);
    backend.respond("test_string_nfa", () => [false, []]);
  });

  it("checks a DFA with test_string_dfa, sending it the automaton on the canvas", async () => {
    openWorkspace(dfa);
    backend.respond("test_string_dfa", () => [true, []]);
    const check = await checkString("a");
    expect(check).toMatchObject({ accepted: true, command: "test_string_dfa", sent: dfa });
    expect(backend.callsTo("test_string_dfa")[0]).toMatchObject({ startStateCoordinates: "200,200", stringToCheck: "a" });
  });

  it("checks an NFA with test_string_nfa", async () => {
    openWorkspace(dfa, { type: "NFA" });
    backend.respond("test_string_nfa", () => [false, []]);
    expect(await checkString("b")).toMatchObject({ accepted: false, command: "test_string_nfa" });
  });

  it("follows a change between DFA and NFA", async () => {
    openWorkspace(dfa);
    setAutomatonType("NFA");
    expect((await checkString("a")).command).toBe("test_string_nfa");
  });

  it("reports an incomplete DFA when strict checking is on, without checking the string", async () => {
    openWorkspace(dfa, { strictChecking: true });
    backend.respond("verify_valid_dfa", () => false);
    const check = await checkString("a");
    expect(check.accepted).toBeNull();
    expect(check.command).toBeNull();
    expect(check.message).toMatch(/does not specify every connection/);
  });

  it("checks the string when strict checking finds the DFA complete", async () => {
    openWorkspace(dfa);
    setStrictChecking(true);
    backend.respond("verify_valid_dfa", () => true);
    backend.respond("test_string_dfa", () => [true, []]);
    expect(await checkString("a")).toMatchObject({ accepted: true, command: "test_string_dfa" });
  });

  it("doesn't strictly check NFAs", async () => {
    openWorkspace(dfa, { type: "NFA", strictChecking: true });
    await checkString("a");
    expect(backend.callsTo("verify_valid_dfa")).toEqual([]);
  });

  it.fails("says when there is no start state", async () => {
    openWorkspace({ states: ["200,200 final"], transitions: [] });
    expect((await checkString("a")).message).toBe("You must specify at least one start state");
  });

  it.fails("checks the empty string", async () => {
    openWorkspace(dfa);
    backend.respond("test_string_dfa", () => [false, []]);
    expect(await checkString("")).toMatchObject({ accepted: false, command: "test_string_dfa" });
  });
});
