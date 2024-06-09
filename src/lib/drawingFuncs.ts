import type { State, Connection, Coordinate, BezierCurve } from "./interfaces";
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
  scale: number,
) => {
  // Needed so position specified for where characters are drawn is not drawn differently depending on if its offset is above/below or
  // left/right of the Connection
  context.textBaseline = "middle";
  context.textAlign = "center";
  context.strokeStyle = "black";
  context.clearRect(0, 0, width, height);

  nodes.forEach((node) => {
    drawNode(context, node, scale);
  });
  connections.forEach((connection, index) => {
    drawConnection(
      context,
      connection,
      index,
      selected_connection_index,
      scale,
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
  scale: number,
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
  // For some reason fills with the wrong colour unless I do this, no idea why
  if (node.is_start) {
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
