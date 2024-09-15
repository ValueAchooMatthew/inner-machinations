export enum Action {
  ADDING_REGULAR_STATE,
  ADDING_FINAL_STATE,
  ADDING_START_STATE,
  PLACING_START_OF_LINE,
  PLACING_START_OF_EPSILON_LINE,
  PLACING_END_OF_LINE,
  DRAGGING_LINE,
  CLICKING
}

export enum Automata {
  DFA,
  NFA
}

export enum WorkspaceType {
  RegularAutomata,
  Regex
}