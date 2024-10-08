// Reworked so every state has a reference to it's position, to allow for a single state interface for use

import type { Automata } from "./enums";
import type { Token } from "./types";

// In both drawing states to the canvas and connecting data in the rust
export interface State {
  // Specifying coordinates of node as an identification for a node
  position: Coordinate;
  // CANNOT use Sets as SERDE cannot deserialize javascript Set into rust HashSet
  states_connected_to: Map<String, Array<String>>;
  is_start: boolean;
  is_final: boolean;
  element: "State";
}

export interface Connection {
  curve: BezierCurve;
  connection_character: string;
  element: "Connection";
}

export interface BezierCurve {
  start_point: Coordinate;
  // Coordinates for control points for drawing bezier curves
  control_point_one: Coordinate;
  control_point_two: Coordinate;
  end_point: Coordinate;
}

export interface Coordinate {
  x: number;
  y: number;
}

export interface CheckedStringResponse {
  dialogue: string,
  // The reason we provide null as an option rather than using a boolean is because if the automata being tested is invalid,
  // We do not want to give a response back to the user indicating the string is either accepted OR rejected since it's not possible
  // to tell if we cannot test the automata in the first place.
  is_string_accepted: boolean | null,
  states_traversed: Array<State>
}

export interface OrOperator {
  operator_character: "+",
  operator_name: "Or",
  left_argument: Token,
  right_argument: Token
}

export interface KleeneOperator {
  operator_character: "*",
  operator_name: "Kleene",
  inner_argument: Token
}

export interface ConcatenatedExpression {
  operator_character: "⋅",
  operator_name: "Concatenated Expressions",
  left_argument: Token,
  right_argument: Token
}

export interface Literal {
  Literal: string
}

export interface WorkspaceData {
  start_state_index: number | null,
  start_state_position: string | null,
  state_positions: { [key: string]: State; },
  list_of_states: Array<State>,
  list_of_connections: Array<Connection>,
  type_of_automata: string,
  date_of_last_update: string,
  alphabet: Array<string>,
  should_strict_check: boolean,
  should_show_string_traversal: boolean,
  default_connection_character: string

}
