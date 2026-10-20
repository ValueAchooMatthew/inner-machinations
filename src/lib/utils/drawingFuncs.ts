import type { Token } from "$lib/types/types";
import type { BezierCurve, Coordinate, KleeneOperator, OrOperator, ConcatenatedExpression } from "../types/interfaces";
import type { Selection } from "./automataStores";
import { connectionCurve, STATE_RADIUS, type Automaton, type AutomatonState, type Point } from "$lib/workbench/automaton";
import {
  getBezierCurveAngleAtPoint,
  getPointOnBezierCurveAtDistance,
} from "./mathFuncs";

const SELECTED_COLOUR = "#00008B";

export interface DrawingOptions {
  // Drawings are this many times smaller than the automaton, as on the dashboard's previews
  scale?: number;
  selection?: Selection | null;
  // The state a string's traversal has reached
  highlightedStateId?: number | null;
  // A connection being drawn, from the state it starts at to the pointer
  connectionPreview?: { from: Point; to: Point } | null;
}

export const drawAutomaton = (
  context: CanvasRenderingContext2D,
  width: number,
  height: number,
  automaton: Automaton,
  options: DrawingOptions = {},
) => {
  const scale = options.scale ?? 1;
  const selection = options.selection ?? null;
  // Needed so position specified for where characters are drawn is not drawn differently depending on if its offset is above/below or
  // left/right of the Connection
  context.textBaseline = "middle";
  context.textAlign = "center";
  context.strokeStyle = "black";
  context.clearRect(0, 0, width, height);

  automaton.states.forEach((state) => {
    drawNode(
      context,
      state,
      state.id === options.highlightedStateId,
      selection?.kind === "state" && selection.id === state.id,
      scale,
    );
  });
  automaton.connections.forEach((connection) => {
    const isSelected = selection?.kind === "connection" && selection.id === connection.id;
    const curve = connectionCurve(automaton, connection);
    drawConnection(context, curve, connection.character, isSelected, scale);
    if (isSelected) {
      drawControlPoints(context, curve, scale);
    }
  });
  if (options.connectionPreview) {
    const { from, to } = options.connectionPreview;
    drawConnection(context, { start_point: from, control_point_one: from, control_point_two: to, end_point: to }, "", false, scale);
  }
};

const drawConnection = (
  context: CanvasRenderingContext2D,
  curve: BezierCurve,
  character: string,
  isSelected: boolean,
  scale: number,
): void => {
  context.lineCap = "round";

  const start_coord: Coordinate = curve.start_point;
  const end_coord: Coordinate = curve.end_point;
  const headSize = 30 / scale;
  context.lineWidth = 5 / scale;
  context.strokeStyle = isSelected ? SELECTED_COLOUR : "black";

  context.beginPath();
  context.moveTo(start_coord.x / scale, start_coord.y / scale);

  context.bezierCurveTo(
    curve.control_point_one.x / scale,
    curve.control_point_one.y / scale,
    curve.control_point_two.x / scale,
    curve.control_point_two.y / scale,
    end_coord.x / scale,
    end_coord.y / scale,
  );

  context.stroke();
  const angle_of_curve_at_end = getBezierCurveAngleAtPoint(curve, 0.99);
  context.moveTo(end_coord.x / scale, end_coord.y / scale);

  context.lineTo(
    end_coord.x / scale -
      headSize * Math.cos(angle_of_curve_at_end - Math.PI / 6),
    end_coord.y / scale -
      headSize * Math.sin(angle_of_curve_at_end - Math.PI / 6),
  );

  context.moveTo(end_coord.x / scale, end_coord.y / scale);

  context.lineTo(
    end_coord.x / scale -
      headSize * Math.cos(angle_of_curve_at_end + Math.PI / 6),
    end_coord.y / scale -
      headSize * Math.sin(angle_of_curve_at_end + Math.PI / 6),
  );

  context.stroke();
  context.font = "40px Arial";
  if (scale > 1) {
    context.font = "20px Arial";
  }
  context.fillStyle = isSelected ? SELECTED_COLOUR : "black";

  const halfway_point = getPointOnBezierCurveAtDistance(curve, 0.5);

  context.fillText(
    character,
    halfway_point.x / scale + (50 / scale) * Math.sin(angle_of_curve_at_end),
    halfway_point.y / scale - (50 / scale) * Math.cos(angle_of_curve_at_end),
  );
};

// Handles on a selected connection's control points, which can be dragged to reshape it
const drawControlPoints = (context: CanvasRenderingContext2D, curve: BezierCurve, scale: number): void => {
  context.lineWidth = 2 / scale;
  context.strokeStyle = SELECTED_COLOUR;
  for (const controlPoint of [curve.control_point_one, curve.control_point_two]) {
    context.setLineDash([6, 6]);
    context.beginPath();
    const end = controlPoint === curve.control_point_one ? curve.start_point : curve.end_point;
    context.moveTo(end.x / scale, end.y / scale);
    context.lineTo(controlPoint.x / scale, controlPoint.y / scale);
    context.stroke();
    context.setLineDash([]);
    context.fillStyle = "white";
    context.beginPath();
    context.arc(controlPoint.x / scale, controlPoint.y / scale, 8 / scale, 0, 2 * Math.PI);
    context.fill();
    context.stroke();
  }
};

