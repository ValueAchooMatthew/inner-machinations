import type { Token } from "$lib/types/types";
import type { State, Connection, Coordinate, BezierCurve, Literal, KleeneOperator, OrOperator, ConcatenatedExpression } from "../types/interfaces";
import {
  getBezierCurveAngleAtPoint,
  getPointOnBezierCurveAtDistance,
} from "./mathFuncs";

export const draw = (
  context: CanvasRenderingContext2D,
  width: number,
  height: number,
  nodes: Array<State>,
  connections: Array<Connection>,
  selected_connection_index: number | null,
  highlighted_state: State | null,
  scale: number
) => {
  // Needed so position specified for where characters are drawn is not drawn differently depending on if its offset is above/below or
  // left/right of the Connection
  context.textBaseline = "middle";
  context.textAlign = "center";
  context.strokeStyle = "black";
  context.clearRect(0, 0, width, height);
  const tick = new Audio("/metronome-85688.mp3");

  nodes.forEach((node) => {
    drawNode(context, node, highlighted_state, scale, tick);
  });
  connections.forEach((connection, index) => {
    drawConnection(
      context,
      connection,
      index,
      selected_connection_index,
      scale
    );
  });
};

const drawConnection = (
  context: CanvasRenderingContext2D,
  connection: Connection,
  index: number,
  selected_connection_index: number | null,
  scale: number,
): void => {
  context.lineCap = "round";

  const start_coord: Coordinate = connection.curve.start_point;
  // +1 to make self loops easier to draw
  const end_coord: Coordinate = connection.curve.end_point;
  const curve: BezierCurve = connection.curve;
  const headSize = 30 / scale;
  context.lineWidth = 5 / scale;

  if (selected_connection_index === index) {
    context.strokeStyle = "#00008B";
  } else {
    context.strokeStyle = "black";
  }
  context.beginPath();
  context.moveTo(start_coord.x / scale, start_coord.y / scale);
  // context.lineTo(endX, endY);

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
  context.fillStyle = "black";

  const halfway_point = getPointOnBezierCurveAtDistance(curve, 0.5);

  context.fillText(
    connection.connection_character,
    halfway_point.x / scale + (50 / scale) * Math.sin(angle_of_curve_at_end),
    halfway_point.y / scale - (50 / scale) * Math.cos(angle_of_curve_at_end),
  );
};

