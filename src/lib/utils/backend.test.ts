import { get } from "svelte/store";
import { beforeEach, describe, expect, it } from "vitest";
import { backend } from "../../tests/fakeBackend";
import { dialogue_to_user } from "./automataStores";
import { describeFailure, invokeCommand, ReportedError } from "./backend";

beforeEach(() => {
  dialogue_to_user.set(null);
});

describe("invokeCommand", () => {
  it("returns what the command returns", async () => {
    backend.respond("get_saved_workspaces", () => ["First", "Second"]);
    expect(await invokeCommand("get_saved_workspaces")).toEqual(["First", "Second"]);
    expect(get(dialogue_to_user)).toBeNull();
  });

  it("shows why a command failed, and rejects so the caller stops", async () => {
    backend.respond("delete_workspace", () => {
      throw "There is no workspace called \"Gone\"";
    });
    await expect(invokeCommand("delete_workspace", { workspaceName: "Gone" })).rejects.toBeInstanceOf(ReportedError);
    expect(get(dialogue_to_user)).toBe("There is no workspace called \"Gone\"");
  });
});

describe("describeFailure", () => {
  it("uses the message, whatever form the failure takes", () => {
    expect(describeFailure("Plain message")).toBe("Plain message");
    expect(describeFailure(new Error("From an error"))).toBe("From an error");
    expect(describeFailure(42)).toBe("42");
  });
});
