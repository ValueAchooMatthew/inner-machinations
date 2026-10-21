// Places a regular expression's parse tree for drawing. Each leaf gets a column of its own and each operator
// sits centred over its arguments, so a subtree gets all the room its leaves need and no two nodes overlap,
// however deep the tree goes.

import type { Token } from "$lib/types/types";

// Leaves are this far apart, and each level this far below the one above
export const NODE_SPACING = 100;
export const LEVEL_HEIGHT = 140;

export interface LaidOutToken {
  label: string;
  kind: "literal" | "or" | "kleene" | "concatenation";
  x: number;
  y: number;
  children: Array<LaidOutToken>;
}

// The tree's nodes, with the leftmost at x = 0 and the root at y = 0, and the size of the whole
export function layoutParseTree(tree: Token): { root: LaidOutToken; width: number; height: number } {
  let next_column = 0;
  let depth_reached = 0;

  const place = (token: Token, depth: number): LaidOutToken => {
    depth_reached = Math.max(depth_reached, depth);
    const y = depth * LEVEL_HEIGHT;

    if ("KleeneOperator" in token) {
      const kleene_operator = token.KleeneOperator;
      const inner = place(kleene_operator.inner_argument, depth + 1);
      return { label: kleene_operator.operator_character, kind: "kleene", x: inner.x, y, children: [inner] };
    }
    if ("OrOperator" in token || "ConcatenatedExpression" in token) {
      const operator = "OrOperator" in token ? token.OrOperator : token.ConcatenatedExpression;
      const left = place(operator.left_argument, depth + 1);
      const right = place(operator.right_argument, depth + 1);
      return {
        label: operator.operator_character,
        kind: "OrOperator" in token ? "or" : "concatenation",
        x: (left.x + right.x) / 2,
        y,
        children: [left, right],
      };
    }
    const x = next_column * NODE_SPACING;
    next_column++;
    return { label: token.Literal, kind: "literal", x, y, children: [] };
  };

  const root = place(tree, 0);
  return { root, width: (next_column - 1) * NODE_SPACING, height: depth_reached * LEVEL_HEIGHT };
}