const drawNode = (
  context: CanvasRenderingContext2D,
  node: State,
  highlighted_state: State | null,
  scale: number,
  tick: HTMLAudioElement
) => {
  context.lineWidth = 3;
  if (node.is_final) {
    context.beginPath();
    context.strokeStyle = "black";
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

  if(highlighted_state?.position && node.position.x === highlighted_state.position.x && node.position.y === highlighted_state.position.y) {
    context.fillStyle = "rgb(218,112,214)";
    context.beginPath();
    context.arc(
      node.position.x / scale,
      node.position.y / scale,
      35 / scale,
      0,
      2 * Math.PI,
    );
    context.fill();
    context.stroke();
    context.closePath();
    tick.play();

  }
  else if (node.is_start) {
    context.fillStyle = "rgb(22, 163, 74)";
    context.beginPath();
    context.arc(
      node.position.x / scale,
      node.position.y / scale,
      35 / scale,
      0,
      2 * Math.PI,
    );
    context.fill();
    context.stroke();
    context.closePath();
  } else {
    context.fillStyle = "rgb(234, 88, 12)";
    context.beginPath();
    context.arc(
      node.position.x / scale,
      node.position.y / scale,
      35 / scale,
      0,
      2 * Math.PI,
    );
    context.fill();
    context.stroke();
    context.closePath();
    context.fillStyle = "rgb(22, 163, 74)";
  }
};

export function drawParseTree(parse_tree: Token, context: CanvasRenderingContext2D, start_position: Coordinate) {
  context.lineWidth = 7;
  context.font = "40px Arial";
  context.textBaseline = "middle";
  context.textAlign = "center";
  console.log(get_parse_tree_size(parse_tree));

  drawToken(context, parse_tree, start_position, 250, 175, 1.5);

}

// Returns number of nodes in parse tree
function get_parse_tree_size(parse_tree: Token): number {

  if("KleeneOperator" in parse_tree) {
    let kleene_operator = parse_tree.KleeneOperator as KleeneOperator;
    return 1 + get_parse_tree_size(kleene_operator.inner_argument)

  } else if("OrOperator" in parse_tree) {
    let or_operator = parse_tree.OrOperator as OrOperator;
    return 1 + get_parse_tree_size(or_operator.left_argument) + get_parse_tree_size(or_operator.right_argument);

  } else if("ConcatenatedExpression" in parse_tree) {
    let concatenated_expression = parse_tree.ConcatenatedExpression as ConcatenatedExpression;
    return 1 + get_parse_tree_size(concatenated_expression.left_argument) + get_parse_tree_size(concatenated_expression.right_argument);

  }

  return 1;
}

function drawToken(
  context: CanvasRenderingContext2D, 
  token: Token, 
  position: Coordinate, 
  x_distance_of_child: number, 
  y_distance_of_child: number, 
  shrink_factor: number
) {
  const circle_radius = 35;
  const arrow_y_end = y_distance_of_child - circle_radius - 6;

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
    drawToken(context, 
      kleene_operator.inner_argument, 
      {x: position.x, y: position.y + y_distance_of_child}, 
      Math.floor(x_distance_of_child - 100), 
      y_distance_of_child,
      shrink_factor*1.25
    );
    drawConnection(context, {
      curve: {
        start_point: {x: position.x, y: position.y + circle_radius},
        control_point_one: {x: position.x, y: position.y + circle_radius},
        control_point_two: {x: position.x, y: position.y + arrow_y_end},
        end_point: {x: position.x, y: position.y + arrow_y_end}
      },
      connection_character: "",
      element: "Connection"
    }, 0, -1, 1);
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
    drawToken(context, 
      or_operator.left_argument, 
      {x: position.x - x_distance_of_child, y: position.y + y_distance_of_child}, 
      Math.floor(x_distance_of_child  - 100), 
      y_distance_of_child,
      shrink_factor*1.25
    );
    drawToken(context, 
      or_operator.right_argument, 
      {x: position.x + x_distance_of_child, y: position.y + y_distance_of_child}, 
      Math.floor(x_distance_of_child  - 100), 
      y_distance_of_child,
      shrink_factor*1.25
    );
    drawConnection(context, {
      curve: {
        start_point: {x: position.x, y: position.y + circle_radius},
        control_point_one: {x: position.x, y: position.y + circle_radius},
        control_point_two: {x: position.x - x_distance_of_child, y: position.y + arrow_y_end},
        end_point: {x: position.x - x_distance_of_child, y: position.y + arrow_y_end}
      },
      connection_character: "",
      element: "Connection"
    }, 0, -1, 1);
    drawConnection(context, {
      curve: {
        start_point: {x: position.x, y: position.y + circle_radius},
        control_point_one: {x: position.x, y: position.y + circle_radius},
        control_point_two: {x: position.x + x_distance_of_child, y: position.y + arrow_y_end},
        end_point: {x: position.x + x_distance_of_child, y: position.y + arrow_y_end}
      },
      connection_character: "",
      element: "Connection"
    }, 0, -1, 1);
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
    drawToken(context, 
      concatenated_expression.left_argument, 
      {x: position.x - x_distance_of_child, y: position.y + y_distance_of_child}, 
      Math.floor(x_distance_of_child - 100), 
      y_distance_of_child,
      shrink_factor*1.25
    );
    drawToken(context, 
      concatenated_expression.right_argument, 
      {x: position.x + x_distance_of_child, y: position.y + y_distance_of_child}, 
      Math.floor(x_distance_of_child - 100), 
      y_distance_of_child,
      shrink_factor*1.25
    );
    drawConnection(context, {
      curve: {
        start_point: {x: position.x, y: position.y + circle_radius},
        control_point_one: {x: position.x, y: position.y + circle_radius},
        control_point_two: {x: position.x - x_distance_of_child, y: position.y + arrow_y_end},
        end_point: {x: position.x - x_distance_of_child, y: position.y + arrow_y_end}
      },
      connection_character: "",
      element: "Connection"
    }, 0, -1, 1);
    drawConnection(context, {
      curve: {
        start_point: {x: position.x, y: position.y + circle_radius},
        control_point_one: {x: position.x, y: position.y + circle_radius},
        control_point_two: {x: position.x + x_distance_of_child, y: position.y + arrow_y_end},
        end_point: {x: position.x + x_distance_of_child, y: position.y + arrow_y_end}
      },
      connection_character: "",
      element: "Connection"
    }, 0, -1, 1);
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