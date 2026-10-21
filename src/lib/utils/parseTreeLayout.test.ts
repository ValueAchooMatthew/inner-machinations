import { describe, expect, it } from "vitest";
import type { Token } from "$lib/types/types";
import { layoutParseTree, NODE_SPACING, type LaidOutToken } from "./parseTreeLayout";

const literal = (character: string): Token => ({ Literal: character });
const or = (left: Token, right: Token): Token =>
  ({ OrOperator: { operator_character: "+", operator_name: "Or", left_argument: left, right_argument: right } });
const concatenation = (left: Token, right: Token): Token =>
  ({ ConcatenatedExpression: { operator_character: "⋅", operator_name: "Concatenated Expressions", left_argument: left, right_argument: right } });
const star = (inner: Token): Token =>
  ({ KleeneOperator: { operator_character: "*", operator_name: "Kleene", inner_argument: inner } });

const nodes = (node: LaidOutToken): Array<LaidOutToken> => [node, ...node.children.flatMap(nodes)];

describe("layoutParseTree", () => {
  it("puts children below their parent, with unary operators straight above their argument", () => {
    const { root } = layoutParseTree(concatenation(literal("a"), star(literal("b"))));
    const [left, right] = root.children;
    expect(root.label).toBe("⋅");
    expect(left.label).toBe("a");
    expect(right.label).toBe("*");
    expect(left.y).toBe(right.y);
    expect(left.y).toBeGreaterThan(root.y);
    expect(right.x).toBe(right.children[0].x);
    expect(root.x).toBe((left.x + right.x) / 2);
  });

  it("never draws two nodes at the same depth closer than the spacing, however deep the tree", () => {
    // ((a+b)(c+d))*((e+f)(g+h)), the kind of tree that used to overlap past depth 2
    const pair = (first: string, second: string) => or(literal(first), literal(second));
    const tree = concatenation(
      star(concatenation(pair("a", "b"), pair("c", "d"))),
      concatenation(pair("e", "f"), pair("g", "h")),
    );
    const laidOut = nodes(layoutParseTree(tree).root);
    for (const first of laidOut) {
      for (const second of laidOut) {
        if (first !== second && first.y === second.y) {
          expect(Math.abs(first.x - second.x)).toBeGreaterThanOrEqual(NODE_SPACING);
        }
      }
    }
  });

  it("reports the size of the drawing, starting at the origin", () => {
    const { root, width, height } = layoutParseTree(or(literal("a"), literal("b")));
    const laidOut = nodes(root);
    expect(Math.min(...laidOut.map((node) => node.x))).toBe(0);
    expect(Math.max(...laidOut.map((node) => node.x))).toBe(width);
    expect(Math.min(...laidOut.map((node) => node.y))).toBe(0);
    expect(Math.max(...laidOut.map((node) => node.y))).toBe(height);
  });
});
