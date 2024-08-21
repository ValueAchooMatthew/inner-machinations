import type { ConcatenatedExpression, Connection, KleeneOperator, Literal, OrOperator, State } from "./interfaces";

// Not including grouped expressions as they are parsed out in rust backend before being passed to ts
export type Token = OrOperator | KleeneOperator | ConcatenatedExpression | Literal;