import type { Connection, State } from "./interfaces";

export type TauriGeneratedAutomataInformation = [
  number | null, // Corresponds to start state index
  Array<State>, // Corresponds to list of states
  Array<Connection>, // Corresponds to list of connections
  { [key: string]: State }, // Corresponds to state positions
  string, // Corresponds to type of automata enum
  string  // Corresponds to date of last update (not necessary to set anything with this)
];