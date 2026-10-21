import type { Token } from "$lib/types/types";
import type { BezierCurve, Coordinate } from "../types/interfaces";
import { layoutParseTree, type LaidOutToken } from "./parseTreeLayout";
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

const PARSE_TREE_COLOURS: Record<LaidOutToken["kind"], string> = {
  kleene: "#008000",
  or: "#0096FF",
  concatenation: "#F08000",
  literal: "#FFFF00",
};

// Draws the parse tree centred at the top of the canvas, shrunk to fit inside the margin when it is too big
// for it. A small preview wants a small margin
export function drawParseTree(
  parse_tree: Token,
  context: CanvasRenderingContext2D,
  width: number,
  height: number,
  { margin = 60 } = {},
) {
  const { root, width: tree_width, height: tree_height } = layoutParseTree(parse_tree);
  const scale = Math.min(
    1,
    (width - 2 * margin) / Math.max(tree_width, 1),
    (height - 2 * margin) / Math.max(tree_height, 1),
  );

  context.save();
  context.translate(width / 2 - (tree_width * scale) / 2, margin);
  context.scale(scale, scale);
  context.font = "40px Arial";
  context.textBaseline = "middle";
  context.textAlign = "center";
  drawLaidOutToken(context, root);
  context.restore();
}

function drawLaidOutToken(context: CanvasRenderingContext2D, token: LaidOutToken) {
  const circle_radius = 35;

  context.lineWidth = 7;
  context.strokeStyle = PARSE_TREE_COLOURS[token.kind];
  context.beginPath();
  context.arc(token.x, token.y, circle_radius, 0, 2 * Math.PI);
  context.stroke();
  context.closePath();
  context.fillStyle = "black";
  context.fillText(token.label, token.x, token.y);

  for (const child of token.children) {
    drawLaidOutToken(context, child);
    const from = { x: token.x, y: token.y + circle_radius };
    const to = { x: child.x, y: child.y - circle_radius - 6 };
    drawConnection(context, { start_point: from, control_point_one: from, control_point_two: to, end_point: to }, "", false, 1);
  }
}
