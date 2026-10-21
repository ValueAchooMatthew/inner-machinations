import type { Token } from "./types";
import type { Automaton } from "$lib/workbench/automaton";

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

// A saved regular expression, as the backend sends it
export interface RegexData {
  regex_name: string,
  regex: string,
  date_of_last_update: string
}

// Something saved that the dashboard lists, by its name
export interface SavedItem {
  kind: "workspace" | "regex",
  name: string
}

// A saved workspace, as the backend sends it
export interface WorkspaceData {
  automaton: Automaton,
  type_of_automata: string,
  date_of_last_update: string,
  alphabet: Array<string>,
  should_strict_check: boolean,
  should_show_string_traversal: boolean,
  default_connection_character: string
}
