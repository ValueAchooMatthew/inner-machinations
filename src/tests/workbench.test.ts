// How the workbench behaves, through the same actions a user takes. Tests marked it.fails describe the
// intended behaviour for a known bug: they pass while the bug is there, and fail once it's fixed, as a
// reminder to drop the marker.

import { beforeEach, describe, expect, it } from "vitest";
import { backend } from "./fakeBackend";
import {
  alphabet, automaton, automatonType, checkString, click, closeOptions, convert, defaultCharacter, doubleClick, drag, drawing,
  launchApp, message, minimize, openWorkspace, pressKeys, rename, save, selected, selectTool, setAutomatonType, setStrictChecking,
  trash, workspaceName,
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

  it("keeps one start state when another is placed in a reopened workspace", () => {
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

  it("draws each state once after connecting it", () => {
    connect([200, 200], [500, 200]);
    expect(drawing().states).toEqual(["200,200", "500,200"]);
  });

  it("leaves no unfinished arrow behind after switching tools mid-connection", () => {
    selectTool("connection");
    click(200, 200);
    selectTool("state");
    expect(drawing().connections).toEqual([]);
  });

  it("picks the connection nearest a double click with the select tool, changing nothing", () => {
    connect([200, 200], [500, 200]);
    selectTool("select");
    doubleClick(350, 210);
    expect(selected()).toBe("200,200 -a-> 500,200");
    expect(automaton()).toEqual({ states: ["200,200", "500,200"], transitions: ["200,200 -a-> 500,200"] });
  });

  it("doesn't fail when double-clicking a canvas with no connections", () => {
    selectTool("select");
    expect(() => doubleClick(700, 700)).not.toThrow();
  });
});

describe("the select tool", () => {
  const workspace = { states: ["200,200 start", "500,200 final"], transitions: ["200,200 -a-> 500,200", "500,200 -b-> 200,200"] };

  beforeEach(() => {
    openWorkspace(workspace);
    selectTool("select");
  });

  it("selects states and connections, and nothing when the click misses", () => {
    click(210, 190);
    expect(selected()).toBe("200,200");
    click(350, 205);
    expect(selected()).toBe("200,200 -a-> 500,200");
    click(800, 800);
    expect(selected()).toBeNull();
    expect(automaton()).toEqual(workspace);
  });

  it("moves a dragged state, with its connections following", () => {
    drag([500, 200], [520, 260], [541, 318]);
    expect(automaton()).toEqual({
      states: ["200,200 start", "540,320 final"],
      transitions: ["200,200 -a-> 540,320", "540,320 -b-> 200,200"],
    });
  });

  it("won't drop a dragged state on another", () => {
    drag([500, 200], [300, 200], [200, 200]);
    expect(automaton().states).toEqual(["200,200 start", "300,200 final"]);
  });

  it("relabels the selected connection with the next character typed", () => {
    click(350, 205);
    pressKeys("c");
    expect(automaton().transitions).toEqual(["200,200 -c-> 500,200", "500,200 -b-> 200,200"]);
    expect(alphabet()).toEqual(["a", "b", "c"]);
    expect(selected()).toBeNull();
  });

  it("doesn't relabel for shortcuts or keys that aren't characters", () => {
    click(350, 205);
    pressKeys("s", { ctrlKey: true });
    pressKeys("s", { metaKey: true });
    pressKeys("Shift");
    pressKeys("Enter");
    expect(automaton()).toEqual(workspace);
    expect(selected()).toBe("200,200 -a-> 500,200");
  });

  it("ignores typing when nothing, or a state, is selected", () => {
    pressKeys("c");
    click(200, 200);
    pressKeys("c");
    expect(automaton()).toEqual(workspace);
  });

  it("reshapes the selected connection by dragging its control points", () => {
    click(350, 205);
    drag([200, 200], [260, 120]);
    expect(drawing().connections).toContain("200,200 -a-> 500,200");
    // The press landed on the control point at the start state, which moved; the state didn't
    expect(automaton().states).toEqual(["200,200 start", "500,200 final"]);
    expect(selected()).toBe("200,200 -a-> 500,200");
  });

  it("forgets the selection when another tool is picked", () => {
    click(350, 205);
    selectTool("state");
    expect(selected()).toBeNull();
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

  it("starts each workspace on the state tool", () => {
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

  it("saves one start state after the start state of a reopened workspace changes", async () => {
    openWorkspace({ states: ["200,200 start"], transitions: [] });
    selectTool("start");
    click(500, 200);
    expect((await save()).states).toEqual(["200,200", "500,200 start"]);
  });
});

describe("minimizing and converting", () => {
  const nfa = { states: ["200,200 start", "500,200 final"], transitions: ["200,200 -a-> 500,200", "200,200 -ϵ-> 500,200"] };
  // What the backend returns, written as the workbench would draw it
  const dfa = {
    states: [
      { id: 1, position: { x: 200, y: 200 }, isStart: true, isFinal: true },
      { id: 2, position: { x: 500, y: 200 }, isStart: false, isFinal: true },
    ],
    connections: [
      { id: 1, from: 1, to: 2, character: "a", controlPoints: [{ x: 200, y: 200 }, { x: 500, y: 200 }] },
    ],
  };

  it("sends the automaton and alphabet, and shows the minimized DFA", async () => {
    openWorkspace(nfa, { alphabet: ["a"] });
    backend.respond("minimize_dfa", () => ({ automaton: dfa, wasAlreadyMinimal: false }));
    await minimize();
    expect(backend.callsTo("minimize_dfa")[0].inputAlphabet).toEqual(["a"]);
    expect(automaton()).toEqual({ states: ["200,200 start final", "500,200 final"], transitions: ["200,200 -a-> 500,200"] });
  });

  it("leaves a DFA that is already minimal as it was drawn, and says so", async () => {
    openWorkspace(nfa);
    backend.respond("minimize_dfa", () => ({ automaton: dfa, wasAlreadyMinimal: true }));
    await minimize();
    expect(automaton()).toEqual(nfa);
    expect(message()).toBe("This DFA is already minimal");
  });

  it("shows the converted DFA, then makes the workspace a DFA one", async () => {
    openWorkspace(nfa, { type: "NFA" });
    backend.respond("convert_nfa_to_dfa", () => dfa);
    await convert();
    expect(automaton().transitions).toEqual(["200,200 -a-> 500,200"]);
    expect(automatonType()).toBe("DFA");
    expect(backend.callsTo("update_automata_type")[0].typeOfAutomata).toBe("DFA");
  });

  it("leaves an NFA that couldn't be converted as it was", async () => {
    openWorkspace(nfa, { type: "NFA" });
    backend.respond("convert_nfa_to_dfa", () => {
      throw "The NFA needs a start state to be converted";
    });
    await convert();
    expect(automaton()).toEqual(nfa);
    expect(automatonType()).toBe("NFA");
    expect(backend.callsTo("update_automata_type")).toEqual([]);
    expect(message()).toBe("The NFA needs a start state to be converted");
  });

  it("doesn't save either result itself", async () => {
    openWorkspace(nfa, { type: "NFA" });
    backend.respond("minimize_dfa", () => ({ automaton: dfa, wasAlreadyMinimal: false }));
    backend.respond("convert_nfa_to_dfa", () => dfa);
    await minimize();
    await convert();
    expect(backend.callsTo("save_workspace")).toEqual([]);
  });
});

describe("renaming", () => {
  beforeEach(() => {
    backend.respond("does_workspace_name_exist", (args) => args.workspaceName === "Taken");
    backend.respond("save_workspace", (args) => args.alphabet);
  });

  it("saves, then renames, and remembers the new name for the session", async () => {
    await rename("Renamed", { submitted: true });
    expect(backend.calls.map((call) => call.command)).toEqual(["does_workspace_name_exist", "save_workspace", "update_workspace_name"]);
    expect(backend.callsTo("update_workspace_name")[0]).toEqual({ originalWorkspaceName: "Test workspace", newWorkspaceName: "Renamed" });
    expect(workspaceName()).toBe("Renamed");
    expect(sessionStorage.getItem("workspace_name")).toBe("Renamed");
  });

  it("won't take a blank name, and says so when the name is submitted", async () => {
    await rename("   ");
    expect(message()).toBeNull();
    await rename("", { submitted: true });
    expect(message()).toBe("A workspace needs a name");
    expect(backend.callsTo("update_workspace_name")).toEqual([]);
    expect(workspaceName()).toBe("Test workspace");
  });

  it("won't take another workspace's name, and says so when the name is submitted", async () => {
    await rename("Taken");
    expect(message()).toBeNull();
    await rename("Taken", { submitted: true });
    expect(message()).toBe("There is already a workspace called \"Taken\"");
    expect(backend.callsTo("update_workspace_name")).toEqual([]);
  });

  it("keeps the old name when renaming fails", async () => {
    backend.respond("update_workspace_name", () => {
      throw "Couldn't rename the workspace: disk I/O error";
    });
    await rename("Renamed", { submitted: true });
    expect(workspaceName()).toBe("Test workspace");
    expect(message()).toBe("Couldn't rename the workspace: disk I/O error");
  });
});

describe("the options menu", () => {
  beforeEach(() => {
    backend.respond("update_workspace_alphabet", (args) => args.alphabet.filter((character: string) => character.length === 1));
  });

  it("saves the alphabet and default character when it closes", async () => {
    await closeOptions({ alphabet: ["x", "", "y"], defaultCharacter: "y" });
    expect(alphabet()).toEqual(["x", "y"]);
    expect(defaultCharacter()).toBe("y");
    expect(backend.callsTo("update_default_connection_character")[0].defaultConnectionCharacter).toBe("y");
    expect(backend.callsTo("update_strict_checking")).toHaveLength(1);
    expect(backend.callsTo("update_showing_string_traversal")).toHaveLength(1);
  });

  it("keeps the default character when the new one isn't a single character", async () => {
    for (const notACharacter of ["", "ab", "ϵ"]) {
      await closeOptions({ alphabet: ["a", "b"], defaultCharacter: notACharacter });
      expect(defaultCharacter()).toBe("a");
      expect(message()).toBe("The default connection character must be a single character, other than ϵ");
    }
    expect(backend.callsTo("update_default_connection_character")).toEqual([]);
    // The rest is saved regardless
    expect(backend.callsTo("update_workspace_alphabet")).toHaveLength(3);
  });

  it("doesn't save the default character when it hasn't changed", async () => {
    await closeOptions({ alphabet: ["a", "b"], defaultCharacter: "a" });
    expect(backend.callsTo("update_default_connection_character")).toEqual([]);
  });
});

describe("checking strings", () => {
  const dfa = { states: ["200,200 start", "500,200 final"], transitions: ["200,200 -a-> 500,200"] };

  beforeEach(() => {
    backend.respond("test_string", () => [false, []]);
  });

  it("sends the automaton on the canvas and the string, and shows the answer", async () => {
    openWorkspace(dfa);
    backend.respond("test_string", () => [true, [1, 2]]);
    const check = await checkString("a");
    expect(check).toMatchObject({ accepted: true, command: "test_string", sent: dfa });
    expect(backend.callsTo("test_string")[0]).toMatchObject({ stringToCheck: "a" });
  });

  it("checks DFAs and NFAs the same way", async () => {
    openWorkspace(dfa, { type: "NFA" });
    expect((await checkString("b")).command).toBe("test_string");
    setAutomatonType("DFA");
    expect((await checkString("b")).command).toBe("test_string");
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
    backend.respond("test_string", () => [true, []]);
    expect(await checkString("a")).toMatchObject({ accepted: true, command: "test_string" });
  });

  it("doesn't strictly check NFAs", async () => {
    openWorkspace(dfa, { type: "NFA", strictChecking: true });
    await checkString("a");
    expect(backend.callsTo("verify_valid_dfa")).toEqual([]);
  });

  it("says when there is no start state", async () => {
    openWorkspace({ states: ["200,200 final"], transitions: [] });
    expect((await checkString("a")).message).toBe("You must specify at least one start state");
  });

  it.fails("checks the empty string", async () => {
    openWorkspace(dfa);
    expect(await checkString("")).toMatchObject({ accepted: false, command: "test_string" });
  });
});