const drawNode = (
  context: CanvasRenderingContext2D,
  node: AutomatonState,
  isHighlighted: boolean,
  isSelected: boolean,
  scale: number,
) => {
  context.lineWidth = 3;
  context.strokeStyle = "black";
  if (node.isFinal) {
    context.beginPath();
    context.arc(
      node.position.x / scale,
      node.position.y / scale,
      42 / scale,
      0,
      2 * Math.PI,
    );
    context.stroke();
    context.closePath();
  }

  if (isHighlighted) {
    context.fillStyle = "rgb(218,112,214)";
  } else if (node.isStart) {
    context.fillStyle = "rgb(22, 163, 74)";
  } else {
    context.fillStyle = "rgb(234, 88, 12)";
  }
  if (isSelected) {
    context.lineWidth = 6;
    context.strokeStyle = SELECTED_COLOUR;
  }
  context.beginPath();
  context.arc(
    node.position.x / scale,
    node.position.y / scale,
    STATE_RADIUS / scale,
    0,
    2 * Math.PI,
  );
  context.fill();
  context.stroke();
  context.closePath();
};

export function drawParseTree(parse_tree: Token, context: CanvasRenderingContext2D, start_position: Coordinate) {
  // Ugliest fucking code of all type because javascript is a piece of shit language and should kill itself
  context.lineWidth = 7;
  context.font = "40px Arial";
  context.textBaseline = "middle";
  context.textAlign = "center";
  drawToken(context, parse_tree, start_position);

}

function drawToken(context: CanvasRenderingContext2D, token: Token, position: Coordinate) {
  const distance = 175;
  const circle_radius = 35;
  const arrow_y_end = distance - circle_radius - 6;

  if ("KleeneOperator" in token) {
    // We know the current token in the parse tree is a literal
    const kleene_operator = token.KleeneOperator as KleeneOperator;
    context.beginPath();
    context.strokeStyle = "#008000";
    context
      .arc(position.x, position.y, circle_radius, 0, 2 * Math.PI);
    context.stroke();
    context.closePath();
    context.fillText(
      kleene_operator.operator_character,
      position.x,
      position.y
    )
    
    drawToken(context, kleene_operator.inner_argument, {x: position.x, y: position.y + distance});
    drawConnection(context, {
      start_point: {x: position.x, y: position.y + circle_radius},
      control_point_one: {x: position.x, y: position.y + circle_radius},
      control_point_two: {x: position.x, y: position.y + arrow_y_end},
      end_point: {x: position.x, y: position.y + arrow_y_end}
    }, "", false, 1);
  } else if ("OrOperator" in token) {
    const or_operator = token.OrOperator as OrOperator;
    context.beginPath();
    context.strokeStyle = "#0096FF";
    context
      .arc(position.x, position.y, circle_radius, 0, 2 * Math.PI);
    context.stroke();
    context.closePath();
    context.fillText(
      or_operator.operator_character,
      position.x,
      position.y
    )
    drawToken(context, or_operator.left_argument, {x: position.x - distance, y: position.y + distance});
    drawToken(context, or_operator.right_argument, {x: position.x + distance, y: position.y + distance});
    drawConnection(context, {
      start_point: {x: position.x, y: position.y + circle_radius},
      control_point_one: {x: position.x, y: position.y + circle_radius},
      control_point_two: {x: position.x - distance, y: position.y + arrow_y_end},
      end_point: {x: position.x - distance, y: position.y + arrow_y_end}
    }, "", false, 1);
    drawConnection(context, {
      start_point: {x: position.x, y: position.y + circle_radius},
      control_point_one: {x: position.x, y: position.y + circle_radius},
      control_point_two: {x: position.x + distance, y: position.y + arrow_y_end},
      end_point: {x: position.x + distance, y: position.y + arrow_y_end}
    }, "", false, 1);
  } else if ("ConcatenatedExpression" in token) {
    const concatenated_expression = token.ConcatenatedExpression as ConcatenatedExpression;
    context.beginPath();
    context.strokeStyle = "#F08000";
    context
      .arc(position.x, position.y, circle_radius, 0, 2 * Math.PI);
    context.stroke();
    context.closePath();
    context.fillText(
      concatenated_expression.operator_character,
      position.x,
      position.y
    )
    drawToken(context, concatenated_expression.left_argument, {x: position.x - distance, y: position.y + distance});
    drawToken(context, concatenated_expression.right_argument, {x: position.x + distance, y: position.y + distance});
    drawConnection(context, {
      start_point: {x: position.x, y: position.y + circle_radius},
      control_point_one: {x: position.x, y: position.y + circle_radius},
      control_point_two: {x: position.x - distance, y: position.y + arrow_y_end},
      end_point: {x: position.x - distance, y: position.y + arrow_y_end}
    }, "", false, 1);
    drawConnection(context, {
      start_point: {x: position.x, y: position.y + circle_radius},
      control_point_one: {x: position.x, y: position.y + circle_radius},
      control_point_two: {x: position.x + distance, y: position.y + arrow_y_end},
      end_point: {x: position.x + distance, y: position.y + arrow_y_end}
    }, "", false, 1);
  } else if ("Literal" in token) {
    context.beginPath();
    context.strokeStyle = "#FFFF00";
    context
      .arc(position.x, position.y, circle_radius, 0, 2 * Math.PI);
    context.stroke();
    context.closePath();
    context.fillText(
      token.Literal,
      position.x,
      position.y
    )
  }

}