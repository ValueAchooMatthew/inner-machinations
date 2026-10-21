import { describe, expect, it } from "vitest";
import { PARSE_TREE_NODE_REACH, placeParseTree } from "./drawingFuncs";

describe("placeParseTree", () => {
  // The edges of the circles at the tree's corners, on the canvas
  const edges = (tree_width: number, tree_height: number, width: number, height: number, margin: number) => {
    const { scale, left, top } = placeParseTree(tree_width, tree_height, width, height, margin);
    return {
      scale,
      left: left - PARSE_TREE_NODE_REACH * scale,
      top: top - PARSE_TREE_NODE_REACH * scale,
      right: left + (tree_width + PARSE_TREE_NODE_REACH) * scale,
      bottom: top + (tree_height + PARSE_TREE_NODE_REACH) * scale,
    };
  };

  it("keeps a small tree full size, centred at the top inside the margin", () => {
    expect(edges(200, 140, 1000, 600, 60)).toEqual({ scale: 1, left: 361.5, top: 60, right: 638.5, bottom: 277 });
  });

  it("shrinks a tree too big for a card's preview so every circle is inside the margin", () => {
    for (const [tree_width, tree_height] of [[700, 140], [100, 560], [700, 560]]) {
      const placed = edges(tree_width, tree_height, 232, 150, 12);
      expect(placed.scale).toBeLessThan(1);
      expect(placed.left).toBeGreaterThanOrEqual(12 - 1e-9);
      expect(placed.top).toBeCloseTo(12);
      expect(placed.right).toBeLessThanOrEqual(232 - 12 + 1e-9);
      expect(placed.bottom).toBeLessThanOrEqual(150 - 12 + 1e-9);
    }
  });
});
