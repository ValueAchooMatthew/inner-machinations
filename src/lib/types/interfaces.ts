import type { Token } from "./types";

// A state as the backend sends and receives it: keyed by its position, and listing the positions of the
// states it connects to, by character
export interface State {
  position: Coordinate;
  states_connected_to: Record<string, Array<string>>;
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
