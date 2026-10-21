import type { ConcatenatedExpression, KleeneOperator, Literal, OrOperator } from "./interfaces";

// A node of a regular expression's parse tree, as the backend sends it: an object with one key, naming the
// kind of node. Grouped expressions are parsed out in the backend before the tree is sent
export type Token =
  | Literal
  | { OrOperator: OrOperator }
  | { KleeneOperator: KleeneOperator }
  | { ConcatenatedExpression: ConcatenatedExpression };
