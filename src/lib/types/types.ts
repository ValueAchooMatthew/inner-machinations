import type { ConcatenatedExpression, Connection, KleeneOperator, Literal, OrOperator, State } from "./interfaces";

// Should switch to using json responses from backend rather than tuples to allow for interface representation in future
// very stupid, tech debt
export type TauriGeneratedAutomataInformation = [
  number | null, // Corresponds to start state index
  Array<State>, // Corresponds to list of states
  Array<Connection>, // Corresponds to list of connections
  { [key: string]: State }, // Corresponds to state positions
  string, // Corresponds to type of automata enum
  string, // Corresponds to date of last update (not necessary to set anything with this)
  Array<string>, // Corresponds to input alphabet
];

// Not including grouped expressions as they are parsed out in rust backend before being passed to ts
export type Token = OrOperator | KleeneOperator | ConcatenatedExpression | Literal;