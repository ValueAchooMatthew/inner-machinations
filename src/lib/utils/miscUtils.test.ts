import { describe, expect, it } from "vitest";
import { firstFreeName } from "./miscUtils";

describe("firstFreeName", () => {
  it("uses the name as it is when it's free", () => {
    expect(firstFreeName("Untitled Project", ["Other"])).toBe("Untitled Project");
  });

  it("numbers the name from 2 when it's taken", () => {
    expect(firstFreeName("Untitled Project", ["Untitled Project"])).toBe("Untitled Project 2");
    expect(firstFreeName("Untitled Project", ["Untitled Project", "Untitled Project 2", "Untitled Project 4"]))
      .toBe("Untitled Project 3");
  });
});
